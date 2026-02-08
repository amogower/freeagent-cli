package com.openclaw.callingnode.config

import android.content.Context
import com.openclaw.callingnode.controller.AgentController
import com.openclaw.callingnode.service.whatsapp.WebRTCManager
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.components.SingletonComponent
import javax.inject.Singleton

/**
 * Hilt module providing application-wide singleton dependencies.
 */
@Module
@InstallIn(SingletonComponent::class)
object AppModule {

    @Provides
    @Singleton
    fun provideConfigManager(
        @ApplicationContext context: Context
    ): ConfigManager {
        return ConfigManager(context)
    }

    @Provides
    @Singleton
    fun provideWebRTCManager(
        @ApplicationContext context: Context
    ): WebRTCManager {
        return WebRTCManager(context)
    }

    @Provides
    @Singleton
    fun provideAgentController(
        configManager: ConfigManager
    ): AgentController {
        return AgentController(configManager)
    }
}
