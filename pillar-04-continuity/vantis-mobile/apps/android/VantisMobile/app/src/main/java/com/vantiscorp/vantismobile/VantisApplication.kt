package com.vantiscorp.vantismobile

import android.app.Application
import com.vantiscorp.vantismobile.service.SecureTunnelService

/**
 * Main application class for VantisMobile
 */
class VantisApplication : Application() {
    
    companion object {
        lateinit var instance: VantisApplication
            private set
        val tunnelService: SecureTunnelService by lazy { SecureTunnelService() }
    }
    
    override fun onCreate() {
        super.onCreate()
        instance = this
        
        // Initialize services
        // TODO: Initialize notification service
        // TODO: Initialize analytics
        // TODO: Initialize crash reporting
    }
}