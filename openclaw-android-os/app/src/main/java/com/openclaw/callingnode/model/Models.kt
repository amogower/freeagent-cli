package com.openclaw.callingnode.model

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

// ═══════════════════════════════════════════════════════════════════════════════
// Call Models
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Represents the type of calling service to use.
 */
@Serializable
enum class CallProvider {
    @SerialName("vapi") VAPI,
    @SerialName("whatsapp") WHATSAPP,
    @SerialName("native") NATIVE
}

/**
 * Represents the current state of a call.
 */
@Serializable
enum class CallState {
    @SerialName("idle") IDLE,
    @SerialName("initiating") INITIATING,
    @SerialName("ringing") RINGING,
    @SerialName("active") ACTIVE,
    @SerialName("on_hold") ON_HOLD,
    @SerialName("ending") ENDING,
    @SerialName("ended") ENDED,
    @SerialName("failed") FAILED
}

/**
 * Represents the direction of a call.
 */
@Serializable
enum class CallDirection {
    @SerialName("inbound") INBOUND,
    @SerialName("outbound") OUTBOUND
}

/**
 * Core call data model used throughout the application.
 */
@Serializable
data class CallRecord(
    val id: String,
    val provider: CallProvider,
    val direction: CallDirection,
    val state: CallState,
    val phoneNumber: String,
    val displayName: String? = null,
    val assistantId: String? = null,
    val vapiCallId: String? = null,
    val whatsappCallId: String? = null,
    val startedAt: Long? = null,
    val endedAt: Long? = null,
    val durationSeconds: Int? = null,
    val transcript: String? = null,
    val metadata: Map<String, String> = emptyMap()
)

// ═══════════════════════════════════════════════════════════════════════════════
// Gateway Command Models
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Represents a command received from the OpenClaw Gateway.
 */
@Serializable
data class GatewayCommand(
    val id: String,
    val type: CommandType,
    val payload: CommandPayload,
    val timestamp: Long = System.currentTimeMillis()
)

@Serializable
enum class CommandType {
    @SerialName("make_call") MAKE_CALL,
    @SerialName("end_call") END_CALL,
    @SerialName("transfer_call") TRANSFER_CALL,
    @SerialName("send_dtmf") SEND_DTMF,
    @SerialName("get_call_status") GET_CALL_STATUS,
    @SerialName("list_active_calls") LIST_ACTIVE_CALLS,
    @SerialName("configure") CONFIGURE,
    @SerialName("ping") PING
}

@Serializable
data class CommandPayload(
    val phoneNumber: String? = null,
    val provider: CallProvider? = null,
    val assistantId: String? = null,
    val systemPrompt: String? = null,
    val firstMessage: String? = null,
    val callId: String? = null,
    val dtmfDigits: String? = null,
    val transferTo: String? = null,
    val config: Map<String, String>? = null
)

// ═══════════════════════════════════════════════════════════════════════════════
// Gateway Response Models
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Represents a response sent back to the OpenClaw Gateway.
 */
@Serializable
data class GatewayResponse(
    val id: String,
    val commandId: String,
    val type: ResponseType,
    val payload: ResponsePayload,
    val timestamp: Long = System.currentTimeMillis()
)

@Serializable
enum class ResponseType {
    @SerialName("call_initiated") CALL_INITIATED,
    @SerialName("call_status") CALL_STATUS,
    @SerialName("call_ended") CALL_ENDED,
    @SerialName("call_failed") CALL_FAILED,
    @SerialName("call_list") CALL_LIST,
    @SerialName("config_updated") CONFIG_UPDATED,
    @SerialName("pong") PONG,
    @SerialName("error") ERROR
}

@Serializable
data class ResponsePayload(
    val callId: String? = null,
    val callState: CallState? = null,
    val callRecord: CallRecord? = null,
    val activeCalls: List<CallRecord>? = null,
    val message: String? = null,
    val error: String? = null
)

// ═══════════════════════════════════════════════════════════════════════════════
// Vapi API Models
// ═══════════════════════════════════════════════════════════════════════════════

@Serializable
data class VapiCreateCallRequest(
    val name: String? = null,
    @SerialName("assistantId")
    val assistantId: String? = null,
    val assistant: VapiAssistantOverride? = null,
    @SerialName("phoneNumberId")
    val phoneNumberId: String? = null,
    val customer: VapiCustomer? = null
)

@Serializable
data class VapiAssistantOverride(
    val model: VapiModel? = null,
    @SerialName("firstMessage")
    val firstMessage: String? = null,
    val voice: VapiVoice? = null
)

@Serializable
data class VapiModel(
    val provider: String = "openai",
    val model: String = "gpt-4o",
    val messages: List<VapiMessage> = emptyList()
)

@Serializable
data class VapiMessage(
    val role: String,
    val content: String
)

@Serializable
data class VapiVoice(
    val provider: String = "11labs",
    @SerialName("voiceId")
    val voiceId: String = "rachel"
)

@Serializable
data class VapiCustomer(
    val number: String,
    val name: String? = null
)

@Serializable
data class VapiCallResponse(
    val id: String,
    val status: String? = null,
    val type: String? = null,
    @SerialName("phoneNumberId")
    val phoneNumberId: String? = null,
    @SerialName("assistantId")
    val assistantId: String? = null,
    @SerialName("createdAt")
    val createdAt: String? = null,
    @SerialName("updatedAt")
    val updatedAt: String? = null,
    @SerialName("startedAt")
    val startedAt: String? = null,
    @SerialName("endedAt")
    val endedAt: String? = null,
    val transcript: String? = null,
    val summary: String? = null,
    val cost: Double? = null
)

@Serializable
data class VapiAssistantResponse(
    val id: String,
    val name: String? = null,
    @SerialName("firstMessage")
    val firstMessage: String? = null,
    val model: VapiModel? = null,
    val voice: VapiVoice? = null,
    @SerialName("createdAt")
    val createdAt: String? = null
)

// ═══════════════════════════════════════════════════════════════════════════════
// WhatsApp Business Calling API Models
// ═══════════════════════════════════════════════════════════════════════════════

@Serializable
data class WhatsAppCallRequest(
    @SerialName("messaging_product")
    val messagingProduct: String = "whatsapp",
    val to: String,
    val type: String = "voice",
    @SerialName("call_type")
    val callType: String = "voice_call"
)

@Serializable
data class WhatsAppCallPermissionRequest(
    @SerialName("messaging_product")
    val messagingProduct: String = "whatsapp",
    val to: String,
    val type: String = "template",
    val template: WhatsAppCallTemplate
)

@Serializable
data class WhatsAppCallTemplate(
    val name: String = "call_permission_request",
    val language: WhatsAppLanguage = WhatsAppLanguage(),
    val components: List<WhatsAppTemplateComponent> = emptyList()
)

@Serializable
data class WhatsAppLanguage(
    val code: String = "en_US"
)

@Serializable
data class WhatsAppTemplateComponent(
    val type: String,
    val parameters: List<WhatsAppTemplateParameter> = emptyList()
)

@Serializable
data class WhatsAppTemplateParameter(
    val type: String,
    val text: String? = null
)

@Serializable
data class WhatsAppCallResponse(
    val id: String? = null,
    @SerialName("call_id")
    val callId: String? = null,
    val status: String? = null,
    @SerialName("messaging_product")
    val messagingProduct: String? = null
)

@Serializable
data class WhatsAppWebhookPayload(
    val `object`: String? = null,
    val entry: List<WhatsAppWebhookEntry> = emptyList()
)

@Serializable
data class WhatsAppWebhookEntry(
    val id: String,
    val changes: List<WhatsAppWebhookChange> = emptyList()
)

@Serializable
data class WhatsAppWebhookChange(
    val field: String,
    val value: WhatsAppWebhookValue
)

@Serializable
data class WhatsAppWebhookValue(
    @SerialName("messaging_product")
    val messagingProduct: String? = null,
    val metadata: WhatsAppMetadata? = null,
    val calls: List<WhatsAppWebhookCall>? = null,
    val statuses: List<WhatsAppCallStatus>? = null
)

@Serializable
data class WhatsAppMetadata(
    @SerialName("display_phone_number")
    val displayPhoneNumber: String? = null,
    @SerialName("phone_number_id")
    val phoneNumberId: String? = null
)

@Serializable
data class WhatsAppWebhookCall(
    val id: String,
    val from: String,
    val timestamp: String,
    val type: String? = null
)

@Serializable
data class WhatsAppCallStatus(
    val id: String,
    val status: String,
    val timestamp: String,
    @SerialName("recipient_id")
    val recipientId: String? = null
)

// ═══════════════════════════════════════════════════════════════════════════════
// WebRTC Signaling Models (for WhatsApp Calling)
// ═══════════════════════════════════════════════════════════════════════════════

@Serializable
data class WebRTCOffer(
    val sdp: String,
    val type: String = "offer"
)

@Serializable
data class WebRTCAnswer(
    val sdp: String,
    val type: String = "answer"
)

@Serializable
data class WebRTCIceCandidate(
    val candidate: String,
    @SerialName("sdpMid")
    val sdpMid: String?,
    @SerialName("sdpMLineIndex")
    val sdpMLineIndex: Int?
)

// ═══════════════════════════════════════════════════════════════════════════════
// Configuration Models
// ═══════════════════════════════════════════════════════════════════════════════

@Serializable
data class NodeConfiguration(
    val gatewayUrl: String = "ws://localhost:18789",
    val gatewayToken: String = "",
    val vapiApiKey: String = "",
    val vapiDefaultAssistantId: String? = null,
    val vapiPhoneNumberId: String? = null,
    val whatsappAccessToken: String = "",
    val whatsappPhoneNumberId: String = "",
    val whatsappBusinessAccountId: String = "",
    val defaultProvider: CallProvider = CallProvider.VAPI,
    val autoReconnect: Boolean = true,
    val reconnectDelayMs: Long = 5000,
    val maxReconnectAttempts: Int = 10,
    val enableCallLogging: Boolean = true,
    val enableTranscription: Boolean = true
)
