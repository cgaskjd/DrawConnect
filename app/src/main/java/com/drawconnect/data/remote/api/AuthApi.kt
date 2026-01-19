package com.drawconnect.data.remote.api

import com.drawconnect.data.remote.dto.LoginRequest
import com.drawconnect.data.remote.dto.LoginResponse
import com.drawconnect.data.remote.dto.RegisterRequest
import com.drawconnect.data.remote.dto.RegisterResponse
import retrofit2.Response
import retrofit2.http.Body
import retrofit2.http.POST

interface AuthApi {
    @POST("auth/register")
    suspend fun register(@Body request: RegisterRequest): Response<RegisterResponse>

    @POST("auth/login")
    suspend fun login(@Body request: LoginRequest): Response<LoginResponse>
}