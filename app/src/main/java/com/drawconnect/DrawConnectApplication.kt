package com.drawconnect

import android.app.Application
import dagger.hilt.android.HiltAndroidApp

@HiltAndroidApp
class DrawConnectApplication : Application() {
    override fun onCreate() {
        super.onCreate()
    }
}