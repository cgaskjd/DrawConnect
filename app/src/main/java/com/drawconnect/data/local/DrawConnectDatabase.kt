package com.drawconnect.data.local

import androidx.room.Database
import androidx.room.RoomDatabase
import com.drawconnect.data.local.dao.ArtworkDao
import com.drawconnect.data.local.dao.UserDao
import com.drawconnect.data.local.entity.ArtworkEntity
import com.drawconnect.data.local.entity.UserEntity

@Database(
    entities = [ArtworkEntity::class, UserEntity::class],
    version = 1,
    exportSchema = false
)
abstract class DrawConnectDatabase : RoomDatabase() {
    abstract fun artworkDao(): ArtworkDao
    abstract fun userDao(): UserDao
}