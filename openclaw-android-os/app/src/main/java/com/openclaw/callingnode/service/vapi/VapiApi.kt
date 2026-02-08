package com.openclaw.callingnode.service.vapi

import com.openclaw.callingnode.model.*
import retrofit2.Response
import retrofit2.http.*

/**
 * Retrofit interface for the Vapi AI REST API.
 *
 * Vapi provides the telephony infrastructure for making and receiving
 * traditional phone calls with AI voice agents. This interface covers
 * the core endpoints needed for programmatic call management.
 *
 * @see <a href="https://docs.vapi.ai/api-reference">Vapi API Reference</a>
 */
interface VapiApi {

    // ─── Calls ──────────────────────────────────────────────────────────────

    /**
     * Create an outbound phone call.
     *
     * This initiates a call from a Vapi phone number to the specified
     * customer number. The call will be handled by the specified assistant
     * or an inline assistant configuration.
     */
    @POST("call")
    suspend fun createCall(
        @Body request: VapiCreateCallRequest
    ): Response<VapiCallResponse>

    /**
     * Get the details and status of a specific call.
     */
    @GET("call/{callId}")
    suspend fun getCall(
        @Path("callId") callId: String
    ): Response<VapiCallResponse>

    /**
     * List all calls with optional filtering.
     */
    @GET("call")
    suspend fun listCalls(
        @Query("limit") limit: Int = 20,
        @Query("createdAtGt") createdAfter: String? = null,
        @Query("createdAtLt") createdBefore: String? = null
    ): Response<List<VapiCallResponse>>

    /**
     * End an active call.
     */
    @DELETE("call/{callId}")
    suspend fun endCall(
        @Path("callId") callId: String
    ): Response<VapiCallResponse>

    // ─── Assistants ─────────────────────────────────────────────────────────

    /**
     * Create a new voice assistant.
     */
    @POST("assistant")
    suspend fun createAssistant(
        @Body request: VapiAssistantOverride
    ): Response<VapiAssistantResponse>

    /**
     * Get an existing assistant by ID.
     */
    @GET("assistant/{assistantId}")
    suspend fun getAssistant(
        @Path("assistantId") assistantId: String
    ): Response<VapiAssistantResponse>

    /**
     * List all assistants.
     */
    @GET("assistant")
    suspend fun listAssistants(
        @Query("limit") limit: Int = 20
    ): Response<List<VapiAssistantResponse>>

    /**
     * Delete an assistant.
     */
    @DELETE("assistant/{assistantId}")
    suspend fun deleteAssistant(
        @Path("assistantId") assistantId: String
    ): Response<Unit>

    // ─── Phone Numbers ──────────────────────────────────────────────────────

    /**
     * List all phone numbers associated with the account.
     */
    @GET("phone-number")
    suspend fun listPhoneNumbers(
        @Query("limit") limit: Int = 20
    ): Response<List<VapiPhoneNumber>>
}

/**
 * Vapi phone number model.
 */
@kotlinx.serialization.Serializable
data class VapiPhoneNumber(
    val id: String,
    val number: String? = null,
    val provider: String? = null,
    @kotlinx.serialization.SerialName("assistantId")
    val assistantId: String? = null,
    @kotlinx.serialization.SerialName("createdAt")
    val createdAt: String? = null
)
