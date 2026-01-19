package com.drawconnect.ui.canvas

import androidx.compose.ui.graphics.Color
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.drawconnect.domain.model.*
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import javax.inject.Inject

@HiltViewModel
class DrawingViewModel @Inject constructor(
    private val artworkRepository: com.drawconnect.data.repository.ArtworkRepository
) : ViewModel() {

    private val _canvasState = MutableStateFlow(CanvasState())
    val canvasState: StateFlow<CanvasState> = _canvasState.asStateFlow()

    private val _settings = MutableStateFlow(DrawingSettings())
    val settings: StateFlow<DrawingSettings> = _settings.asStateFlow()

    private val _undoStack = MutableStateFlow<List<DrawingPath>>(emptyList())
    private val _redoStack = MutableStateFlow<List<DrawingPath>>(emptyList())

    private val _saveState = MutableStateFlow<SaveState>(SaveState.Idle)
    val saveState: StateFlow<SaveState> = _saveState.asStateFlow()

    private var currentArtworkId: String? = null

    fun addPath(path: DrawingPath) {
        viewModelScope.launch {
            val currentPaths = _canvasState.value.paths.toMutableList()
            currentPaths.add(path)
            _canvasState.value = _canvasState.value.copy(paths = currentPaths)
            _redoStack.value = emptyList() // Clear redo stack when new action is performed
        }
    }

    fun undo() {
        viewModelScope.launch {
            val currentPaths = _canvasState.value.paths
            if (currentPaths.isNotEmpty()) {
                val lastPath = currentPaths.last()
                _undoStack.value = _undoStack.value + lastPath
                _canvasState.value = _canvasState.value.copy(
                    paths = currentPaths.dropLast(1)
                )
            }
        }
    }

    fun redo() {
        viewModelScope.launch {
            val undoStack = _undoStack.value
            if (undoStack.isNotEmpty()) {
                val pathToRestore = undoStack.last()
                _undoStack.value = undoStack.dropLast(1)
                val currentPaths = _canvasState.value.paths.toMutableList()
                currentPaths.add(pathToRestore)
                _canvasState.value = _canvasState.value.copy(paths = currentPaths)
            }
        }
    }

    fun clearCanvas() {
        viewModelScope.launch {
            _canvasState.value = _canvasState.value.copy(paths = emptyList())
            _undoStack.value = emptyList()
            _redoStack.value = emptyList()
        }
    }

    fun setTool(tool: DrawingTool) {
        _settings.value = _settings.value.copy(currentTool = tool)
    }

    fun setColor(color: Color) {
        _settings.value = _settings.value.copy(currentColor = color)
    }

    fun setStrokeWidth(width: Float) {
        _settings.value = _settings.value.copy(strokeWidth = width)
    }

    fun setEraserWidth(width: Float) {
        _settings.value = _settings.value.copy(eraserWidth = width)
    }

    fun setBackgroundColor(color: Color) {
        _canvasState.value = _canvasState.value.copy(backgroundColor = color)
    }

    fun canUndo(): Boolean = _canvasState.value.paths.isNotEmpty()
    fun canRedo(): Boolean = _undoStack.value.isNotEmpty()

    fun saveArtwork(userId: String, title: String) {
        viewModelScope.launch {
            _saveState.value = SaveState.Saving
            val result = artworkRepository.saveArtwork(
                userId = userId,
                title = title,
                paths = _canvasState.value.paths,
                width = _canvasState.value.width,
                height = _canvasState.value.height,
                backgroundColor = _canvasState.value.backgroundColor
            )

            _saveState.value = if (result.isSuccess) {
                currentArtworkId = result.getOrNull()?.id
                SaveState.Success
            } else {
                SaveState.Error(result.exceptionOrNull()?.message ?: "保存失败")
            }
        }
    }

    fun loadArtwork(artworkId: String) {
        viewModelScope.launch {
            _saveState.value = SaveState.Loading
            val result = artworkRepository.loadArtwork(artworkId)

            if (result.isSuccess) {
                val drawingData = result.getOrNull()!!
                val paths = drawingData.paths.map { pathData ->
                    DrawingPath(
                        points = pathData.points.map {
                            DrawingPoint(it.x, it.y, it.pressure)
                        },
                        color = Color(pathData.color),
                        strokeWidth = pathData.strokeWidth,
                        tool = DrawingTool.valueOf(pathData.tool)
                    )
                }

                _canvasState.value = CanvasState(
                    width = drawingData.width,
                    height = drawingData.height,
                    backgroundColor = Color(drawingData.backgroundColor),
                    paths = paths
                )
                currentArtworkId = artworkId
                _saveState.value = SaveState.Success
            } else {
                _saveState.value = SaveState.Error(
                    result.exceptionOrNull()?.message ?: "加载失败"
                )
            }
        }
    }

    fun resetSaveState() {
        _saveState.value = SaveState.Idle
    }
}

sealed class SaveState {
    object Idle : SaveState()
    object Saving : SaveState()
    object Loading : SaveState()
    object Success : SaveState()
    data class Error(val message: String) : SaveState()
}