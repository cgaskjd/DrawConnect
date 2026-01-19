package com.drawconnect.data.local.entity

import androidx.room.Entity
import androidx.room.PrimaryKey

@Entity(tableName = "artworks")
data class ArtworkEntity(
    @PrimaryKey
    val id: String,
    val userId: String,
    val title: String,
    val thumbnailPath: String,
    val dataPath: String,
    val width: Int,
    val height: Int,
    val createdAt: Long,
    val updatedAt: Long,
    val isSynced: Boolean = false
)

@Entity(tableName = "users")
data class UserEntity(
    @PrimaryKey
    val id: String,
    val username: String,
    val email: String,
    val avatarUrl: String?,
    val createdAt: Long
)