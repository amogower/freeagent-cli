package com.openclaw.callingnode.service.whatsapp

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import retrofit2.Response
import retrofit2.http.*

/**
 * Retrofit interface for the WhatsApp Business Cloud API (Calling).
 *
 * The WhatsApp Business Calling API enables VoIP calls with WhatsApp users
 * using WebRTC for media transport and the Graph API for signaling.
 *
 * Architecture:
 * - Signaling: Graph API + Webhooks (HTTPS)
 * - Media: WebRTC (ICE + DTLS + SRTP)
 * - Audio codec: OPUS
 *
 * @see <a href="https://developers.facebook.com/documentation/business-messaging/whatsapp/calling">WhatsApp Calling API</a>
 */
interface WhatsAppApi {

    // ─── Call Management ────────────────────────────────────────────────────

    /**
     * Initiate a business-initiated call to a WhatsApp user.
     *
     * Prerequisites:
     * - User must have granted call permission
     * - Business must have 2000+ BIC messaging limit
     * - Calling must be enabled on the phone number
     *
     * Note: Business-initiated calling is NOT available in USA, Canada,
     * Turkey, Egypt, Vietnam, or Nigeria.
     */
    @POST("{phoneNumberId}/calls")
    suspend fun initiateCall(
        @Path("phoneNumberId") phoneNumberId: String,
        @Body request: WhatsAppInitiateCallRequest
    ): Response<WhatsAppCallApiResponse>

    /**
     * Answer an incoming user-initiated call.
     *
     * When a user calls your business WhatsApp number, you receive a
     * webhook notification. Use this endpoint to answer the call and
     * establish the WebRTC media session.
     */
    @POST("{phoneNumberId}/calls")
    suspend fun answerCall(
        @Path("phoneNumberId") phoneNumberId: String,
        @Body request: WhatsAppAnswerCallRequest
    ): Response<WhatsAppCallApiResponse>

    /**
     * End an active call.
     */
    @POST("{phoneNumberId}/calls")
    suspend fun endCall(
        @Path("phoneNumberId") phoneNumberId: String,
        @Body request: WhatsAppEndCallRequest
    ): Response<WhatsAppCallApiResponse>

    /**
     * Send a WebRTC SDP answer to complete the media negotiation.
     */
    @POST("{phoneNumberId}/calls")
    suspend fun sendSdpAnswer(
        @Path("phoneNumberId") phoneNumberId: String,
        @Body request: WhatsAppSdpAnswerRequest
    ): Response<WhatsAppCallApiResponse>

    /**
     * Send ICE candidates for WebRTC connectivity.
     */
    @POST("{phoneNumberId}/calls")
    suspend fun sendIceCandidates(
        @Path("phoneNumberId") phoneNumberId: String,
        @Body request: WhatsAppIceCandidateRequest
    ): Response<WhatsAppCallApiResponse>

    // ─── Call Permissions ───────────────────────────────────────────────────

    /**
     * Request call permission from a WhatsApp user.
     *
     * This sends a permission request message to the user. They must
     * approve it before you can make business-initiated calls to them.
     *
     * Limits (production):
     * - 1 permission request per user per day
     * - 2 permission requests per user per week
     */
    @POST("{phoneNumberId}/messages")
    suspend fun requestCallPermission(
        @Path("phoneNumberId") phoneNumberId: String,
        @Body request: WhatsAppCallPermissionMessageRequest
    ): Response<WhatsAppMessageResponse>

    // ─── Phone Number Settings ──────────────────────────────────────────────

    /**
     * Configure calling settings for a phone number.
     *
     * This enables/disables calling features and configures business hours,
     * callback requests, and inbound call control.
     */
    @POST("{phoneNumberId}")
    suspend fun configureCallSettings(
        @Path("phoneNumberId") phoneNumberId: String,
        @Body request: WhatsAppCallSettingsRequest
    ): Response<WhatsAppSettingsResponse>

    /**
     * Get current calling settings for a phone number.
     */
    @GET("{phoneNumberId}")
    suspend fun getCallSettings(
        @Path("phoneNumberId") phoneNumberId: String,
        @Query("fields") fields: String = "call_settings"
    ): Response<WhatsAppSettingsResponse>
}

// ═══════════════════════════════════════════════════════════════════════════════
// Request Models
// ═══════════════════════════════════════════════════════════════════════════════

@Serializable
data class WhatsAppInitiateCallRequest(
    @SerialName("messaging_product")
    val messagingProduct: String = "whatsapp",
    val to: String,
    val type: String = "call",
    val call: WhatsAppCallPayload
)

@Serializable
data class WhatsAppCallPayload(
    val action: String, // "create", "answer", "end"
    @SerialName("call_id")
    val callId: String? = null,
    val sdp: WhatsAppSdpPayload? = null,
    @SerialName("ice_candidates")
    val iceCandidates: List<WhatsAppIceCandidatePayload>? = null
)

@Serializable
data class WhatsAppSdpPayload(
    val type: String, // "offer" or "answer"
    val sdp: String
)

@Serializable
data class WhatsAppIceCandidatePayload(
    val candidate: String,
    @SerialName("sdp_mid")
    val sdpMid: String?,
    @SerialName("sdp_m_line_index")
    val sdpMLineIndex: Int?
)

@Serializable
data class WhatsAppAnswerCallRequest(
    @SerialName("messaging_product")
    val messagingProduct: String = "whatsapp",
    val type: String = "call",
    val call: WhatsAppCallPayload
)

@Serializable
data class WhatsAppEndCallRequest(
    @SerialName("messaging_product")
    val messagingProduct: String = "whatsapp",
    val type: String = "call",
    val call: WhatsAppCallPayload
)

@Serializable
data class WhatsAppSdpAnswerRequest(
    @SerialName("messaging_product")
    val messagingProduct: String = "whatsapp",
    val type: String = "call",
    val call: WhatsAppCallPayload
)

@Serializable
data class WhatsAppIceCandidateRequest(
    @SerialName("messaging_product")
    val messagingProduct: String = "whatsapp",
    val type: String = "call",
    val call: WhatsAppCallPayload
)

@Serializable
data class WhatsAppCallPermissionMessageRequest(
    @SerialName("messaging_product")
    val messagingProduct: String = "whatsapp",
    val to: String,
    val type: String = "template",
    val template: WhatsAppPermissionTemplate
)

@Serializable
data class WhatsAppPermissionTemplate(
    val name: String,
    val language: WhatsAppPermissionLanguage,
    val components: List<WhatsAppPermissionComponent> = emptyList()
)

@Serializable
data class WhatsAppPermissionLanguage(
    val code: String = "en_US"
)

@Serializable
data class WhatsAppPermissionComponent(
    val type: String,
    val parameters: List<WhatsAppPermissionParameter> = emptyList()
)

@Serializable
data class WhatsAppPermissionParameter(
    val type: String,
    val text: String? = null
)

@Serializable
data class WhatsAppCallSettingsRequest(
    @SerialName("call_settings")
    val callSettings: WhatsAppCallSettings
)

@Serializable
data class WhatsAppCallSettings(
    @SerialName("calling_enabled")
    val callingEnabled: Boolean = true,
    @SerialName("inbound_calling_enabled")
    val inboundCallingEnabled: Boolean = true,
    @SerialName("business_hours")
    val businessHours: WhatsAppBusinessHours? = null,
    @SerialName("callback_request_enabled")
    val callbackRequestEnabled: Boolean = false
)

@Serializable
data class WhatsAppBusinessHours(
    val timezone: String = "UTC",
    val hours: List<WhatsAppDayHours> = emptyList()
)

@Serializable
data class WhatsAppDayHours(
    val day: String,
    val open: String,
    val close: String
)

// ═══════════════════════════════════════════════════════════════════════════════
// Response Models
// ═══════════════════════════════════════════════════════════════════════════════

@Serializable
data class WhatsAppCallApiResponse(
    val success: Boolean? = null,
    @SerialName("call_id")
    val callId: String? = null,
    val sdp: WhatsAppSdpPayload? = null,
    @SerialName("ice_candidates")
    val iceCandidates: List<WhatsAppIceCandidatePayload>? = null,
    val error: WhatsAppApiError? = null
)

@Serializable
data class WhatsAppApiError(
    val message: String? = null,
    val type: String? = null,
    val code: Int? = null,
    @SerialName("error_subcode")
    val errorSubcode: Int? = null,
    @SerialName("fbtrace_id")
    val fbtraceId: String? = null
)

@Serializable
data class WhatsAppMessageResponse(
    @SerialName("messaging_product")
    val messagingProduct: String? = null,
    val contacts: List<WhatsAppContact>? = null,
    val messages: List<WhatsAppMessageId>? = null
)

@Serializable
data class WhatsAppContact(
    val input: String? = null,
    @SerialName("wa_id")
    val waId: String? = null
)

@Serializable
data class WhatsAppMessageId(
    val id: String
)

@Serializable
data class WhatsAppSettingsResponse(
    val id: String? = null,
    @SerialName("call_settings")
    val callSettings: WhatsAppCallSettings? = null
)
