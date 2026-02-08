package com.openclaw.callingnode.receiver

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import com.openclaw.callingnode.gateway.GatewayConnectionService
import com.openclaw.callingnode.service.vapi.VapiCallingService
import com.openclaw.callingnode.service.whatsapp.WhatsAppCallingService
import timber.log.Timber

/**
 * Broadcast receiver that starts the OpenClaw Calling Node services
 * automatically when the device boots up.
 *
 * This ensures the Gateway connection is re-established and the
 * calling services are ready to receive commands without requiring
 * the user to manually open the app.
 */
class BootReceiver : BroadcastReceiver() {

    override fun onReceive(context: Context, intent: Intent) {
        if (intent.action != Intent.ACTION_BOOT_COMPLETED) return

        Timber.i("Boot completed — starting OpenClaw Calling Node services")

        // Start Gateway Connection Service
        Intent(context, GatewayConnectionService::class.java).also {
            context.startForegroundService(it)
        }

        // Start Vapi Calling Service
        Intent(context, VapiCallingService::class.java).also {
            context.startForegroundService(it)
        }

        // Start WhatsApp Calling Service
        Intent(context, WhatsAppCallingService::class.java).also {
            context.startForegroundService(it)
        }
    }
}
