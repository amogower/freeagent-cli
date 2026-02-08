package com.openclaw.callingnode.service.vapi

import android.app.Notification
import android.app.PendingIntent
import android.app.Service
import android.content.Intent
import android.os.IBinder
import androidx.core.app.NotificationCompat
import com.openclaw.callingnode.OpenClawApplication
import com.openclaw.callingnode.R
import com.openclaw.callingnode.config.ConfigManager
import com.openclaw.callingnode.model.*
import dagger.hilt.android.AndroidEntryPoint
import kotlinx.coroutines.*
import kotlinx.coroutines.flow.*
import kotlinx.serialization.json.Json
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.OkHttpClient
import okhttp3.logging.HttpLoggingInterceptor
import retrofit2.Retrofit
import retrofit2.converter.kotlinx.serialization.asConverterFactory
import timber.log.Timber
import java.util.UUID
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.TimeUnit
import javax.inject.Inject

/**
 * Service responsible for managing voice calls through the Vapi AI platform.
 *
 * Vapi handles the telephony infrastructure (Twilio, etc.) and AI agent
 * interaction. This service acts as the bridge between the OpenClaw
 * Agent Controller and the Vapi REST API.
 *
 * Key capabilities:
 * - Create outbound calls with AI assistants
 * - Monitor call status in real-time
 * - End calls programmatically
 * - Create and manage Vapi assistants on-the-fly
 */
@AndroidEntryPoint
class VapiCallingService : Service() {

    @Inject lateinit var configManager: ConfigManager

    private val serviceScope = CoroutineScope(SupervisorJob() + Dispatchers.IO)
    private val json = Json { ignoreUnknownKeys = true; encodeDefaults = true }

    private var vapiApi: VapiApi? = null
    private val activeCalls = ConcurrentHashMap<String, CallRecord>()
    private val pollingJobs = ConcurrentHashMap<String, Job>()

    private val _callEvents = MutableSharedFlow<CallEvent>(extraBufferCapacity = 64)
    val callEvents: SharedFlow<CallEvent> = _callEvents.asSharedFlow()

    data class CallEvent(
        val callId: String,
        val type: CallEventType,
        val callRecord: CallRecord? = null,
        val error: String? = null
    )

    enum class CallEventType {
        CALL_STARTED, CALL_RINGING, CALL_ACTIVE, CALL_ENDED, CALL_FAILED, STATUS_UPDATE
    }

    companion object {
        private const val NOTIFICATION_ID = 2001
        private const val POLL_INTERVAL_MS = 2000L

        @Volatile
        var instance: VapiCallingService? = null
            private set
    }

    override fun onCreate() {
        super.onCreate()
        instance = this

        serviceScope.launch {
            configManager.configuration.collect { config ->
                initializeApi(config)
            }
        }

        Timber.i("VapiCallingService created")
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        startForeground(NOTIFICATION_ID, createNotification("Vapi service ready"))
        return START_STICKY
    }

    override fun onBind(intent: Intent?): IBinder? = null

    override fun onDestroy() {
        instance = null
        pollingJobs.values.forEach { it.cancel() }
        serviceScope.cancel()
        super.onDestroy()
        Timber.i("VapiCallingService destroyed")
    }

    // ─── API Initialization ─────────────────────────────────────────────────

    private fun initializeApi(config: NodeConfiguration) {
        if (config.vapiApiKey.isEmpty()) {
            Timber.w("Vapi API key not configured")
            return
        }

        val loggingInterceptor = HttpLoggingInterceptor { message ->
            Timber.tag("VapiHttp").d(message)
        }.apply {
            level = HttpLoggingInterceptor.Level.BODY
        }

        val client = OkHttpClient.Builder()
            .addInterceptor { chain ->
                val request = chain.request().newBuilder()
                    .addHeader("Authorization", "Bearer ${config.vapiApiKey}")
                    .addHeader("Content-Type", "application/json")
                    .build()
                chain.proceed(request)
            }
            .addInterceptor(loggingInterceptor)
            .connectTimeout(30, TimeUnit.SECONDS)
            .readTimeout(30, TimeUnit.SECONDS)
            .writeTimeout(30, TimeUnit.SECONDS)
            .build()

        val contentType = "application/json".toMediaType()

        vapiApi = Retrofit.Builder()
            .baseUrl("https://api.vapi.ai/")
            .client(client)
            .addConverterFactory(json.asConverterFactory(contentType))
            .build()
            .create(VapiApi::class.java)

        Timber.i("Vapi API initialized")
    }

    // ─── Call Management ────────────────────────────────────────────────────

    /**
     * Initiate an outbound call via Vapi.
     *
     * @param phoneNumber The phone number to call (E.164 format)
     * @param assistantId Optional Vapi assistant ID (uses default if null)
     * @param systemPrompt Optional system prompt for an inline assistant
     * @param firstMessage Optional first message the assistant will say
     * @return The call record, or null if the call failed to initiate
     */
    suspend fun makeCall(
        phoneNumber: String,
        assistantId: String? = null,
        systemPrompt: String? = null,
        firstMessage: String? = null
    ): CallRecord? {
        val api = vapiApi ?: run {
            Timber.e("Vapi API not initialized")
            return null
        }

        val config = configManager.configuration.first()
        val callId = UUID.randomUUID().toString()

        Timber.i("Initiating Vapi call to $phoneNumber (callId: $callId)")

        val request = VapiCreateCallRequest(
            name = "OpenClaw Call $callId",
            assistantId = assistantId ?: config.vapiDefaultAssistantId,
            assistant = if (assistantId == null && config.vapiDefaultAssistantId == null) {
                // Create an inline assistant if no ID is provided
                VapiAssistantOverride(
                    model = VapiModel(
                        provider = "openai",
                        model = "gpt-4o",
                        messages = listOf(
                            VapiMessage(
                                role = "system",
                                content = systemPrompt
                                    ?: "You are a helpful AI assistant making a phone call on behalf of the user. Be concise, friendly, and professional."
                            )
                        )
                    ),
                    firstMessage = firstMessage
                        ?: "Hello! I'm calling on behalf of your OpenClaw assistant. How can I help you today?",
                    voice = VapiVoice(provider = "11labs", voiceId = "rachel")
                )
            } else null,
            phoneNumberId = config.vapiPhoneNumberId,
            customer = VapiCustomer(
                number = phoneNumber
            )
        )

        return try {
            val response = api.createCall(request)

            if (response.isSuccessful) {
                val vapiCall = response.body()!!
                val record = CallRecord(
                    id = callId,
                    provider = CallProvider.VAPI,
                    direction = CallDirection.OUTBOUND,
                    state = CallState.INITIATING,
                    phoneNumber = phoneNumber,
                    assistantId = vapiCall.assistantId,
                    vapiCallId = vapiCall.id,
                    startedAt = System.currentTimeMillis()
                )

                activeCalls[callId] = record
                startPollingCallStatus(callId, vapiCall.id)
                updateNotification("Call active: $phoneNumber")

                serviceScope.launch {
                    _callEvents.emit(CallEvent(callId, CallEventType.CALL_STARTED, record))
                }

                Timber.i("Vapi call initiated successfully: ${vapiCall.id}")
                record
            } else {
                val errorBody = response.errorBody()?.string()
                Timber.e("Vapi call failed: ${response.code()} - $errorBody")

                serviceScope.launch {
                    _callEvents.emit(
                        CallEvent(callId, CallEventType.CALL_FAILED, error = errorBody)
                    )
                }
                null
            }
        } catch (e: Exception) {
            Timber.e(e, "Exception creating Vapi call")
            serviceScope.launch {
                _callEvents.emit(
                    CallEvent(callId, CallEventType.CALL_FAILED, error = e.message)
                )
            }
            null
        }
    }

    /**
     * End an active Vapi call.
     */
    suspend fun endCall(callId: String): Boolean {
        val api = vapiApi ?: return false
        val record = activeCalls[callId] ?: return false
        val vapiCallId = record.vapiCallId ?: return false

        return try {
            val response = api.endCall(vapiCallId)
            if (response.isSuccessful) {
                val updatedRecord = record.copy(
                    state = CallState.ENDED,
                    endedAt = System.currentTimeMillis()
                )
                activeCalls[callId] = updatedRecord
                pollingJobs[callId]?.cancel()
                pollingJobs.remove(callId)

                serviceScope.launch {
                    _callEvents.emit(CallEvent(callId, CallEventType.CALL_ENDED, updatedRecord))
                }

                Timber.i("Vapi call ended: $callId")
                true
            } else {
                Timber.e("Failed to end Vapi call: ${response.code()}")
                false
            }
        } catch (e: Exception) {
            Timber.e(e, "Exception ending Vapi call")
            false
        }
    }

    /**
     * Get the current status of a call.
     */
    suspend fun getCallStatus(callId: String): CallRecord? {
        return activeCalls[callId]
    }

    /**
     * Get all active calls.
     */
    fun getActiveCalls(): List<CallRecord> {
        return activeCalls.values.filter { it.state != CallState.ENDED && it.state != CallState.FAILED }
    }

    // ─── Status Polling ─────────────────────────────────────────────────────

    private fun startPollingCallStatus(callId: String, vapiCallId: String) {
        pollingJobs[callId]?.cancel()
        pollingJobs[callId] = serviceScope.launch {
            while (isActive) {
                delay(POLL_INTERVAL_MS)
                pollCallStatus(callId, vapiCallId)
            }
        }
    }

    private suspend fun pollCallStatus(callId: String, vapiCallId: String) {
        val api = vapiApi ?: return

        try {
            val response = api.getCall(vapiCallId)
            if (response.isSuccessful) {
                val vapiCall = response.body() ?: return
                val currentRecord = activeCalls[callId] ?: return

                val newState = mapVapiStatus(vapiCall.status)
                if (newState != currentRecord.state) {
                    val updatedRecord = currentRecord.copy(
                        state = newState,
                        transcript = vapiCall.transcript ?: currentRecord.transcript,
                        endedAt = if (newState == CallState.ENDED) System.currentTimeMillis() else currentRecord.endedAt
                    )
                    activeCalls[callId] = updatedRecord

                    val eventType = when (newState) {
                        CallState.RINGING -> CallEventType.CALL_RINGING
                        CallState.ACTIVE -> CallEventType.CALL_ACTIVE
                        CallState.ENDED -> CallEventType.CALL_ENDED
                        CallState.FAILED -> CallEventType.CALL_FAILED
                        else -> CallEventType.STATUS_UPDATE
                    }

                    _callEvents.emit(CallEvent(callId, eventType, updatedRecord))

                    if (newState == CallState.ENDED || newState == CallState.FAILED) {
                        pollingJobs[callId]?.cancel()
                        pollingJobs.remove(callId)
                        updateNotification("Vapi service ready")
                    }
                }
            }
        } catch (e: Exception) {
            Timber.e(e, "Error polling Vapi call status")
        }
    }

    private fun mapVapiStatus(status: String?): CallState {
        return when (status?.lowercase()) {
            "queued" -> CallState.INITIATING
            "ringing" -> CallState.RINGING
            "in-progress" -> CallState.ACTIVE
            "forwarding" -> CallState.ACTIVE
            "ended" -> CallState.ENDED
            "failed" -> CallState.FAILED
            else -> CallState.INITIATING
        }
    }

    // ─── Assistant Management ───────────────────────────────────────────────

    /**
     * Create a new Vapi assistant for use with calls.
     */
    suspend fun createAssistant(
        systemPrompt: String,
        firstMessage: String,
        voiceId: String = "rachel"
    ): String? {
        val api = vapiApi ?: return null

        val request = VapiAssistantOverride(
            model = VapiModel(
                provider = "openai",
                model = "gpt-4o",
                messages = listOf(VapiMessage("system", systemPrompt))
            ),
            firstMessage = firstMessage,
            voice = VapiVoice(provider = "11labs", voiceId = voiceId)
        )

        return try {
            val response = api.createAssistant(request)
            if (response.isSuccessful) {
                response.body()?.id
            } else {
                Timber.e("Failed to create assistant: ${response.code()}")
                null
            }
        } catch (e: Exception) {
            Timber.e(e, "Exception creating assistant")
            null
        }
    }

    // ─── Notification ───────────────────────────────────────────────────────

    private fun createNotification(text: String): Notification {
        val pendingIntent = PendingIntent.getActivity(
            this, 0,
            packageManager.getLaunchIntentForPackage(packageName),
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )

        return NotificationCompat.Builder(this, OpenClawApplication.CHANNEL_ACTIVE_CALL)
            .setContentTitle("Vapi Voice Agent")
            .setContentText(text)
            .setSmallIcon(R.drawable.ic_notification)
            .setContentIntent(pendingIntent)
            .setOngoing(true)
            .setSilent(true)
            .build()
    }

    private fun updateNotification(text: String) {
        val notification = createNotification(text)
        val manager = getSystemService(NOTIFICATION_SERVICE) as android.app.NotificationManager
        manager.notify(NOTIFICATION_ID, notification)
    }
}
