package com.drawconnect.data.local.dao

import androidx.room.*
import com.drawconnect.data.local.entity.ArtworkEntity
import kotlinx.coroutines.flow.Flow

@Dao
interface ArtworkDao {
    @Query("SELECT * FROM artworks WHERE userId = :userId ORDER BY updatedAt DESC")
    fun getArtworksByUser(userId: String): Flow<List<ArtworkEntity>>

    @Query("SELECT * FROM artworks WHERE id = :id")
    suspend fun getArtworkById(id: String): ArtworkEntity?

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insertArtwork(artwork: ArtworkEntity)

    @Update
    suspend fun updateArtwork(artwork: ArtworkEntity)

    @Delete
    suspend fun deleteArtwork(artwork: ArtworkEntity)

    @Query("DELETE FROM artworks WHERE id = :id")
    suspend fun deleteArtworkById(id: String)

    @Query("SELECT * FROM artworks WHERE isSynced = 0")
    suspend fun getUnsyncedArtworks(): List<ArtworkEntity>
}