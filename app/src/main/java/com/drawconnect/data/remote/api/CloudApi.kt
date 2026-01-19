package com.drawconnect.data.remote.api

import com.drawconnect.data.remote.dto.ArtworkDto
import com.drawconnect.data.remote.dto.UploadResponse
import okhttp3.MultipartBody
import retrofit2.Response
import retrofit2.http.*

interface CloudApi {
    @Multipart
    @POST("cloud/upload")
    suspend fun uploadArtwork(
        @Part file: MultipartBody.Part,
        @Part("userId") userId: String,
        @Part("title") title: String
    ): Response<UploadResponse>

    @GET("cloud/artworks/{userId}")
    suspend fun getArtworks(@Path("userId") userId: String): Response<List<ArtworkDto>>

    @GET("cloud/artwork/{id}")
    suspend fun getArtwork(@Path("id") id: String): Response<ArtworkDto>

    @DELETE("cloud/artwork/{id}")
    suspend fun deleteArtwork(@Path("id") id: String): Response<Unit>
}