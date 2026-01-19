package com.drawconnect.domain.model

/**
 * 用户数据模型
 */
data class User(
    val id: String,
    val username: String,
    val email: String,
    val avatarUrl: String? = null,
    val createdAt: Long = System.currentTimeMillis()
)

/**
 * 作品数据模型
 */
data class Artwork(
    val id: String,
    val userId: String,
    val title: String,
    val thumbnailPath: String,
    val dataPath: String,
    val width: Int,
    val height: Int,
    val createdAt: Long = System.currentTimeMillis(),
    val updatedAt: Long = System.currentTimeMillis(),
    val isSynced: Boolean = false
)

/**
 * 认证状态
 */
sealed class AuthState {
    object Idle : AuthState()
    object Loading : AuthState()
    data class Success(val user: User) : AuthState()
    data class Error(val message: String) : AuthState()
}