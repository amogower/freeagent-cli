package com.openclaw.callingnode.service.whatsapp

import android.content.Context
import dagger.hilt.android.qualifiers.ApplicationContext
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.SharedFlow
import kotlinx.coroutines.flow.asSharedFlow
import org.webrtc.*
import timber.log.Timber
import javax.inject.Inject
import javax.inject.Singleton

/**
 * Manages WebRTC peer connections for WhatsApp Business Calling API.
 *
 * The WhatsApp Calling API uses WebRTC for real-time media transport:
 * - ICE + DTLS + SRTP for secure media
 * - OPUS audio codec
 * - Signaling via Graph API webhooks (not WebSocket)
 *
 * This manager handles:
 * - PeerConnection lifecycle
 * - SDP offer/answer negotiation
 * - ICE candidate gathering and exchange
 * - Audio track management
 * - Connection state monitoring
 */
@Singleton
class WebRTCManager @Inject constructor(
    @ApplicationContext private val context: Context
) {
    private var peerConnectionFactory: PeerConnectionFactory? = null
    private var peerConnection: PeerConnection? = null
    private var localAudioTrack: AudioTrack? = null
    private var audioSource: AudioSource? = null

    private val _webRTCEvents = MutableSharedFlow<WebRTCEvent>(extraBufferCapacity = 32)
    val webRTCEvents: SharedFlow<WebRTCEvent> = _webRTCEvents.asSharedFlow()

    sealed class WebRTCEvent {
        data class IceCandidateGenerated(val candidate: IceCandidate) : WebRTCEvent()
        data class IceConnectionStateChanged(val state: PeerConnection.IceConnectionState) : WebRTCEvent()
        data class SdpCreated(val sdp: SessionDescription) : WebRTCEvent()
        data class Error(val message: String) : WebRTCEvent()
        data object Connected : WebRTCEvent()
        data object Disconnected : WebRTCEvent()
    }

    /**
     * Initialize the WebRTC factory. Must be called before creating connections.
     */
    fun initialize() {
        Timber.i("Initializing WebRTC")

        val options = PeerConnectionFactory.InitializationOptions.builder(context)
            .setEnableInternalTracer(false)
            .createInitializationOptions()
        PeerConnectionFactory.initialize(options)

        val encoderFactory = DefaultVideoEncoderFactory(
            EglBase.create().eglBaseContext, true, true
        )
        val decoderFactory = DefaultVideoDecoderFactory(
            EglBase.create().eglBaseContext
        )

        peerConnectionFactory = PeerConnectionFactory.builder()
            .setAudioDeviceModule(
                JavaAudioDeviceModule.builder(context)
                    .setUseHardwareAcousticEchoCanceler(true)
                    .setUseHardwareNoiseSuppressor(true)
                    .createAudioDeviceModule()
            )
            .setVideoEncoderFactory(encoderFactory)
            .setVideoDecoderFactory(decoderFactory)
            .createPeerConnectionFactory()

        Timber.i("WebRTC factory initialized")
    }

    /**
     * Create a new peer connection for a WhatsApp call.
     *
     * WhatsApp uses standard STUN/TURN servers for ICE connectivity.
     * The actual TURN credentials are provided via the Graph API
     * during call setup.
     */
    fun createPeerConnection(
        iceServers: List<PeerConnection.IceServer> = getDefaultIceServers()
    ): Boolean {
        val factory = peerConnectionFactory ?: run {
            Timber.e("PeerConnectionFactory not initialized")
            return false
        }

        val rtcConfig = PeerConnection.RTCConfiguration(iceServers).apply {
            sdpSemantics = PeerConnection.SdpSemantics.UNIFIED_PLAN
            continualGatheringPolicy = PeerConnection.ContinualGatheringPolicy.GATHER_CONTINUALLY
            bundlePolicy = PeerConnection.BundlePolicy.MAXBUNDLE
            rtcpMuxPolicy = PeerConnection.RtcpMuxPolicy.REQUIRE
        }

        peerConnection = factory.createPeerConnection(rtcConfig, object : PeerConnection.Observer {
            override fun onSignalingChange(state: PeerConnection.SignalingState?) {
                Timber.d("Signaling state: $state")
            }

            override fun onIceConnectionChange(state: PeerConnection.IceConnectionState?) {
                Timber.i("ICE connection state: $state")
                state?.let {
                    _webRTCEvents.tryEmit(WebRTCEvent.IceConnectionStateChanged(it))
                    when (it) {
                        PeerConnection.IceConnectionState.CONNECTED ->
                            _webRTCEvents.tryEmit(WebRTCEvent.Connected)
                        PeerConnection.IceConnectionState.DISCONNECTED,
                        PeerConnection.IceConnectionState.FAILED ->
                            _webRTCEvents.tryEmit(WebRTCEvent.Disconnected)
                        else -> {}
                    }
                }
            }

            override fun onIceConnectionReceivingChange(receiving: Boolean) {
                Timber.d("ICE receiving: $receiving")
            }

            override fun onIceGatheringChange(state: PeerConnection.IceGatheringState?) {
                Timber.d("ICE gathering state: $state")
            }

            override fun onIceCandidate(candidate: IceCandidate?) {
                candidate?.let {
                    Timber.d("ICE candidate generated: ${it.sdpMid}")
                    _webRTCEvents.tryEmit(WebRTCEvent.IceCandidateGenerated(it))
                }
            }

            override fun onIceCandidatesRemoved(candidates: Array<out IceCandidate>?) {
                Timber.d("ICE candidates removed")
            }

            override fun onAddStream(stream: MediaStream?) {
                Timber.d("Remote stream added")
            }

            override fun onRemoveStream(stream: MediaStream?) {
                Timber.d("Remote stream removed")
            }

            override fun onDataChannel(channel: DataChannel?) {
                Timber.d("Data channel: ${channel?.label()}")
            }

            override fun onRenegotiationNeeded() {
                Timber.d("Renegotiation needed")
            }

            override fun onAddTrack(receiver: RtpReceiver?, streams: Array<out MediaStream>?) {
                Timber.d("Track added: ${receiver?.track()?.kind()}")
            }
        })

        if (peerConnection == null) {
            Timber.e("Failed to create PeerConnection")
            return false
        }

        // Add local audio track
        addLocalAudioTrack()

        Timber.i("PeerConnection created successfully")
        return true
    }

    /**
     * Add a local audio track for the microphone.
     */
    private fun addLocalAudioTrack() {
        val factory = peerConnectionFactory ?: return

        val audioConstraints = MediaConstraints().apply {
            mandatory.add(MediaConstraints.KeyValuePair("googEchoCancellation", "true"))
            mandatory.add(MediaConstraints.KeyValuePair("googNoiseSuppression", "true"))
            mandatory.add(MediaConstraints.KeyValuePair("googAutoGainControl", "true"))
            mandatory.add(MediaConstraints.KeyValuePair("googHighpassFilter", "true"))
        }

        audioSource = factory.createAudioSource(audioConstraints)
        localAudioTrack = factory.createAudioTrack("openclaw-audio-0", audioSource)
        localAudioTrack?.setEnabled(true)

        peerConnection?.addTrack(localAudioTrack, listOf("openclaw-stream-0"))
        Timber.d("Local audio track added")
    }

    /**
     * Create an SDP offer for initiating a call.
     */
    fun createOffer(callback: (SessionDescription?) -> Unit) {
        val constraints = MediaConstraints().apply {
            mandatory.add(MediaConstraints.KeyValuePair("OfferToReceiveAudio", "true"))
            mandatory.add(MediaConstraints.KeyValuePair("OfferToReceiveVideo", "false"))
        }

        peerConnection?.createOffer(object : SdpObserver {
            override fun onCreateSuccess(sdp: SessionDescription?) {
                sdp?.let {
                    peerConnection?.setLocalDescription(object : SdpObserver {
                        override fun onCreateSuccess(p0: SessionDescription?) {}
                        override fun onSetSuccess() {
                            Timber.d("Local description set (offer)")
                            _webRTCEvents.tryEmit(WebRTCEvent.SdpCreated(it))
                            callback(it)
                        }
                        override fun onCreateFailure(error: String?) {
                            Timber.e("Failed to set local description: $error")
                            callback(null)
                        }
                        override fun onSetFailure(error: String?) {
                            Timber.e("Failed to set local description: $error")
                            callback(null)
                        }
                    }, it)
                }
            }
            override fun onSetSuccess() {}
            override fun onCreateFailure(error: String?) {
                Timber.e("Failed to create offer: $error")
                callback(null)
            }
            override fun onSetFailure(error: String?) {}
        }, constraints)
    }

    /**
     * Create an SDP answer in response to a remote offer.
     */
    fun createAnswer(callback: (SessionDescription?) -> Unit) {
        val constraints = MediaConstraints().apply {
            mandatory.add(MediaConstraints.KeyValuePair("OfferToReceiveAudio", "true"))
            mandatory.add(MediaConstraints.KeyValuePair("OfferToReceiveVideo", "false"))
        }

        peerConnection?.createAnswer(object : SdpObserver {
            override fun onCreateSuccess(sdp: SessionDescription?) {
                sdp?.let {
                    peerConnection?.setLocalDescription(object : SdpObserver {
                        override fun onCreateSuccess(p0: SessionDescription?) {}
                        override fun onSetSuccess() {
                            Timber.d("Local description set (answer)")
                            _webRTCEvents.tryEmit(WebRTCEvent.SdpCreated(it))
                            callback(it)
                        }
                        override fun onCreateFailure(error: String?) {
                            callback(null)
                        }
                        override fun onSetFailure(error: String?) {
                            callback(null)
                        }
                    }, it)
                }
            }
            override fun onSetSuccess() {}
            override fun onCreateFailure(error: String?) {
                Timber.e("Failed to create answer: $error")
                callback(null)
            }
            override fun onSetFailure(error: String?) {}
        }, constraints)
    }

    /**
     * Set the remote SDP description (offer or answer from WhatsApp).
     */
    fun setRemoteDescription(sdp: SessionDescription, callback: (Boolean) -> Unit) {
        peerConnection?.setRemoteDescription(object : SdpObserver {
            override fun onCreateSuccess(p0: SessionDescription?) {}
            override fun onSetSuccess() {
                Timber.d("Remote description set: ${sdp.type}")
                callback(true)
            }
            override fun onCreateFailure(error: String?) {
                Timber.e("Failed to set remote description: $error")
                callback(false)
            }
            override fun onSetFailure(error: String?) {
                Timber.e("Failed to set remote description: $error")
                callback(false)
            }
        }, sdp)
    }

    /**
     * Add a remote ICE candidate received from WhatsApp signaling.
     */
    fun addIceCandidate(candidate: IceCandidate): Boolean {
        return peerConnection?.addIceCandidate(candidate) ?: false
    }

    /**
     * Mute/unmute the local audio track.
     */
    fun setMuted(muted: Boolean) {
        localAudioTrack?.setEnabled(!muted)
        Timber.d("Audio ${if (muted) "muted" else "unmuted"}")
    }

    /**
     * Close the peer connection and release resources.
     */
    fun closePeerConnection() {
        Timber.i("Closing WebRTC peer connection")
        localAudioTrack?.setEnabled(false)
        localAudioTrack?.dispose()
        localAudioTrack = null
        audioSource?.dispose()
        audioSource = null
        peerConnection?.close()
        peerConnection?.dispose()
        peerConnection = null
    }

    /**
     * Release all WebRTC resources.
     */
    fun release() {
        closePeerConnection()
        peerConnectionFactory?.dispose()
        peerConnectionFactory = null
        Timber.i("WebRTC resources released")
    }

    private fun getDefaultIceServers(): List<PeerConnection.IceServer> {
        return listOf(
            PeerConnection.IceServer.builder("stun:stun.l.google.com:19302").createIceServer(),
            PeerConnection.IceServer.builder("stun:stun1.l.google.com:19302").createIceServer(),
            PeerConnection.IceServer.builder("stun:stun2.l.google.com:19302").createIceServer()
        )
    }
}
