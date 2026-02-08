package com.openclaw.callingnode.ui

import android.Manifest
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.core.content.ContextCompat
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.openclaw.callingnode.gateway.GatewayConnectionService
import com.openclaw.callingnode.service.vapi.VapiCallingService
import com.openclaw.callingnode.service.whatsapp.WhatsAppCallingService
import com.openclaw.callingnode.ui.screens.HomeScreen
import com.openclaw.callingnode.ui.screens.SettingsScreen
import com.openclaw.callingnode.ui.theme.OpenClawCallingNodeTheme
import com.openclaw.callingnode.viewmodel.MainViewModel
import dagger.hilt.android.AndroidEntryPoint
import timber.log.Timber

/**
 * Main Activity for the OpenClaw Calling Node.
 *
 * This activity:
 * 1. Requests necessary runtime permissions
 * 2. Starts the background services (Gateway, Vapi, WhatsApp)
 * 3. Hosts the Jetpack Compose navigation graph
 */
@AndroidEntryPoint
class MainActivity : ComponentActivity() {

    private val requiredPermissions = buildList {
        add(Manifest.permission.RECORD_AUDIO)
        add(Manifest.permission.CALL_PHONE)
        add(Manifest.permission.READ_PHONE_STATE)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            add(Manifest.permission.POST_NOTIFICATIONS)
            add(Manifest.permission.READ_PHONE_NUMBERS)
        }
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            add(Manifest.permission.BLUETOOTH_CONNECT)
        }
    }

    private val permissionLauncher = registerForActivityResult(
        ActivityResultContracts.RequestMultiplePermissions()
    ) { permissions ->
        val allGranted = permissions.values.all { it }
        if (allGranted) {
            Timber.i("All permissions granted")
            startServices()
        } else {
            Timber.w("Some permissions denied: ${permissions.filter { !it.value }.keys}")
            // Start services anyway; some features may be limited
            startServices()
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()

        Timber.i("MainActivity created")

        // Check and request permissions
        val missingPermissions = requiredPermissions.filter {
            ContextCompat.checkSelfPermission(this, it) != PackageManager.PERMISSION_GRANTED
        }

        if (missingPermissions.isEmpty()) {
            startServices()
        } else {
            permissionLauncher.launch(missingPermissions.toTypedArray())
        }

        setContent {
            OpenClawCallingNodeTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    OpenClawNavGraph()
                }
            }
        }
    }

    private fun startServices() {
        Timber.i("Starting background services")

        // Start Gateway Connection Service
        Intent(this, GatewayConnectionService::class.java).also {
            startForegroundService(it)
        }

        // Start Vapi Calling Service
        Intent(this, VapiCallingService::class.java).also {
            startForegroundService(it)
        }

        // Start WhatsApp Calling Service
        Intent(this, WhatsAppCallingService::class.java).also {
            startForegroundService(it)
        }
    }

    override fun onDestroy() {
        super.onDestroy()
        Timber.i("MainActivity destroyed")
    }
}

/**
 * Navigation graph for the application.
 */
@Composable
fun OpenClawNavGraph() {
    val navController = rememberNavController()
    val viewModel: MainViewModel = hiltViewModel()

    NavHost(navController = navController, startDestination = "home") {
        composable("home") {
            HomeScreen(
                viewModel = viewModel,
                onNavigateToSettings = { navController.navigate("settings") }
            )
        }
        composable("settings") {
            SettingsScreen(
                viewModel = viewModel,
                onNavigateBack = { navController.popBackStack() }
            )
        }
    }
}
