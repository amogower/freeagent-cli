package com.openclaw.callingnode.service.whatsapp

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
import org.webrtc.IceCandidate
import org.webrtc.PeerConnection
import org.webrtc.SessionDescription
import retrofit2.Retrofit
import retrofit2.converter.kotlinx.serialization.asConverterFactory
import timber.log.Timber
import java.util.UUID
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.TimeUnit
import javax.inject.Inject

/**
 * Service responsible for managing voice calls through the WhatsApp Business
 * Calling API with WebRTC media transport.
 *
 * This service handles the full lifecycle of WhatsApp voice calls:
 * 1. Requesting call permissions from users
 * 2. Initiating business-to-user calls via Graph API
 * 3. Receiving user-to-business calls via webhooks
 * 4. Managing WebRTC peer connections for audio
 * 5. Handling ICE candidate exchange and SDP negotiation
 *
 * Architecture:
 * - Signaling: Meta Graph API + Webhooks (HTTPS)
 * - Media: WebRTC (ICE + DTLS + SRTP) with OPUS codec
 * - The service acts as a bridge between the Graph API signaling
 *   and the local WebRTC peer connection
 */
@AndroidEntryPoint
class WhatsAppCallingService : Service() {

    @Inject lateinit var configManager: ConfigManager
    @Inject lateinit var webRTCManager: WebRTCManager

    private val serviceScope = CoroutineScope(SupervisorJob() + Dispatchers.IO)
    private val json = Json { ignoreUnknownKeys = true; encodeDefaults = true }

    private var whatsAppApi: WhatsAppApi? = null
    private val activeCalls = ConcurrentHashMap<String, CallRecord>()
    private val callPermissions = ConcurrentHashMap<String, Boolean>() // phone -> hasPermission

    private val _callEvents = MutableSharedFlow<WhatsAppCallEvent>(extraBufferCapacity = 64)
    val callEvents: SharedFlow<WhatsAppCallEvent> = _callEvents.asSharedFlow()

    data class WhatsAppCallEvent(
        val callId: String,
        val type: WhatsAppCallEventType,
        val callRecord: CallRecord? = null,
        val error: String? = null
    )

    enum class WhatsAppCallEventType {
        PERMISSION_REQUESTED,
        PERMISSION_GRANTED,
        PERMISSION_DENIED,
        CALL_INITIATED,
        CALL_RINGING,
        CALL_CONNECTED,
        CALL_ENDED,
        CALL_FAILED,
        INCOMING_CALL,
        WEBRTC_CONNECTED,
        WEBRTC_DISCONNECTED
    }

    companion object {
        private const val NOTIFICATION_ID = 3001
        private const val GRAPH_API_VERSION = "v21.0"
        private const val GRAPH_API_BASE_URL = "https://graph.facebook.com/$GRAPH_API_VERSION/"

        @Volatile
        var instance: WhatsAppCallingService? = null
            private set
    }

    override fun onCreate() {
        super.onCreate()
        instance = this

        // Initialize WebRTC
        webRTCManager.initialize()

        // Observe WebRTC events
        serviceScope.launch {
            webRTCManager.webRTCEvents.collect { event ->
                handleWebRTCEvent(event)
            }
        }

        // Initialize API when config is available
        serviceScope.launch {
            configManager.configuration.collect { config ->
                initializeApi(config)
            }
        }

        Timber.i("WhatsAppCallingService created")
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        startForeground(NOTIFICATION_ID, createNotification("WhatsApp calling ready"))
        return START_STICKY
    }

    override fun onBind(intent: Intent?): IBinder? = null

    override fun onDestroy() {
        instance = null
        webRTCManager.release()
        serviceScope.cancel()
        super.onDestroy()
        Timber.i("WhatsAppCallingService destroyed")
    }

    // ─── API Initialization ─────────────────────────────────────────────────

    private fun initializeApi(config: NodeConfiguration) {
        if (config.whatsappAccessToken.isEmpty()) {
            Timber.w("WhatsApp access token not configured")
            return
        }

        val loggingInterceptor = HttpLoggingInterceptor { message ->
            Timber.tag("WhatsAppHttp").d(message)
        }.apply {
            level = HttpLoggingInterceptor.Level.BODY
        }

        val client = OkHttpClient.Builder()
            .addInterceptor { chain ->
                val request = chain.request().newBuilder()
                    .addHeader("Authorization", "Bearer ${config.whatsappAccessToken}")
                    .addHeader("Content-Type", "application/json")
                    .build()
                chain.proceed(request)
            }
            .addInterceptor(loggingInterceptor)
            .connectTimeout(30, TimeUnit.SECONDS)
            .readTimeout(60, TimeUnit.SECONDS)
            .writeTimeout(30, TimeUnit.SECONDS)
            .build()

        val contentType = "application/json".toMediaType()

        whatsAppApi = Retrofit.Builder()
            .baseUrl(GRAPH_API_BASE_URL)
            .client(client)
            .addConverterFactory(json.asConverterFactory(contentType))
            .build()
            .create(WhatsAppApi::class.java)

        Timber.i("WhatsApp API initialized (Graph API $GRAPH_API_VERSION)")
    }

    // ─── Call Permission Management ─────────────────────────────────────────

    /**
     * Request call permission from a WhatsApp user.
     *
     * Business-initiated calls require explicit user consent. This sends
     * a template message asking the user to grant call permission.
     *
     * @param phoneNumber The user's WhatsApp number (E.164 format)
     * @param templateName The approved template name for permission requests
     * @return true if the permission request was sent successfully
     */
    suspend fun requestCallPermission(
        phoneNumber: String,
        templateName: String = "call_permission_request"
    ): Boolean {
        val api = whatsAppApi ?: run {
            Timber.e("WhatsApp API not initialized")
            return false
        }

        val config = configManager.configuration.first()

        val request = WhatsAppCallPermissionMessageRequest(
            to = phoneNumber,
            template = WhatsAppPermissionTemplate(
                name = templateName,
                language = WhatsAppPermissionLanguage("en_US")
            )
        )

        return try {
            val response = api.requestCallPermission(config.whatsappPhoneNumberId, request)
            if (response.isSuccessful) {
                Timber.i("Call permission requested for $phoneNumber")
                serviceScope.launch {
                    _callEvents.emit(
                        WhatsAppCallEvent(
                            callId = UUID.randomUUID().toString(),
                            type = WhatsAppCallEventType.PERMISSION_REQUESTED
                        )
                    )
                }
                true
            } else {
                val error = response.errorBody()?.string()
                Timber.e("Failed to request call permission: ${response.code()} - $error")
                false
            }
        } catch (e: Exception) {
            Timber.e(e, "Exception requesting call permission")
            false
        }
    }

    // ─── Outbound Call Management ───────────────────────────────────────────

    /**
     * Initiate a business-to-user WhatsApp voice call.
     *
     * This creates a WebRTC peer connection, generates an SDP offer,
     * and sends it to WhatsApp via the Graph API. The call will ring
     * on the user's WhatsApp client.
     *
     * @param phoneNumber The user's WhatsApp number (E.164 format)
     * @return The call record, or null if initiation failed
     */
    suspend fun makeCall(phoneNumber: String): CallRecord? {
        val api = whatsAppApi ?: run {
            Timber.e("WhatsApp API not initialized")
            return null
        }

        val config = configManager.configuration.first()
        val callId = UUID.randomUUID().toString()

        Timber.i("Initiating WhatsApp call to $phoneNumber (callId: $callId)")

        // Create WebRTC peer connection
        if (!webRTCManager.createPeerConnection()) {
            Timber.e("Failed to create WebRTC peer connection")
            return null
        }

        // Create SDP offer
        val sdpOffer = suspendCancellableCoroutine<SessionDescription?> { cont ->
            webRTCManager.createOffer { sdp ->
                cont.resume(sdp) {}
            }
        }

        if (sdpOffer == null) {
            Timber.e("Failed to create SDP offer")
            webRTCManager.closePeerConnection()
            return null
        }

        // Send call initiation to WhatsApp Graph API
        val request = WhatsAppInitiateCallRequest(
            to = phoneNumber,
            call = WhatsAppCallPayload(
                action = "create",
                sdp = WhatsAppSdpPayload(
                    type = "offer",
                    sdp = sdpOffer.description
                )
            )
        )

        return try {
            val response = api.initiateCall(config.whatsappPhoneNumberId, request)

            if (response.isSuccessful) {
                val apiResponse = response.body()!!

                // Set remote SDP answer if provided
                apiResponse.sdp?.let { remoteSdp ->
                    val answer = SessionDescription(
                        SessionDescription.Type.ANSWER,
                        remoteSdp.sdp
                    )
                    webRTCManager.setRemoteDescription(answer) { success ->
                        if (!success) {
                            Timber.e("Failed to set remote SDP answer")
                        }
                    }
                }

                // Add remote ICE candidates if provided
                apiResponse.iceCandidates?.forEach { candidate ->
                    webRTCManager.addIceCandidate(
                        IceCandidate(
                            candidate.sdpMid,
                            candidate.sdpMLineIndex ?: 0,
                            candidate.candidate
                        )
                    )
                }

                val record = CallRecord(
                    id = callId,
                    provider = CallProvider.WHATSAPP,
                    direction = CallDirection.OUTBOUND,
                    state = CallState.RINGING,
                    phoneNumber = phoneNumber,
                    whatsappCallId = apiResponse.callId,
                    startedAt = System.currentTimeMillis()
                )

                activeCalls[callId] = record
                updateNotification("WhatsApp call: $phoneNumber")

                serviceScope.launch {
                    _callEvents.emit(
                        WhatsAppCallEvent(callId, WhatsAppCallEventType.CALL_INITIATED, record)
                    )
                }

                Timber.i("WhatsApp call initiated: ${apiResponse.callId}")
                record
            } else {
                val errorBody = response.errorBody()?.string()
                Timber.e("WhatsApp call failed: ${response.code()} - $errorBody")
                webRTCManager.closePeerConnection()

                serviceScope.launch {
                    _callEvents.emit(
                        WhatsAppCallEvent(callId, WhatsAppCallEventType.CALL_FAILED, error = errorBody)
                    )
                }
                null
            }
        } catch (e: Exception) {
            Timber.e(e, "Exception initiating WhatsApp call")
            webRTCManager.closePeerConnection()
            serviceScope.launch {
                _callEvents.emit(
                    WhatsAppCallEvent(callId, WhatsAppCallEventType.CALL_FAILED, error = e.message)
                )
            }
            null
        }
    }

    /**
     * End an active WhatsApp call.
     */
    suspend fun endCall(callId: String): Boolean {
        val api = whatsAppApi ?: return false
        val record = activeCalls[callId] ?: return false
        val whatsappCallId = record.whatsappCallId ?: return false
        val config = configManager.configuration.first()

        return try {
            val request = WhatsAppEndCallRequest(
                call = WhatsAppCallPayload(
                    action = "end",
                    callId = whatsappCallId
                )
            )

            val response = api.endCall(config.whatsappPhoneNumberId, request)

            // Close WebRTC regardless of API response
            webRTCManager.closePeerConnection()

            val updatedRecord = record.copy(
                state = CallState.ENDED,
                endedAt = System.currentTimeMillis()
            )
            activeCalls[callId] = updatedRecord

            serviceScope.launch {
                _callEvents.emit(
                    WhatsAppCallEvent(callId, WhatsAppCallEventType.CALL_ENDED, updatedRecord)
                )
            }

            updateNotification("WhatsApp calling ready")
            Timber.i("WhatsApp call ended: $callId")
            true
        } catch (e: Exception) {
            Timber.e(e, "Exception ending WhatsApp call")
            webRTCManager.closePeerConnection()
            false
        }
    }

    // ─── Incoming Call Handling ──────────────────────────────────────────────

    /**
     * Handle an incoming WhatsApp call webhook notification.
     *
     * This is called when a user initiates a call to the business
     * WhatsApp number. The webhook contains the call ID and remote
     * SDP offer for WebRTC negotiation.
     */
    suspend fun handleIncomingCall(
        whatsappCallId: String,
        fromNumber: String,
        remoteSdp: String?
    ): CallRecord? {
        val api = whatsAppApi ?: return null
        val config = configManager.configuration.first()
        val callId = UUID.randomUUID().toString()

        Timber.i("Incoming WhatsApp call from $fromNumber (whatsappCallId: $whatsappCallId)")

        // Create WebRTC peer connection
        if (!webRTCManager.createPeerConnection()) {
            Timber.e("Failed to create WebRTC peer connection for incoming call")
            return null
        }

        // Set remote SDP offer if provided
        if (remoteSdp != null) {
            val offer = SessionDescription(SessionDescription.Type.OFFER, remoteSdp)
            val setSuccess = suspendCancellableCoroutine<Boolean> { cont ->
                webRTCManager.setRemoteDescription(offer) { success ->
                    cont.resume(success) {}
                }
            }

            if (!setSuccess) {
                Timber.e("Failed to set remote SDP offer")
                webRTCManager.closePeerConnection()
                return null
            }
        }

        // Create SDP answer
        val sdpAnswer = suspendCancellableCoroutine<SessionDescription?> { cont ->
            webRTCManager.createAnswer { sdp ->
                cont.resume(sdp) {}
            }
        }

        if (sdpAnswer == null) {
            Timber.e("Failed to create SDP answer")
            webRTCManager.closePeerConnection()
            return null
        }

        // Send answer to WhatsApp
        val answerRequest = WhatsAppAnswerCallRequest(
            call = WhatsAppCallPayload(
                action = "answer",
                callId = whatsappCallId,
                sdp = WhatsAppSdpPayload(
                    type = "answer",
                    sdp = sdpAnswer.description
                )
            )
        )

        return try {
            val response = api.answerCall(config.whatsappPhoneNumberId, answerRequest)

            if (response.isSuccessful) {
                val record = CallRecord(
                    id = callId,
                    provider = CallProvider.WHATSAPP,
                    direction = CallDirection.INBOUND,
                    state = CallState.ACTIVE,
                    phoneNumber = fromNumber,
                    whatsappCallId = whatsappCallId,
                    startedAt = System.currentTimeMillis()
                )

                activeCalls[callId] = record
                updateNotification("WhatsApp call: $fromNumber")

                serviceScope.launch {
                    _callEvents.emit(
                        WhatsAppCallEvent(callId, WhatsAppCallEventType.CALL_CONNECTED, record)
                    )
                }

                Timber.i("Incoming WhatsApp call answered: $callId")
                record
            } else {
                val error = response.errorBody()?.string()
                Timber.e("Failed to answer WhatsApp call: ${response.code()} - $error")
                webRTCManager.closePeerConnection()
                null
            }
        } catch (e: Exception) {
            Timber.e(e, "Exception answering WhatsApp call")
            webRTCManager.closePeerConnection()
            null
        }
    }

    /**
     * Handle ICE candidates received from WhatsApp webhook.
     */
    fun handleRemoteIceCandidates(candidates: List<WhatsAppIceCandidatePayload>) {
        candidates.forEach { candidate ->
            webRTCManager.addIceCandidate(
                IceCandidate(
                    candidate.sdpMid,
                    candidate.sdpMLineIndex ?: 0,
                    candidate.candidate
                )
            )
        }
    }

    // ─── WebRTC Event Handling ──────────────────────────────────────────────

    private suspend fun handleWebRTCEvent(event: WebRTCManager.WebRTCEvent) {
        when (event) {
            is WebRTCManager.WebRTCEvent.Connected -> {
                Timber.i("WebRTC connected")
                // Update all active calls to ACTIVE state
                activeCalls.forEach { (callId, record) ->
                    if (record.state == CallState.RINGING || record.state == CallState.INITIATING) {
                        val updated = record.copy(state = CallState.ACTIVE)
                        activeCalls[callId] = updated
                        _callEvents.emit(
                            WhatsAppCallEvent(callId, WhatsAppCallEventType.WEBRTC_CONNECTED, updated)
                        )
                    }
                }
            }

            is WebRTCManager.WebRTCEvent.Disconnected -> {
                Timber.i("WebRTC disconnected")
                activeCalls.forEach { (callId, record) ->
                    if (record.state == CallState.ACTIVE) {
                        val updated = record.copy(
                            state = CallState.ENDED,
                            endedAt = System.currentTimeMillis()
                        )
                        activeCalls[callId] = updated
                        _callEvents.emit(
                            WhatsAppCallEvent(callId, WhatsAppCallEventType.WEBRTC_DISCONNECTED, updated)
                        )
                    }
                }
            }

            is WebRTCManager.WebRTCEvent.IceCandidateGenerated -> {
                // Send ICE candidate to WhatsApp via Graph API
                sendIceCandidateToWhatsApp(event.candidate)
            }

            is WebRTCManager.WebRTCEvent.Error -> {
                Timber.e("WebRTC error: ${event.message}")
            }

            else -> {}
        }
    }

    private suspend fun sendIceCandidateToWhatsApp(candidate: IceCandidate) {
        val api = whatsAppApi ?: return
        val config = configManager.configuration.first()

        // Find the active call's WhatsApp call ID
        val activeCall = activeCalls.values.firstOrNull {
            it.state == CallState.RINGING || it.state == CallState.ACTIVE || it.state == CallState.INITIATING
        } ?: return

        val request = WhatsAppIceCandidateRequest(
            call = WhatsAppCallPayload(
                action = "ice",
                callId = activeCall.whatsappCallId,
                iceCandidates = listOf(
                    WhatsAppIceCandidatePayload(
                        candidate = candidate.sdp,
                        sdpMid = candidate.sdpMid,
                        sdpMLineIndex = candidate.sdpMLineIndex
                    )
                )
            )
        )

        try {
            api.sendIceCandidates(config.whatsappPhoneNumberId, request)
        } catch (e: Exception) {
            Timber.e(e, "Failed to send ICE candidate to WhatsApp")
        }
    }

    // ─── Utility ────────────────────────────────────────────────────────────

    fun getActiveCalls(): List<CallRecord> {
        return activeCalls.values.filter { it.state != CallState.ENDED && it.state != CallState.FAILED }
    }

    fun getCallStatus(callId: String): CallRecord? = activeCalls[callId]

    fun setMuted(muted: Boolean) {
        webRTCManager.setMuted(muted)
    }

    // ─── Notification ───────────────────────────────────────────────────────

    private fun createNotification(text: String): Notification {
        val pendingIntent = PendingIntent.getActivity(
            this, 0,
            packageManager.getLaunchIntentForPackage(packageName),
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )

        return NotificationCompat.Builder(this, OpenClawApplication.CHANNEL_ACTIVE_CALL)
            .setContentTitle("WhatsApp Voice Agent")
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
