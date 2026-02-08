package com.openclaw.callingnode.ui.screens

import androidx.compose.animation.*
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material.icons.outlined.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.openclaw.callingnode.gateway.GatewayConnectionService
import com.openclaw.callingnode.model.*
import com.openclaw.callingnode.viewmodel.MainViewModel

/**
 * Main home screen for the OpenClaw Calling Node.
 *
 * Displays:
 * - Gateway connection status
 * - Active calls with controls
 * - Dial pad for initiating new calls
 * - Recent call history
 */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun HomeScreen(
    viewModel: MainViewModel,
    onNavigateToSettings: () -> Unit
) {
    val uiState by viewModel.uiState.collectAsState()
    val dialState by viewModel.dialState.collectAsState()

    Scaffold(
        topBar = {
            TopAppBar(
                title = {
                    Row(verticalAlignment = Alignment.CenterVertically) {
                        Text(
                            text = "OpenClaw",
                            style = MaterialTheme.typography.titleLarge,
                            fontWeight = FontWeight.Bold
                        )
                        Spacer(modifier = Modifier.width(8.dp))
                        Text(
                            text = "Calling Node",
                            style = MaterialTheme.typography.titleMedium,
                            color = MaterialTheme.colorScheme.onSurfaceVariant
                        )
                    }
                },
                actions = {
                    // Gateway status indicator
                    GatewayStatusChip(uiState.gatewayState)
                    IconButton(onClick = onNavigateToSettings) {
                        Icon(Icons.Outlined.Settings, contentDescription = "Settings")
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
            verticalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            // ── Error Banner ──
            item {
                AnimatedVisibility(visible = uiState.lastError != null) {
                    Card(
                        colors = CardDefaults.cardColors(
                            containerColor = MaterialTheme.colorScheme.errorContainer
                        ),
                        modifier = Modifier.fillMaxWidth()
                    ) {
                        Row(
                            modifier = Modifier.padding(16.dp),
                            verticalAlignment = Alignment.CenterVertically
                        ) {
                            Icon(
                                Icons.Filled.Error,
                                contentDescription = null,
                                tint = MaterialTheme.colorScheme.error
                            )
                            Spacer(modifier = Modifier.width(12.dp))
                            Text(
                                text = uiState.lastError ?: "",
                                style = MaterialTheme.typography.bodyMedium,
                                modifier = Modifier.weight(1f)
                            )
                            IconButton(onClick = { viewModel.clearError() }) {
                                Icon(Icons.Filled.Close, contentDescription = "Dismiss")
                            }
                        }
                    }
                }
            }

            // ── Active Calls ──
            if (uiState.activeCalls.isNotEmpty()) {
                item {
                    Text(
                        text = "Active Calls",
                        style = MaterialTheme.typography.titleMedium,
                        fontWeight = FontWeight.Bold
                    )
                }
                items(uiState.activeCalls) { call ->
                    ActiveCallCard(call = call, onEndCall = { viewModel.endCall(call.id) })
                }
            }

            // ── Dial Pad ──
            item {
                DialPadCard(
                    dialState = dialState,
                    onPhoneNumberChange = viewModel::updatePhoneNumber,
                    onProviderChange = viewModel::updateSelectedProvider,
                    onSystemPromptChange = viewModel::updateSystemPrompt,
                    onFirstMessageChange = viewModel::updateFirstMessage,
                    onMakeCall = viewModel::makeCall,
                    onRequestPermission = viewModel::requestWhatsAppPermission
                )
            }

            // ── Call History ──
            if (uiState.callHistory.isNotEmpty()) {
                item {
                    Text(
                        text = "Recent Calls",
                        style = MaterialTheme.typography.titleMedium,
                        fontWeight = FontWeight.Bold
                    )
                }
                items(uiState.callHistory.take(20)) { call ->
                    CallHistoryItem(call)
                }
            }

            // Bottom spacer
            item { Spacer(modifier = Modifier.height(32.dp)) }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Sub-components
// ═══════════════════════════════════════════════════════════════════════════════

@Composable
fun GatewayStatusChip(state: GatewayConnectionService.ConnectionState) {
    val (color, label) = when (state) {
        GatewayConnectionService.ConnectionState.CONNECTED -> Color(0xFF4CAF50) to "Connected"
        GatewayConnectionService.ConnectionState.CONNECTING -> Color(0xFFFFC107) to "Connecting"
        GatewayConnectionService.ConnectionState.RECONNECTING -> Color(0xFFFF9800) to "Reconnecting"
        GatewayConnectionService.ConnectionState.FAILED -> Color(0xFFF44336) to "Failed"
        GatewayConnectionService.ConnectionState.DISCONNECTED -> Color(0xFF9E9E9E) to "Offline"
    }

    SuggestionChip(
        onClick = {},
        label = { Text(label, style = MaterialTheme.typography.labelSmall) },
        icon = {
            Box(
                modifier = Modifier
                    .size(8.dp)
                    .clip(CircleShape)
                    .background(color)
            )
        }
    )
}

@Composable
fun ActiveCallCard(call: CallRecord, onEndCall: () -> Unit) {
    Card(
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.primaryContainer
        ),
        modifier = Modifier.fillMaxWidth()
    ) {
        Row(
            modifier = Modifier
                .padding(16.dp)
                .fillMaxWidth(),
            verticalAlignment = Alignment.CenterVertically
        ) {
            // Provider icon
            Icon(
                imageVector = when (call.provider) {
                    CallProvider.VAPI -> Icons.Filled.Phone
                    CallProvider.WHATSAPP -> Icons.Filled.Chat
                    CallProvider.NATIVE -> Icons.Filled.PhoneAndroid
                },
                contentDescription = null,
                tint = MaterialTheme.colorScheme.onPrimaryContainer,
                modifier = Modifier.size(32.dp)
            )

            Spacer(modifier = Modifier.width(16.dp))

            Column(modifier = Modifier.weight(1f)) {
                Text(
                    text = call.displayName ?: call.phoneNumber,
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = FontWeight.Bold
                )
                Text(
                    text = "${call.provider.name} | ${call.state.name}",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onPrimaryContainer.copy(alpha = 0.7f)
                )
            }

            // End call button
            FilledIconButton(
                onClick = onEndCall,
                colors = IconButtonDefaults.filledIconButtonColors(
                    containerColor = MaterialTheme.colorScheme.error
                )
            ) {
                Icon(Icons.Filled.CallEnd, contentDescription = "End Call")
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DialPadCard(
    dialState: MainViewModel.DialState,
    onPhoneNumberChange: (String) -> Unit,
    onProviderChange: (CallProvider) -> Unit,
    onSystemPromptChange: (String) -> Unit,
    onFirstMessageChange: (String) -> Unit,
    onMakeCall: () -> Unit,
    onRequestPermission: () -> Unit
) {
    var showAdvanced by remember { mutableStateOf(false) }

    Card(
        modifier = Modifier.fillMaxWidth()
    ) {
        Column(
            modifier = Modifier.padding(20.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            Text(
                text = "Make a Call",
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.Bold
            )

            // Phone number input
            OutlinedTextField(
                value = dialState.phoneNumber,
                onValueChange = onPhoneNumberChange,
                label = { Text("Phone Number") },
                placeholder = { Text("+1234567890") },
                leadingIcon = { Icon(Icons.Filled.Phone, contentDescription = null) },
                modifier = Modifier.fillMaxWidth(),
                singleLine = true
            )

            // Provider selector
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                CallProvider.entries.forEach { provider ->
                    FilterChip(
                        selected = dialState.selectedProvider == provider,
                        onClick = { onProviderChange(provider) },
                        label = {
                            Text(
                                when (provider) {
                                    CallProvider.VAPI -> "Vapi AI"
                                    CallProvider.WHATSAPP -> "WhatsApp"
                                    CallProvider.NATIVE -> "Native"
                                }
                            )
                        },
                        leadingIcon = {
                            if (dialState.selectedProvider == provider) {
                                Icon(
                                    Icons.Filled.Check,
                                    contentDescription = null,
                                    modifier = Modifier.size(18.dp)
                                )
                            }
                        }
                    )
                }
            }

            // Advanced options (Vapi-specific)
            AnimatedVisibility(visible = dialState.selectedProvider == CallProvider.VAPI) {
                Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
                    TextButton(onClick = { showAdvanced = !showAdvanced }) {
                        Icon(
                            if (showAdvanced) Icons.Filled.ExpandLess else Icons.Filled.ExpandMore,
                            contentDescription = null
                        )
                        Spacer(modifier = Modifier.width(4.dp))
                        Text("Agent Configuration")
                    }

                    AnimatedVisibility(visible = showAdvanced) {
                        Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
                            OutlinedTextField(
                                value = dialState.systemPrompt,
                                onValueChange = onSystemPromptChange,
                                label = { Text("System Prompt (optional)") },
                                placeholder = { Text("You are a helpful assistant...") },
                                modifier = Modifier.fillMaxWidth(),
                                minLines = 2,
                                maxLines = 4
                            )

                            OutlinedTextField(
                                value = dialState.firstMessage,
                                onValueChange = onFirstMessageChange,
                                label = { Text("First Message (optional)") },
                                placeholder = { Text("Hello! How can I help you?") },
                                modifier = Modifier.fillMaxWidth(),
                                singleLine = true
                            )
                        }
                    }
                }
            }

            // WhatsApp permission request
            AnimatedVisibility(visible = dialState.selectedProvider == CallProvider.WHATSAPP) {
                OutlinedButton(
                    onClick = onRequestPermission,
                    modifier = Modifier.fillMaxWidth()
                ) {
                    Icon(Icons.Outlined.Security, contentDescription = null)
                    Spacer(modifier = Modifier.width(8.dp))
                    Text("Request Call Permission First")
                }
            }

            // Call button
            Button(
                onClick = onMakeCall,
                modifier = Modifier
                    .fillMaxWidth()
                    .height(56.dp),
                enabled = dialState.phoneNumber.isNotBlank() && !dialState.isDialing,
                shape = RoundedCornerShape(16.dp)
            ) {
                if (dialState.isDialing) {
                    CircularProgressIndicator(
                        modifier = Modifier.size(24.dp),
                        color = MaterialTheme.colorScheme.onPrimary,
                        strokeWidth = 2.dp
                    )
                    Spacer(modifier = Modifier.width(12.dp))
                    Text("Calling...")
                } else {
                    Icon(Icons.Filled.Call, contentDescription = null)
                    Spacer(modifier = Modifier.width(12.dp))
                    Text(
                        text = when (dialState.selectedProvider) {
                            CallProvider.VAPI -> "Call with Vapi AI Agent"
                            CallProvider.WHATSAPP -> "Call via WhatsApp"
                            CallProvider.NATIVE -> "Native Call"
                        },
                        style = MaterialTheme.typography.titleMedium
                    )
                }
            }
        }
    }
}

@Composable
fun CallHistoryItem(call: CallRecord) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 8.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        // Direction icon
        Icon(
            imageVector = when (call.direction) {
                CallDirection.OUTBOUND -> Icons.Filled.CallMade
                CallDirection.INBOUND -> Icons.Filled.CallReceived
            },
            contentDescription = null,
            tint = when (call.state) {
                CallState.ENDED -> MaterialTheme.colorScheme.primary
                CallState.FAILED -> MaterialTheme.colorScheme.error
                else -> MaterialTheme.colorScheme.onSurface
            },
            modifier = Modifier.size(20.dp)
        )

        Spacer(modifier = Modifier.width(12.dp))

        Column(modifier = Modifier.weight(1f)) {
            Text(
                text = call.displayName ?: call.phoneNumber,
                style = MaterialTheme.typography.bodyLarge,
                maxLines = 1,
                overflow = TextOverflow.Ellipsis
            )
            Text(
                text = "${call.provider.name} | ${call.state.name}",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }

        // Duration
        call.durationSeconds?.let { duration ->
            Text(
                text = formatDuration(duration),
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }
    }
}

private fun formatDuration(seconds: Int): String {
    val mins = seconds / 60
    val secs = seconds % 60
    return "${mins}:${secs.toString().padStart(2, '0')}"
}
