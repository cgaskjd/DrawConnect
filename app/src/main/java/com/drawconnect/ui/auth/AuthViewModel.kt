package com.drawconnect.ui.auth

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.drawconnect.data.remote.api.AuthApi
import com.drawconnect.data.remote.dto.LoginRequest
import com.drawconnect.data.remote.dto.RegisterRequest
import com.drawconnect.domain.model.AuthState
import com.drawconnect.domain.model.User
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import javax.inject.Inject

@HiltViewModel
class AuthViewModel @Inject constructor(
    private val authApi: AuthApi
) : ViewModel() {

    private val _authState = MutableStateFlow<AuthState>(AuthState.Idle)
    val authState: StateFlow<AuthState> = _authState.asStateFlow()

    fun login(email: String, password: String) {
        viewModelScope.launch {
            _authState.value = AuthState.Loading
            try {
                val response = authApi.login(LoginRequest(email, password))
                if (response.isSuccessful && response.body()?.success == true) {
                    val userDto = response.body()?.user
                    if (userDto != null) {
                        val user = User(
                            id = userDto.id,
                            username = userDto.username,
                            email = userDto.email,
                            avatarUrl = userDto.avatarUrl,
                            createdAt = userDto.createdAt
                        )
                        _authState.value = AuthState.Success(user)
                    } else {
                        _authState.value = AuthState.Error("登录失败: 用户数据为空")
                    }
                } else {
                    _authState.value = AuthState.Error(
                        response.body()?.message ?: "登录失败"
                    )
                }
            } catch (e: Exception) {
                _authState.value = AuthState.Error("网络错误: ${e.message}")
            }
        }
    }

    fun register(username: String, email: String, password: String) {
        viewModelScope.launch {
            _authState.value = AuthState.Loading
            try {
                val response = authApi.register(
                    RegisterRequest(username, email, password)
                )
                if (response.isSuccessful && response.body()?.success == true) {
                    val userDto = response.body()?.user
                    if (userDto != null) {
                        val user = User(
                            id = userDto.id,
                            username = userDto.username,
                            email = userDto.email,
                            avatarUrl = userDto.avatarUrl,
                            createdAt = userDto.createdAt
                        )
                        _authState.value = AuthState.Success(user)
                    } else {
                        _authState.value = AuthState.Error("注册失败: 用户数据为空")
                    }
                } else {
                    _authState.value = AuthState.Error(
                        response.body()?.message ?: "注册失败"
                    )
                }
            } catch (e: Exception) {
                _authState.value = AuthState.Error("网络错误: ${e.message}")
            }
        }
    }

    fun resetAuthState() {
        _authState.value = AuthState.Idle
    }
}