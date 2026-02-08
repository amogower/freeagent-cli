package com.openclaw.callingnode.service.telephony

import android.net.Uri
import android.os.Bundle
import android.telecom.Connection
import android.telecom.ConnectionRequest
import android.telecom.ConnectionService
import android.telecom.PhoneAccountHandle
import android.telecom.TelecomManager
import timber.log.Timber

/**
 * Native Android Telephony integration via TelecomManager.
 *
 * This ConnectionService allows the OpenClaw Calling Node to integrate
 * with the native Android dialer and call management system. When
 * registered as a PhoneAccount, the system can route calls through
 * this service, enabling:
 *
 * - Appearing in the native call UI
 * - Integration with Bluetooth headsets and car systems
 * - Call audio routing through the standard Android audio pipeline
 * - Interaction with the system's Do Not Disturb settings
 *
 * This is primarily used as a bridge: the actual call audio is handled
 * by either Vapi (via their infrastructure) or WhatsApp (via WebRTC),
 * but this service provides the native Android call management layer.
 */
class NativeTelephonyService : ConnectionService() {

    companion object {
        const val PHONE_ACCOUNT_ID = "openclaw_calling_node"
        const val EXTRA_CALL_PROVIDER = "com.openclaw.callingnode.CALL_PROVIDER"
        const val EXTRA_CALL_ID = "com.openclaw.callingnode.CALL_ID"
    }

    override fun onCreateOutgoingConnection(
        connectionManagerPhoneAccount: PhoneAccountHandle?,
        request: ConnectionRequest?
    ): Connection {
        Timber.i("Creating outgoing connection: ${request?.address}")

        return OpenClawConnection().apply {
            setAddress(request?.address, TelecomManager.PRESENTATION_ALLOWED)
            setCallerDisplayName("OpenClaw Agent", TelecomManager.PRESENTATION_ALLOWED)
            setInitializing()

            // Extract provider info from extras
            request?.extras?.let { extras ->
                val provider = extras.getString(EXTRA_CALL_PROVIDER, "vapi")
                val callId = extras.getString(EXTRA_CALL_ID, "")
                Timber.d("Outgoing call via $provider (callId: $callId)")
            }
        }
    }

    override fun onCreateIncomingConnection(
        connectionManagerPhoneAccount: PhoneAccountHandle?,
        request: ConnectionRequest?
    ): Connection {
        Timber.i("Creating incoming connection: ${request?.address}")

        return OpenClawConnection().apply {
            setAddress(request?.address, TelecomManager.PRESENTATION_ALLOWED)
            setCallerDisplayName("OpenClaw Incoming", TelecomManager.PRESENTATION_ALLOWED)
            setRinging()
        }
    }

    override fun onCreateOutgoingConnectionFailed(
        connectionManagerPhoneAccount: PhoneAccountHandle?,
        request: ConnectionRequest?
    ) {
        Timber.e("Failed to create outgoing connection: ${request?.address}")
    }

    override fun onCreateIncomingConnectionFailed(
        connectionManagerPhoneAccount: PhoneAccountHandle?,
        request: ConnectionRequest?
    ) {
        Timber.e("Failed to create incoming connection: ${request?.address}")
    }

    /**
     * Represents a single call connection managed by the OpenClaw node.
     */
    inner class OpenClawConnection : Connection() {

        init {
            connectionProperties = PROPERTY_SELF_MANAGED
            audioModeIsVoip = true
        }

        override fun onAnswer() {
            Timber.i("Call answered")
            setActive()
        }

        override fun onReject() {
            Timber.i("Call rejected")
            setDisconnected(android.telecom.DisconnectCause(android.telecom.DisconnectCause.REJECTED))
            destroy()
        }

        override fun onDisconnect() {
            Timber.i("Call disconnected")
            setDisconnected(android.telecom.DisconnectCause(android.telecom.DisconnectCause.LOCAL))
            destroy()
        }

        override fun onHold() {
            Timber.i("Call held")
            setOnHold()
        }

        override fun onUnhold() {
            Timber.i("Call unheld")
            setActive()
        }

        override fun onCallAudioStateChanged(state: android.telecom.CallAudioState?) {
            Timber.d("Audio state changed: route=${state?.route}, muted=${state?.isMuted}")
        }

        override fun onShowIncomingCallUi() {
            Timber.d("Show incoming call UI requested")
        }

        override fun onSilence() {
            Timber.d("Call silenced")
        }
    }
}
