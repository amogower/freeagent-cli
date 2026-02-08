package com.openclaw.callingnode.gateway

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
import okhttp3.*
import timber.log.Timber
import java.util.concurrent.TimeUnit
import javax.inject.Inject

/**
 * Foreground service that maintains a persistent WebSocket connection
 * to the OpenClaw Gateway. This is the primary communication channel
 * between the Android node and the central control plane.
 *
 * The service receives commands from the Gateway (e.g., "make a call")
 * and sends status updates back (e.g., "call connected").
 */
@AndroidEntryPoint
class GatewayConnectionService : Service() {

    @Inject lateinit var configManager: ConfigManager

    private val serviceScope = CoroutineScope(SupervisorJob() + Dispatchers.IO)
    private val json = Json { ignoreUnknownKeys = true; encodeDefaults = true }

    private var webSocket: WebSocket? = null
    private var reconnectJob: Job? = null
    private var reconnectAttempts = 0

    private val _connectionState = MutableStateFlow(ConnectionState.DISCONNECTED)
    val connectionState: StateFlow<ConnectionState> = _connectionState.asStateFlow()

    private val _incomingCommands = MutableSharedFlow<GatewayCommand>(extraBufferCapacity = 64)
    val incomingCommands: SharedFlow<GatewayCommand> = _incomingCommands.asSharedFlow()

    enum class ConnectionState {
        DISCONNECTED, CONNECTING, CONNECTED, RECONNECTING, FAILED
    }

    companion object {
        private const val NOTIFICATION_ID = 1001
        private const val PING_INTERVAL_MS = 30_000L

        @Volatile
        var instance: GatewayConnectionService? = null
            private set
    }

    override fun onCreate() {
        super.onCreate()
        instance = this
        Timber.i("GatewayConnectionService created")
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        startForeground(NOTIFICATION_ID, createNotification("Connecting to Gateway..."))

        serviceScope.launch {
            configManager.configuration.collect { config ->
                if (config.gatewayUrl.isNotEmpty()) {
                    connectToGateway(config)
                }
            }
        }

        return START_STICKY
    }

    override fun onBind(intent: Intent?): IBinder? = null

    override fun onDestroy() {
        instance = null
        disconnect()
        serviceScope.cancel()
        super.onDestroy()
        Timber.i("GatewayConnectionService destroyed")
    }

    // ─── Connection Management ───────────────────────────────────────────────

    private fun connectToGateway(config: NodeConfiguration) {
        disconnect()
        _connectionState.value = ConnectionState.CONNECTING

        val client = OkHttpClient.Builder()
            .pingInterval(PING_INTERVAL_MS, TimeUnit.MILLISECONDS)
            .readTimeout(0, TimeUnit.MILLISECONDS) // No timeout for WebSocket
            .build()

        val request = Request.Builder()
            .url(config.gatewayUrl)
            .apply {
                if (config.gatewayToken.isNotEmpty()) {
                    addHeader("Authorization", "Bearer ${config.gatewayToken}")
                }
                addHeader("X-Node-Type", "android-calling")
                addHeader("X-Node-Version", "1.0.0-alpha")
            }
            .build()

        Timber.i("Connecting to Gateway: ${config.gatewayUrl}")

        webSocket = client.newWebSocket(request, object : WebSocketListener() {
            override fun onOpen(webSocket: WebSocket, response: Response) {
                Timber.i("Gateway connection established")
                _connectionState.value = ConnectionState.CONNECTED
                reconnectAttempts = 0
                updateNotification("Connected to Gateway")

                // Send registration message
                val registration = json.encodeToString(
                    GatewayResponse.serializer(),
                    GatewayResponse(
                        id = java.util.UUID.randomUUID().toString(),
                        commandId = "registration",
                        type = ResponseType.PONG,
                        payload = ResponsePayload(
                            message = "Android Calling Node v1.0.0-alpha registered"
                        )
                    )
                )
                webSocket.send(registration)
            }

            override fun onMessage(webSocket: WebSocket, text: String) {
                Timber.d("Gateway message received: ${text.take(200)}")
                try {
                    val command = json.decodeFromString(GatewayCommand.serializer(), text)
                    serviceScope.launch {
                        _incomingCommands.emit(command)
                    }
                } catch (e: Exception) {
                    Timber.e(e, "Failed to parse gateway command")
                }
            }

            override fun onClosing(webSocket: WebSocket, code: Int, reason: String) {
                Timber.i("Gateway connection closing: $code $reason")
                webSocket.close(1000, null)
            }

            override fun onClosed(webSocket: WebSocket, code: Int, reason: String) {
                Timber.i("Gateway connection closed: $code $reason")
                _connectionState.value = ConnectionState.DISCONNECTED
                updateNotification("Disconnected from Gateway")
                scheduleReconnect(config)
            }

            override fun onFailure(webSocket: WebSocket, t: Throwable, response: Response?) {
                Timber.e(t, "Gateway connection failed")
                _connectionState.value = ConnectionState.DISCONNECTED
                updateNotification("Connection failed - retrying...")
                scheduleReconnect(config)
            }
        })
    }

    private fun disconnect() {
        reconnectJob?.cancel()
        webSocket?.close(1000, "Service stopping")
        webSocket = null
        _connectionState.value = ConnectionState.DISCONNECTED
    }

    private fun scheduleReconnect(config: NodeConfiguration) {
        if (!config.autoReconnect) return
        if (reconnectAttempts >= config.maxReconnectAttempts) {
            _connectionState.value = ConnectionState.FAILED
            updateNotification("Connection failed after ${config.maxReconnectAttempts} attempts")
            return
        }

        reconnectJob?.cancel()
        reconnectJob = serviceScope.launch {
            _connectionState.value = ConnectionState.RECONNECTING
            val delay = config.reconnectDelayMs * (reconnectAttempts + 1)
            Timber.i("Reconnecting in ${delay}ms (attempt ${reconnectAttempts + 1})")
            delay(delay)
            reconnectAttempts++
            connectToGateway(config)
        }
    }

    // ─── Outbound Messages ──────────────────────────────────────────────────

    /**
     * Send a response back to the OpenClaw Gateway.
     */
    fun sendResponse(response: GatewayResponse) {
        val ws = webSocket
        if (ws == null || _connectionState.value != ConnectionState.CONNECTED) {
            Timber.w("Cannot send response: not connected to Gateway")
            return
        }

        try {
            val message = json.encodeToString(GatewayResponse.serializer(), response)
            ws.send(message)
            Timber.d("Response sent to Gateway: ${response.type}")
        } catch (e: Exception) {
            Timber.e(e, "Failed to send response to Gateway")
        }
    }

    // ─── Notification Management ────────────────────────────────────────────

    private fun createNotification(text: String): Notification {
        val pendingIntent = PendingIntent.getActivity(
            this, 0,
            packageManager.getLaunchIntentForPackage(packageName),
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )

        return NotificationCompat.Builder(this, OpenClawApplication.CHANNEL_GATEWAY)
            .setContentTitle("OpenClaw Calling Node")
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
