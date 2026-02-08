package com.openclaw.callingnode.controller

import com.openclaw.callingnode.config.ConfigManager
import com.openclaw.callingnode.gateway.GatewayConnectionService
import com.openclaw.callingnode.model.*
import com.openclaw.callingnode.service.vapi.VapiCallingService
import com.openclaw.callingnode.service.whatsapp.WhatsAppCallingService
import kotlinx.coroutines.*
import kotlinx.coroutines.flow.*
import timber.log.Timber
import java.util.UUID
import java.util.concurrent.ConcurrentHashMap
import javax.inject.Inject
import javax.inject.Singleton

/**
 * The Agent Controller is the central orchestrator for the OpenClaw Calling Node.
 *
 * It serves as the "brain" of the application, responsible for:
 * 1. Receiving commands from the OpenClaw Gateway
 * 2. Routing calls to the appropriate provider (Vapi AI or WhatsApp)
 * 3. Managing call lifecycle across all providers
 * 4. Reporting call status back to the Gateway
 * 5. Handling error recovery and fallback logic
 *
 * The controller maintains a unified view of all active calls regardless
 * of the underlying provider, enabling the Gateway to interact with a
 * single, consistent API.
 *
 * Command flow:
 * ```
 * Gateway → WebSocket → GatewayConnectionService → AgentController
 *     ↓                                                    ↓
 *     ← ← ← ← ← ← ← ← ← ← ← ← ← ← ← ← ← ← ← ← ← ←
 *                                                    ↓
 *                                          ┌─────────┴─────────┐
 *                                          │                   │
 *                                    VapiService      WhatsAppService
 * ```
 */
@Singleton
class AgentController @Inject constructor(
    private val configManager: ConfigManager
) {
    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.IO)
    private val callHistory = ConcurrentHashMap<String, CallRecord>()

    private val _controllerState = MutableStateFlow(ControllerState.IDLE)
    val controllerState: StateFlow<ControllerState> = _controllerState.asStateFlow()

    private val _callUpdates = MutableSharedFlow<CallRecord>(extraBufferCapacity = 64)
    val callUpdates: SharedFlow<CallRecord> = _callUpdates.asSharedFlow()

    enum class ControllerState {
        IDLE, PROCESSING, ERROR
    }

    /**
     * Start the Agent Controller.
     *
     * This begins listening for commands from the Gateway and events
     * from the calling services.
     */
    fun start() {
        Timber.i("Agent Controller starting")

        // Listen for Gateway commands
        scope.launch {
            GatewayConnectionService.instance?.incomingCommands?.collect { command ->
                handleGatewayCommand(command)
            }
        }

        // Listen for Vapi call events
        scope.launch {
            VapiCallingService.instance?.callEvents?.collect { event ->
                handleVapiEvent(event)
            }
        }

        // Listen for WhatsApp call events
        scope.launch {
            WhatsAppCallingService.instance?.callEvents?.collect { event ->
                handleWhatsAppEvent(event)
            }
        }

        Timber.i("Agent Controller started")
    }

    /**
     * Stop the Agent Controller and clean up resources.
     */
    fun stop() {
        scope.cancel()
        Timber.i("Agent Controller stopped")
    }

    // ─── Gateway Command Handling ───────────────────────────────────────────

    private suspend fun handleGatewayCommand(command: GatewayCommand) {
        Timber.i("Processing gateway command: ${command.type} (id: ${command.id})")
        _controllerState.value = ControllerState.PROCESSING

        try {
            when (command.type) {
                CommandType.MAKE_CALL -> handleMakeCall(command)
                CommandType.END_CALL -> handleEndCall(command)
                CommandType.GET_CALL_STATUS -> handleGetCallStatus(command)
                CommandType.LIST_ACTIVE_CALLS -> handleListActiveCalls(command)
                CommandType.SEND_DTMF -> handleSendDtmf(command)
                CommandType.TRANSFER_CALL -> handleTransferCall(command)
                CommandType.CONFIGURE -> handleConfigure(command)
                CommandType.PING -> handlePing(command)
            }
        } catch (e: Exception) {
            Timber.e(e, "Error handling gateway command: ${command.type}")
            sendErrorResponse(command.id, "Internal error: ${e.message}")
        } finally {
            _controllerState.value = ControllerState.IDLE
        }
    }

    /**
     * Handle a MAKE_CALL command from the Gateway.
     *
     * This determines the appropriate provider and initiates the call.
     * If no provider is specified, the default provider from config is used.
     */
    private suspend fun handleMakeCall(command: GatewayCommand) {
        val phoneNumber = command.payload.phoneNumber
        if (phoneNumber.isNullOrEmpty()) {
            sendErrorResponse(command.id, "Phone number is required")
            return
        }

        val config = configManager.configuration.first()
        val provider = command.payload.provider ?: config.defaultProvider

        Timber.i("Making call via $provider to $phoneNumber")

        val callRecord = when (provider) {
            CallProvider.VAPI -> makeVapiCall(
                phoneNumber = phoneNumber,
                assistantId = command.payload.assistantId,
                systemPrompt = command.payload.systemPrompt,
                firstMessage = command.payload.firstMessage
            )

            CallProvider.WHATSAPP -> makeWhatsAppCall(phoneNumber)

            CallProvider.NATIVE -> {
                sendErrorResponse(command.id, "Native calling not yet implemented")
                return
            }
        }

        if (callRecord != null) {
            callHistory[callRecord.id] = callRecord
            _callUpdates.emit(callRecord)

            sendResponse(
                commandId = command.id,
                type = ResponseType.CALL_INITIATED,
                payload = ResponsePayload(
                    callId = callRecord.id,
                    callState = callRecord.state,
                    callRecord = callRecord,
                    message = "Call initiated via $provider to $phoneNumber"
                )
            )
        } else {
            sendErrorResponse(command.id, "Failed to initiate call via $provider to $phoneNumber")
        }
    }

    private suspend fun handleEndCall(command: GatewayCommand) {
        val callId = command.payload.callId
        if (callId.isNullOrEmpty()) {
            sendErrorResponse(command.id, "Call ID is required")
            return
        }

        val record = callHistory[callId]
        if (record == null) {
            sendErrorResponse(command.id, "Call not found: $callId")
            return
        }

        val success = when (record.provider) {
            CallProvider.VAPI -> VapiCallingService.instance?.endCall(callId) ?: false
            CallProvider.WHATSAPP -> WhatsAppCallingService.instance?.endCall(callId) ?: false
            CallProvider.NATIVE -> false
        }

        if (success) {
            sendResponse(
                commandId = command.id,
                type = ResponseType.CALL_ENDED,
                payload = ResponsePayload(
                    callId = callId,
                    callState = CallState.ENDED,
                    message = "Call ended: $callId"
                )
            )
        } else {
            sendErrorResponse(command.id, "Failed to end call: $callId")
        }
    }

    private suspend fun handleGetCallStatus(command: GatewayCommand) {
        val callId = command.payload.callId
        if (callId.isNullOrEmpty()) {
            sendErrorResponse(command.id, "Call ID is required")
            return
        }

        val record = callHistory[callId]
        if (record == null) {
            sendErrorResponse(command.id, "Call not found: $callId")
            return
        }

        sendResponse(
            commandId = command.id,
            type = ResponseType.CALL_STATUS,
            payload = ResponsePayload(
                callId = callId,
                callState = record.state,
                callRecord = record
            )
        )
    }

    private suspend fun handleListActiveCalls(command: GatewayCommand) {
        val vapiCalls = VapiCallingService.instance?.getActiveCalls() ?: emptyList()
        val whatsappCalls = WhatsAppCallingService.instance?.getActiveCalls() ?: emptyList()
        val allActive = vapiCalls + whatsappCalls

        sendResponse(
            commandId = command.id,
            type = ResponseType.CALL_LIST,
            payload = ResponsePayload(
                activeCalls = allActive,
                message = "${allActive.size} active call(s)"
            )
        )
    }

    private suspend fun handleSendDtmf(command: GatewayCommand) {
        // DTMF is primarily for Vapi/native calls
        sendErrorResponse(command.id, "DTMF not supported for current call type")
    }

    private suspend fun handleTransferCall(command: GatewayCommand) {
        sendErrorResponse(command.id, "Call transfer not yet implemented")
    }

    private suspend fun handleConfigure(command: GatewayCommand) {
        val config = command.payload.config ?: run {
            sendErrorResponse(command.id, "Configuration payload is required")
            return
        }

        config["vapi_api_key"]?.let { configManager.updateVapiApiKey(it) }
        config["vapi_assistant_id"]?.let { configManager.updateVapiDefaultAssistantId(it) }
        config["whatsapp_access_token"]?.let { configManager.updateWhatsAppAccessToken(it) }
        config["whatsapp_phone_number_id"]?.let { configManager.updateWhatsAppPhoneNumberId(it) }
        config["default_provider"]?.let {
            try {
                configManager.updateDefaultProvider(CallProvider.valueOf(it.uppercase()))
            } catch (_: Exception) {}
        }

        sendResponse(
            commandId = command.id,
            type = ResponseType.CONFIG_UPDATED,
            payload = ResponsePayload(message = "Configuration updated")
        )
    }

    private suspend fun handlePing(command: GatewayCommand) {
        sendResponse(
            commandId = command.id,
            type = ResponseType.PONG,
            payload = ResponsePayload(message = "pong")
        )
    }

    // ─── Call Initiation ────────────────────────────────────────────────────

    /**
     * Make a call via Vapi AI.
     *
     * This is the primary method for traditional phone calls with an
     * AI voice agent. Vapi handles the telephony infrastructure and
     * AI conversation.
     */
    suspend fun makeVapiCall(
        phoneNumber: String,
        assistantId: String? = null,
        systemPrompt: String? = null,
        firstMessage: String? = null
    ): CallRecord? {
        val service = VapiCallingService.instance ?: run {
            Timber.e("VapiCallingService not running")
            return null
        }

        return service.makeCall(
            phoneNumber = phoneNumber,
            assistantId = assistantId,
            systemPrompt = systemPrompt,
            firstMessage = firstMessage
        )
    }

    /**
     * Make a call via WhatsApp Business Calling API.
     *
     * This initiates a VoIP call through WhatsApp using WebRTC.
     * The user must have granted call permission beforehand.
     */
    suspend fun makeWhatsAppCall(phoneNumber: String): CallRecord? {
        val service = WhatsAppCallingService.instance ?: run {
            Timber.e("WhatsAppCallingService not running")
            return null
        }

        return service.makeCall(phoneNumber)
    }

    /**
     * Request WhatsApp call permission from a user.
     */
    suspend fun requestWhatsAppCallPermission(phoneNumber: String): Boolean {
        val service = WhatsAppCallingService.instance ?: return false
        return service.requestCallPermission(phoneNumber)
    }

    // ─── Event Handling ─────────────────────────────────────────────────────

    private suspend fun handleVapiEvent(event: VapiCallingService.CallEvent) {
        Timber.d("Vapi event: ${event.type} for call ${event.callId}")

        event.callRecord?.let { record ->
            callHistory[event.callId] = record
            _callUpdates.emit(record)
        }

        val responseType = when (event.type) {
            VapiCallingService.CallEventType.CALL_ENDED -> ResponseType.CALL_ENDED
            VapiCallingService.CallEventType.CALL_FAILED -> ResponseType.CALL_FAILED
            else -> ResponseType.CALL_STATUS
        }

        // Notify the Gateway of the event
        GatewayConnectionService.instance?.sendResponse(
            GatewayResponse(
                id = UUID.randomUUID().toString(),
                commandId = "event",
                type = responseType,
                payload = ResponsePayload(
                    callId = event.callId,
                    callState = event.callRecord?.state,
                    callRecord = event.callRecord,
                    error = event.error
                )
            )
        )
    }

    private suspend fun handleWhatsAppEvent(event: WhatsAppCallingService.WhatsAppCallEvent) {
        Timber.d("WhatsApp event: ${event.type} for call ${event.callId}")

        event.callRecord?.let { record ->
            callHistory[event.callId] = record
            _callUpdates.emit(record)
        }

        val responseType = when (event.type) {
            WhatsAppCallingService.WhatsAppCallEventType.CALL_ENDED -> ResponseType.CALL_ENDED
            WhatsAppCallingService.WhatsAppCallEventType.CALL_FAILED -> ResponseType.CALL_FAILED
            else -> ResponseType.CALL_STATUS
        }

        GatewayConnectionService.instance?.sendResponse(
            GatewayResponse(
                id = UUID.randomUUID().toString(),
                commandId = "event",
                type = responseType,
                payload = ResponsePayload(
                    callId = event.callId,
                    callState = event.callRecord?.state,
                    callRecord = event.callRecord,
                    error = event.error
                )
            )
        )
    }

    // ─── Response Helpers ───────────────────────────────────────────────────

    private fun sendResponse(commandId: String, type: ResponseType, payload: ResponsePayload) {
        GatewayConnectionService.instance?.sendResponse(
            GatewayResponse(
                id = UUID.randomUUID().toString(),
                commandId = commandId,
                type = type,
                payload = payload
            )
        )
    }

    private fun sendErrorResponse(commandId: String, error: String) {
        Timber.e("Command error ($commandId): $error")
        sendResponse(
            commandId = commandId,
            type = ResponseType.ERROR,
            payload = ResponsePayload(error = error)
        )
    }

    // ─── Query Methods ──────────────────────────────────────────────────────

    fun getCallHistory(): List<CallRecord> = callHistory.values.toList()

    fun getActiveCallCount(): Int {
        return callHistory.values.count {
            it.state != CallState.ENDED && it.state != CallState.FAILED && it.state != CallState.IDLE
        }
    }
}
