package com.openclaw.callingnode.viewmodel

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.openclaw.callingnode.config.ConfigManager
import com.openclaw.callingnode.controller.AgentController
import com.openclaw.callingnode.gateway.GatewayConnectionService
import com.openclaw.callingnode.model.*
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.*
import kotlinx.coroutines.launch
import timber.log.Timber
import javax.inject.Inject

/**
 * Main ViewModel for the OpenClaw Calling Node UI.
 *
 * Provides reactive state for the UI to observe, including:
 * - Gateway connection status
 * - Active calls list
 * - Call history
 * - Configuration state
 */
@HiltViewModel
class MainViewModel @Inject constructor(
    private val agentController: AgentController,
    private val configManager: ConfigManager
) : ViewModel() {

    // ─── UI State ───────────────────────────────────────────────────────────

    data class UiState(
        val gatewayConnected: Boolean = false,
        val gatewayState: GatewayConnectionService.ConnectionState = GatewayConnectionService.ConnectionState.DISCONNECTED,
        val activeCalls: List<CallRecord> = emptyList(),
        val callHistory: List<CallRecord> = emptyList(),
        val isCallInProgress: Boolean = false,
        val lastError: String? = null,
        val config: NodeConfiguration = NodeConfiguration()
    )

    private val _uiState = MutableStateFlow(UiState())
    val uiState: StateFlow<UiState> = _uiState.asStateFlow()

    // ─── Dial Pad State ─────────────────────────────────────────────────────

    data class DialState(
        val phoneNumber: String = "",
        val selectedProvider: CallProvider = CallProvider.VAPI,
        val systemPrompt: String = "",
        val firstMessage: String = "",
        val isDialing: Boolean = false
    )

    private val _dialState = MutableStateFlow(DialState())
    val dialState: StateFlow<DialState> = _dialState.asStateFlow()

    init {
        // Observe configuration changes
        viewModelScope.launch {
            configManager.configuration.collect { config ->
                _uiState.update { it.copy(config = config) }
                _dialState.update { it.copy(selectedProvider = config.defaultProvider) }
            }
        }

        // Observe call updates from the Agent Controller
        viewModelScope.launch {
            agentController.callUpdates.collect { callRecord ->
                updateCallState(callRecord)
            }
        }

        // Observe Gateway connection state
        viewModelScope.launch {
            GatewayConnectionService.instance?.connectionState?.collect { state ->
                _uiState.update {
                    it.copy(
                        gatewayState = state,
                        gatewayConnected = state == GatewayConnectionService.ConnectionState.CONNECTED
                    )
                }
            }
        }

        // Start the Agent Controller
        agentController.start()
    }

    // ─── Call Actions ───────────────────────────────────────────────────────

    /**
     * Initiate a call using the current dial state.
     */
    fun makeCall() {
        val dial = _dialState.value
        if (dial.phoneNumber.isBlank()) {
            _uiState.update { it.copy(lastError = "Please enter a phone number") }
            return
        }

        _dialState.update { it.copy(isDialing = true) }

        viewModelScope.launch {
            try {
                val record = when (dial.selectedProvider) {
                    CallProvider.VAPI -> agentController.makeVapiCall(
                        phoneNumber = dial.phoneNumber,
                        systemPrompt = dial.systemPrompt.ifBlank { null },
                        firstMessage = dial.firstMessage.ifBlank { null }
                    )
                    CallProvider.WHATSAPP -> agentController.makeWhatsAppCall(dial.phoneNumber)
                    CallProvider.NATIVE -> {
                        _uiState.update { it.copy(lastError = "Native calling not yet supported") }
                        null
                    }
                }

                if (record != null) {
                    _uiState.update { it.copy(lastError = null) }
                    Timber.i("Call initiated: ${record.id}")
                } else {
                    _uiState.update { it.copy(lastError = "Failed to initiate call") }
                }
            } catch (e: Exception) {
                Timber.e(e, "Error making call")
                _uiState.update { it.copy(lastError = "Error: ${e.message}") }
            } finally {
                _dialState.update { it.copy(isDialing = false) }
            }
        }
    }

    /**
     * End a specific call.
     */
    fun endCall(callId: String) {
        viewModelScope.launch {
            val record = _uiState.value.activeCalls.find { it.id == callId }
            if (record != null) {
                when (record.provider) {
                    CallProvider.VAPI -> {
                        com.openclaw.callingnode.service.vapi.VapiCallingService.instance?.endCall(callId)
                    }
                    CallProvider.WHATSAPP -> {
                        com.openclaw.callingnode.service.whatsapp.WhatsAppCallingService.instance?.endCall(callId)
                    }
                    CallProvider.NATIVE -> {}
                }
            }
        }
    }

    /**
     * Request WhatsApp call permission for a phone number.
     */
    fun requestWhatsAppPermission() {
        val phoneNumber = _dialState.value.phoneNumber
        if (phoneNumber.isBlank()) return

        viewModelScope.launch {
            val success = agentController.requestWhatsAppCallPermission(phoneNumber)
            if (success) {
                _uiState.update { it.copy(lastError = null) }
            } else {
                _uiState.update { it.copy(lastError = "Failed to request WhatsApp call permission") }
            }
        }
    }

    // ─── Dial Pad Updates ───────────────────────────────────────────────────

    fun updatePhoneNumber(number: String) {
        _dialState.update { it.copy(phoneNumber = number) }
    }

    fun updateSelectedProvider(provider: CallProvider) {
        _dialState.update { it.copy(selectedProvider = provider) }
    }

    fun updateSystemPrompt(prompt: String) {
        _dialState.update { it.copy(systemPrompt = prompt) }
    }

    fun updateFirstMessage(message: String) {
        _dialState.update { it.copy(firstMessage = message) }
    }

    fun clearError() {
        _uiState.update { it.copy(lastError = null) }
    }

    // ─── Configuration Updates ──────────────────────────────────────────────

    fun updateConfig(key: String, value: String) {
        viewModelScope.launch {
            when (key) {
                "gateway_url" -> configManager.updateGatewayUrl(value)
                "gateway_token" -> configManager.updateGatewayToken(value)
                "vapi_api_key" -> configManager.updateVapiApiKey(value)
                "vapi_assistant_id" -> configManager.updateVapiDefaultAssistantId(value)
                "vapi_phone_number_id" -> configManager.updateVapiPhoneNumberId(value)
                "whatsapp_access_token" -> configManager.updateWhatsAppAccessToken(value)
                "whatsapp_phone_number_id" -> configManager.updateWhatsAppPhoneNumberId(value)
                "whatsapp_business_account_id" -> configManager.updateWhatsAppBusinessAccountId(value)
            }
        }
    }

    // ─── Internal ───────────────────────────────────────────────────────────

    private fun updateCallState(callRecord: CallRecord) {
        _uiState.update { state ->
            val activeCalls = state.activeCalls.toMutableList()
            val historyIndex = activeCalls.indexOfFirst { it.id == callRecord.id }

            if (callRecord.state == CallState.ENDED || callRecord.state == CallState.FAILED) {
                if (historyIndex >= 0) activeCalls.removeAt(historyIndex)
            } else {
                if (historyIndex >= 0) {
                    activeCalls[historyIndex] = callRecord
                } else {
                    activeCalls.add(callRecord)
                }
            }

            val history = (state.callHistory + callRecord)
                .distinctBy { it.id }
                .sortedByDescending { it.startedAt }
                .take(50)

            state.copy(
                activeCalls = activeCalls,
                callHistory = history,
                isCallInProgress = activeCalls.isNotEmpty()
            )
        }
    }

    override fun onCleared() {
        super.onCleared()
        agentController.stop()
    }
}
