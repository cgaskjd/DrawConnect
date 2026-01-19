package com.drawconnect.data.repository

import android.content.Context
import android.graphics.Bitmap
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.toArgb
import com.drawconnect.data.local.dao.ArtworkDao
import com.drawconnect.data.local.entity.ArtworkEntity
import com.drawconnect.domain.model.Artwork
import com.drawconnect.domain.model.DrawingPath
import com.google.gson.Gson
import com.google.gson.reflect.TypeToken
import dagger.hilt.android.qualifiers.ApplicationContext
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.withContext
import java.io.File
import java.io.FileOutputStream
import java.util.*
import javax.inject.Inject
import javax.inject.Singleton

@Singleton
class ArtworkRepository @Inject constructor(
    @ApplicationContext private val context: Context,
    private val artworkDao: ArtworkDao
) {
    private val gson = Gson()

    suspend fun saveArtwork(
        userId: String,
        title: String,
        paths: List<DrawingPath>,
        width: Int,
        height: Int,
        backgroundColor: Color
    ): Result<Artwork> = withContext(Dispatchers.IO) {
        try {
            val artworkId = UUID.randomUUID().toString()
            val timestamp = System.currentTimeMillis()

            // Save drawing data as JSON
            val dataFile = File(context.filesDir, "artworks/$artworkId.json")
            dataFile.parentFile?.mkdirs()

            val drawingData = DrawingData(
                paths = paths.map { path ->
                    PathData(
                        points = path.points.map { PointData(it.x, it.y, it.pressure) },
                        color = path.color.toArgb(),
                        strokeWidth = path.strokeWidth,
                        tool = path.tool.name
                    )
                },
                backgroundColor = backgroundColor.toArgb(),
                width = width,
                height = height
            )

            dataFile.writeText(gson.toJson(drawingData))

            // Generate thumbnail (placeholder for now)
            val thumbnailPath = "artworks/${artworkId}_thumb.png"

            // Save to database
            val entity = ArtworkEntity(
                id = artworkId,
                userId = userId,
                title = title,
                thumbnailPath = thumbnailPath,
                dataPath = dataFile.absolutePath,
                width = width,
                height = height,
                createdAt = timestamp,
                updatedAt = timestamp,
                isSynced = false
            )

            artworkDao.insertArtwork(entity)

            val artwork = Artwork(
                id = artworkId,
                userId = userId,
                title = title,
                thumbnailPath = thumbnailPath,
                dataPath = dataFile.absolutePath,
                width = width,
                height = height,
                createdAt = timestamp,
                updatedAt = timestamp,
                isSynced = false
            )

            Result.success(artwork)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    suspend fun loadArtwork(artworkId: String): Result<DrawingData> = withContext(Dispatchers.IO) {
        try {
            val entity = artworkDao.getArtworkById(artworkId)
                ?: return@withContext Result.failure(Exception("Artwork not found"))

            val dataFile = File(entity.dataPath)
            if (!dataFile.exists()) {
                return@withContext Result.failure(Exception("Artwork data file not found"))
            }

            val json = dataFile.readText()
            val drawingData = gson.fromJson(json, DrawingData::class.java)

            Result.success(drawingData)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    fun getArtworksByUser(userId: String): Flow<List<Artwork>> {
        return artworkDao.getArtworksByUser(userId).map { entities ->
            entities.map { entity ->
                Artwork(
                    id = entity.id,
                    userId = entity.userId,
                    title = entity.title,
                    thumbnailPath = entity.thumbnailPath,
                    dataPath = entity.dataPath,
                    width = entity.width,
                    height = entity.height,
                    createdAt = entity.createdAt,
                    updatedAt = entity.updatedAt,
                    isSynced = entity.isSynced
                )
            }
        }
    }

    suspend fun deleteArtwork(artworkId: String): Result<Unit> = withContext(Dispatchers.IO) {
        try {
            val entity = artworkDao.getArtworkById(artworkId)
            if (entity != null) {
                // Delete files
                File(entity.dataPath).delete()
                File(context.filesDir, entity.thumbnailPath).delete()

                // Delete from database
                artworkDao.deleteArtworkById(artworkId)
            }
            Result.success(Unit)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }
}

// Data classes for JSON serialization
data class DrawingData(
    val paths: List<PathData>,
    val backgroundColor: Int,
    val width: Int,
    val height: Int
)

data class PathData(
    val points: List<PointData>,
    val color: Int,
    val strokeWidth: Float,
    val tool: String
)

data class PointData(
    val x: Float,
    val y: Float,
    val pressure: Float
)