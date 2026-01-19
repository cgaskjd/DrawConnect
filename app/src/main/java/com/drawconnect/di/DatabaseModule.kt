package com.drawconnect.di

import android.content.Context
import androidx.room.Room
import com.drawconnect.data.local.DrawConnectDatabase
import com.drawconnect.data.local.dao.ArtworkDao
import com.drawconnect.data.local.dao.UserDao
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.components.SingletonComponent
import javax.inject.Singleton

@Module
@InstallIn(SingletonComponent::class)
object DatabaseModule {

    @Provides
    @Singleton
    fun provideDatabase(@ApplicationContext context: Context): DrawConnectDatabase {
        return Room.databaseBuilder(
            context,
            DrawConnectDatabase::class.java,
            "drawconnect_database"
        ).build()
    }

    @Provides
    fun provideArtworkDao(database: DrawConnectDatabase): ArtworkDao {
        return database.artworkDao()
    }

    @Provides
    fun provideUserDao(database: DrawConnectDatabase): UserDao {
        return database.userDao()
    }
}