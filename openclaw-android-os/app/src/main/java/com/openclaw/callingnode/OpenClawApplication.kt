package com.openclaw.callingnode

import android.app.Application
import android.app.NotificationChannel
import android.app.NotificationManager
import android.os.Build
import dagger.hilt.android.HiltAndroidApp
import timber.log.Timber

/**
 * OpenClaw Calling Node Application.
 *
 * This is the entry point for the custom Android OS layer that integrates
 * OpenClaw's AI agent with programmatic phone calling via Vapi AI and
 * WhatsApp Business Calling API.
 */
@HiltAndroidApp
class OpenClawApplication : Application() {

    companion object {
        const val CHANNEL_ACTIVE_CALL = "openclaw_active_call"
        const val CHANNEL_GATEWAY = "openclaw_gateway"
        const val CHANNEL_INCOMING_CALL = "openclaw_incoming_call"
        const val CHANNEL_GENERAL = "openclaw_general"
    }

    override fun onCreate() {
        super.onCreate()

        // Initialize logging
        if (BuildConfig.DEBUG) {
            Timber.plant(Timber.DebugTree())
        } else {
            Timber.plant(CrashReportingTree())
        }

        Timber.i("OpenClaw Calling Node starting up...")

        createNotificationChannels()
    }

    private fun createNotificationChannels() {
        val manager = getSystemService(NotificationManager::class.java)

        val activeCallChannel = NotificationChannel(
            CHANNEL_ACTIVE_CALL,
            "Active Calls",
            NotificationManager.IMPORTANCE_HIGH
        ).apply {
            description = "Notifications for active voice calls"
            setShowBadge(true)
            enableVibration(true)
        }

        val gatewayChannel = NotificationChannel(
            CHANNEL_GATEWAY,
            "Gateway Connection",
            NotificationManager.IMPORTANCE_LOW
        ).apply {
            description = "OpenClaw Gateway connection status"
            setShowBadge(false)
        }

        val incomingCallChannel = NotificationChannel(
            CHANNEL_INCOMING_CALL,
            "Incoming Calls",
            NotificationManager.IMPORTANCE_MAX
        ).apply {
            description = "Incoming call notifications"
            setShowBadge(true)
            enableVibration(true)
            enableLights(true)
        }

        val generalChannel = NotificationChannel(
            CHANNEL_GENERAL,
            "General",
            NotificationManager.IMPORTANCE_DEFAULT
        ).apply {
            description = "General notifications"
        }

        manager.createNotificationChannels(
            listOf(activeCallChannel, gatewayChannel, incomingCallChannel, generalChannel)
        )

        Timber.d("Notification channels created")
    }

    /**
     * Production logging tree that filters verbose/debug logs
     * and reports errors to crash reporting service.
     */
    private class CrashReportingTree : Timber.Tree() {
        override fun log(priority: Int, tag: String?, message: String, t: Throwable?) {
            if (priority == android.util.Log.VERBOSE || priority == android.util.Log.DEBUG) {
                return
            }
            // In production, send errors to crash reporting (e.g., Firebase Crashlytics)
            if (t != null && priority >= android.util.Log.ERROR) {
                // CrashReporter.logException(t)
            }
        }
    }
}
