package com.openclaw.callingnode.config

import android.content.Context
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.booleanPreferencesKey
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.longPreferencesKey
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.datastore.preferences.preferencesDataStore
import com.openclaw.callingnode.BuildConfig
import com.openclaw.callingnode.model.CallProvider
import com.openclaw.callingnode.model.NodeConfiguration
import dagger.hilt.android.qualifiers.ApplicationContext
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map
import timber.log.Timber
import javax.inject.Inject
import javax.inject.Singleton

private val Context.dataStore: DataStore<Preferences> by preferencesDataStore(
    name = "openclaw_config"
)

/**
 * Manages all configuration for the OpenClaw Calling Node.
 *
 * Configuration sources (in order of precedence):
 * 1. DataStore (user-modified settings)
 * 2. BuildConfig (compile-time values from gradle.properties / local.properties)
 * 3. Defaults
 */
@Singleton
class ConfigManager @Inject constructor(
    @ApplicationContext private val context: Context
) {
    private object Keys {
        val GATEWAY_URL = stringPreferencesKey("gateway_url")
        val GATEWAY_TOKEN = stringPreferencesKey("gateway_token")
        val VAPI_API_KEY = stringPreferencesKey("vapi_api_key")
        val VAPI_DEFAULT_ASSISTANT_ID = stringPreferencesKey("vapi_default_assistant_id")
        val VAPI_PHONE_NUMBER_ID = stringPreferencesKey("vapi_phone_number_id")
        val WHATSAPP_ACCESS_TOKEN = stringPreferencesKey("whatsapp_access_token")
        val WHATSAPP_PHONE_NUMBER_ID = stringPreferencesKey("whatsapp_phone_number_id")
        val WHATSAPP_BUSINESS_ACCOUNT_ID = stringPreferencesKey("whatsapp_business_account_id")
        val DEFAULT_PROVIDER = stringPreferencesKey("default_provider")
        val AUTO_RECONNECT = booleanPreferencesKey("auto_reconnect")
        val RECONNECT_DELAY_MS = longPreferencesKey("reconnect_delay_ms")
        val ENABLE_CALL_LOGGING = booleanPreferencesKey("enable_call_logging")
        val ENABLE_TRANSCRIPTION = booleanPreferencesKey("enable_transcription")
    }

    /**
     * Observe the current configuration as a Flow.
     */
    val configuration: Flow<NodeConfiguration> = context.dataStore.data.map { prefs ->
        NodeConfiguration(
            gatewayUrl = prefs[Keys.GATEWAY_URL]
                ?: BuildConfig.OPENCLAW_GATEWAY_URL.ifEmpty { "ws://localhost:18789" },
            gatewayToken = prefs[Keys.GATEWAY_TOKEN]
                ?: BuildConfig.OPENCLAW_GATEWAY_TOKEN,
            vapiApiKey = prefs[Keys.VAPI_API_KEY]
                ?: BuildConfig.VAPI_API_KEY,
            vapiDefaultAssistantId = prefs[Keys.VAPI_DEFAULT_ASSISTANT_ID],
            vapiPhoneNumberId = prefs[Keys.VAPI_PHONE_NUMBER_ID],
            whatsappAccessToken = prefs[Keys.WHATSAPP_ACCESS_TOKEN]
                ?: BuildConfig.WHATSAPP_ACCESS_TOKEN,
            whatsappPhoneNumberId = prefs[Keys.WHATSAPP_PHONE_NUMBER_ID]
                ?: BuildConfig.WHATSAPP_PHONE_NUMBER_ID,
            whatsappBusinessAccountId = prefs[Keys.WHATSAPP_BUSINESS_ACCOUNT_ID]
                ?: BuildConfig.WHATSAPP_BUSINESS_ACCOUNT_ID,
            defaultProvider = prefs[Keys.DEFAULT_PROVIDER]?.let {
                try { CallProvider.valueOf(it.uppercase()) } catch (_: Exception) { CallProvider.VAPI }
            } ?: CallProvider.VAPI,
            autoReconnect = prefs[Keys.AUTO_RECONNECT] ?: true,
            reconnectDelayMs = prefs[Keys.RECONNECT_DELAY_MS] ?: 5000L,
            enableCallLogging = prefs[Keys.ENABLE_CALL_LOGGING] ?: true,
            enableTranscription = prefs[Keys.ENABLE_TRANSCRIPTION] ?: true
        )
    }

    /**
     * Update a specific configuration value.
     */
    suspend fun updateGatewayUrl(url: String) {
        context.dataStore.edit { it[Keys.GATEWAY_URL] = url }
        Timber.i("Gateway URL updated: $url")
    }

    suspend fun updateGatewayToken(token: String) {
        context.dataStore.edit { it[Keys.GATEWAY_TOKEN] = token }
        Timber.i("Gateway token updated")
    }

    suspend fun updateVapiApiKey(key: String) {
        context.dataStore.edit { it[Keys.VAPI_API_KEY] = key }
        Timber.i("Vapi API key updated")
    }

    suspend fun updateVapiDefaultAssistantId(id: String) {
        context.dataStore.edit { it[Keys.VAPI_DEFAULT_ASSISTANT_ID] = id }
        Timber.i("Vapi default assistant ID updated: $id")
    }

    suspend fun updateVapiPhoneNumberId(id: String) {
        context.dataStore.edit { it[Keys.VAPI_PHONE_NUMBER_ID] = id }
        Timber.i("Vapi phone number ID updated: $id")
    }

    suspend fun updateWhatsAppAccessToken(token: String) {
        context.dataStore.edit { it[Keys.WHATSAPP_ACCESS_TOKEN] = token }
        Timber.i("WhatsApp access token updated")
    }

    suspend fun updateWhatsAppPhoneNumberId(id: String) {
        context.dataStore.edit { it[Keys.WHATSAPP_PHONE_NUMBER_ID] = id }
        Timber.i("WhatsApp phone number ID updated: $id")
    }

    suspend fun updateWhatsAppBusinessAccountId(id: String) {
        context.dataStore.edit { it[Keys.WHATSAPP_BUSINESS_ACCOUNT_ID] = id }
        Timber.i("WhatsApp business account ID updated: $id")
    }

    suspend fun updateDefaultProvider(provider: CallProvider) {
        context.dataStore.edit { it[Keys.DEFAULT_PROVIDER] = provider.name.lowercase() }
        Timber.i("Default provider updated: $provider")
    }

    suspend fun updateAutoReconnect(enabled: Boolean) {
        context.dataStore.edit { it[Keys.AUTO_RECONNECT] = enabled }
    }

    suspend fun updateCallLogging(enabled: Boolean) {
        context.dataStore.edit { it[Keys.ENABLE_CALL_LOGGING] = enabled }
    }

    suspend fun updateTranscription(enabled: Boolean) {
        context.dataStore.edit { it[Keys.ENABLE_TRANSCRIPTION] = enabled }
    }

    /**
     * Reset all configuration to defaults.
     */
    suspend fun resetToDefaults() {
        context.dataStore.edit { it.clear() }
        Timber.i("Configuration reset to defaults")
    }
}
