package com.openclaw.callingnode.ui.screens

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.text.input.VisualTransformation
import androidx.compose.ui.unit.dp
import com.openclaw.callingnode.model.NodeConfiguration
import com.openclaw.callingnode.viewmodel.MainViewModel

/**
 * Settings screen for configuring the OpenClaw Calling Node.
 *
 * Allows users to configure:
 * - OpenClaw Gateway connection
 * - Vapi AI credentials and assistant settings
 * - WhatsApp Business API credentials
 * - General preferences
 */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SettingsScreen(
    viewModel: MainViewModel,
    onNavigateBack: () -> Unit
) {
    val uiState by viewModel.uiState.collectAsState()
    val config = uiState.config

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("Settings") },
                navigationIcon = {
                    IconButton(onClick = onNavigateBack) {
                        Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "Back")
                    }
                }
            )
        }
    ) { padding ->
        LazyColumn(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .padding(horizontal = 16.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            // ── Gateway Settings ──
            item {
                SettingsSectionHeader(
                    icon = Icons.Filled.Cloud,
                    title = "OpenClaw Gateway"
                )
            }

            item {
                SettingsTextField(
                    label = "Gateway URL",
                    value = config.gatewayUrl,
                    placeholder = "ws://localhost:18789",
                    onValueChange = { viewModel.updateConfig("gateway_url", it) }
                )
            }

            item {
                SettingsTextField(
                    label = "Gateway Token",
                    value = config.gatewayToken,
                    placeholder = "Your gateway authentication token",
                    onValueChange = { viewModel.updateConfig("gateway_token", it) },
                    isSecret = true
                )
            }

            // ── Vapi Settings ──
            item {
                Spacer(modifier = Modifier.height(16.dp))
                SettingsSectionHeader(
                    icon = Icons.Filled.Phone,
                    title = "Vapi AI"
                )
            }

            item {
                SettingsTextField(
                    label = "API Key",
                    value = config.vapiApiKey,
                    placeholder = "vapi_xxxxxxxxxxxxxxxx",
                    onValueChange = { viewModel.updateConfig("vapi_api_key", it) },
                    isSecret = true
                )
            }

            item {
                SettingsTextField(
                    label = "Default Assistant ID",
                    value = config.vapiDefaultAssistantId ?: "",
                    placeholder = "Optional: Vapi assistant ID",
                    onValueChange = { viewModel.updateConfig("vapi_assistant_id", it) }
                )
            }

            item {
                SettingsTextField(
                    label = "Phone Number ID",
                    value = config.vapiPhoneNumberId ?: "",
                    placeholder = "Optional: Vapi phone number ID",
                    onValueChange = { viewModel.updateConfig("vapi_phone_number_id", it) }
                )
            }

            // ── WhatsApp Settings ──
            item {
                Spacer(modifier = Modifier.height(16.dp))
                SettingsSectionHeader(
                    icon = Icons.Filled.Chat,
                    title = "WhatsApp Business"
                )
            }

            item {
                SettingsTextField(
                    label = "Access Token",
                    value = config.whatsappAccessToken,
                    placeholder = "WhatsApp Cloud API access token",
                    onValueChange = { viewModel.updateConfig("whatsapp_access_token", it) },
                    isSecret = true
                )
            }

            item {
                SettingsTextField(
                    label = "Phone Number ID",
                    value = config.whatsappPhoneNumberId,
                    placeholder = "WhatsApp Business phone number ID",
                    onValueChange = { viewModel.updateConfig("whatsapp_phone_number_id", it) }
                )
            }

            item {
                SettingsTextField(
                    label = "Business Account ID",
                    value = config.whatsappBusinessAccountId,
                    placeholder = "WhatsApp Business Account ID",
                    onValueChange = { viewModel.updateConfig("whatsapp_business_account_id", it) }
                )
            }

            // ── Info ──
            item {
                Spacer(modifier = Modifier.height(16.dp))
                Card(
                    colors = CardDefaults.cardColors(
                        containerColor = MaterialTheme.colorScheme.surfaceVariant
                    ),
                    modifier = Modifier.fillMaxWidth()
                ) {
                    Column(modifier = Modifier.padding(16.dp)) {
                        Text(
                            text = "About",
                            style = MaterialTheme.typography.titleSmall,
                            fontWeight = FontWeight.Bold
                        )
                        Spacer(modifier = Modifier.height(8.dp))
                        Text(
                            text = "OpenClaw Calling Node v2026.2.15-beta.1",
                            style = MaterialTheme.typography.bodyMedium
                        )
                        Text(
                            text = "Custom Android OS layer for programmatic phone calling with AI agents via Vapi and WhatsApp Business Calling API.",
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant
                        )
                    }
                }
            }

            item { Spacer(modifier = Modifier.height(32.dp)) }
        }
    }
}

@Composable
fun SettingsSectionHeader(
    icon: androidx.compose.ui.graphics.vector.ImageVector,
    title: String
) {
    Row(
        modifier = Modifier.padding(vertical = 8.dp),
        horizontalArrangement = Arrangement.spacedBy(8.dp)
    ) {
        Icon(
            imageVector = icon,
            contentDescription = null,
            tint = MaterialTheme.colorScheme.primary
        )
        Text(
            text = title,
            style = MaterialTheme.typography.titleMedium,
            fontWeight = FontWeight.Bold,
            color = MaterialTheme.colorScheme.primary
        )
    }
}

@Composable
fun SettingsTextField(
    label: String,
    value: String,
    placeholder: String,
    onValueChange: (String) -> Unit,
    isSecret: Boolean = false
) {
    var showSecret by remember { mutableStateOf(false) }

    OutlinedTextField(
        value = value,
        onValueChange = onValueChange,
        label = { Text(label) },
        placeholder = { Text(placeholder) },
        modifier = Modifier.fillMaxWidth(),
        singleLine = true,
        visualTransformation = if (isSecret && !showSecret) {
            PasswordVisualTransformation()
        } else {
            VisualTransformation.None
        },
        trailingIcon = {
            if (isSecret) {
                IconButton(onClick = { showSecret = !showSecret }) {
                    Icon(
                        imageVector = if (showSecret) Icons.Filled.VisibilityOff else Icons.Filled.Visibility,
                        contentDescription = if (showSecret) "Hide" else "Show"
                    )
                }
            }
        }
    )
}
