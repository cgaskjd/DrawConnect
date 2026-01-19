package com.drawconnect.ui.canvas

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import com.drawconnect.domain.model.DrawingTool

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DrawingScreen(
    viewModel: DrawingViewModel = hiltViewModel()
) {
    val canvasState by viewModel.canvasState.collectAsState()
    val settings by viewModel.settings.collectAsState()
    val saveState by viewModel.saveState.collectAsState()
    var showColorPicker by remember { mutableStateOf(false) }
    var showSaveDialog by remember { mutableStateOf(false) }

    LaunchedEffect(saveState) {
        if (saveState is SaveState.Success) {
            showSaveDialog = false
            viewModel.resetSaveState()
        }
    }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("轻画") },
                actions = {
                    IconButton(onClick = { viewModel.undo() }) {
                        Icon(Icons.Default.Undo, contentDescription = "撤销")
                    }
                    IconButton(onClick = { viewModel.redo() }) {
                        Icon(Icons.Default.Redo, contentDescription = "重做")
                    }
                    IconButton(onClick = { viewModel.clearCanvas() }) {
                        Icon(Icons.Default.Delete, contentDescription = "清空")
                    }
                    IconButton(onClick = { showSaveDialog = true }) {
                        Icon(Icons.Default.Save, contentDescription = "保存")
                    }
                }
            )
        },
        bottomBar = {
            ToolBar(
                currentTool = settings.currentTool,
                currentColor = settings.currentColor,
                onToolSelected = { viewModel.setTool(it) },
                onColorPickerClick = { showColorPicker = true }
            )
        }
    ) { paddingValues ->
        Box(
            modifier = Modifier
                .fillMaxSize()
                .padding(paddingValues)
        ) {
            DrawingCanvas(
                paths = canvasState.paths,
                settings = settings,
                backgroundColor = canvasState.backgroundColor,
                onPathAdded = { viewModel.addPath(it) },
                modifier = Modifier.fillMaxSize()
            )

            if (showColorPicker) {
                ColorPickerDialog(
                    currentColor = settings.currentColor,
                    onColorSelected = { color ->
                        viewModel.setColor(color)
                        showColorPicker = false
                    },
                    onDismiss = { showColorPicker = false }
                )
            }

            if (showSaveDialog) {
                SaveArtworkDialog(
                    saveState = saveState,
                    onSave = { title ->
                        viewModel.saveArtwork("default_user", title)
                    },
                    onDismiss = { showSaveDialog = false }
                )
            }
        }
    }
}

@Composable
fun SaveArtworkDialog(
    saveState: SaveState,
    onSave: (String) -> Unit,
    onDismiss: () -> Unit
) {
    var title by remember { mutableStateOf("") }

    AlertDialog(
        onDismissRequest = onDismiss,
        title = { Text("保存作品") },
        text = {
            Column {
                OutlinedTextField(
                    value = title,
                    onValueChange = { title = it },
                    label = { Text("作品标题") },
                    singleLine = true,
                    modifier = Modifier.fillMaxWidth()
                )

                when (saveState) {
                    is SaveState.Saving -> {
                        Spacer(modifier = Modifier.height(8.dp))
                        CircularProgressIndicator(modifier = Modifier.size(24.dp))
                    }
                    is SaveState.Error -> {
                        Spacer(modifier = Modifier.height(8.dp))
                        Text(
                            text = saveState.message,
                            color = MaterialTheme.colorScheme.error
                        )
                    }
                    else -> {}
                }
            }
        },
        confirmButton = {
            Button(
                onClick = { onSave(title.ifBlank { "未命名作品" }) },
                enabled = saveState !is SaveState.Saving
            ) {
                Text("保存")
            }
        },
        dismissButton = {
            TextButton(onClick = onDismiss) {
                Text("取消")
            }
        }
    )
}

@Composable
fun ToolBar(
    currentTool: DrawingTool,
    currentColor: Color,
    onToolSelected: (DrawingTool) -> Unit,
    onColorPickerClick: () -> Unit
) {
    Surface(
        modifier = Modifier.fillMaxWidth(),
        tonalElevation = 3.dp
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(8.dp),
            horizontalArrangement = Arrangement.SpaceEvenly,
            verticalAlignment = Alignment.CenterVertically
        ) {
            ToolButton(
                icon = Icons.Default.Brush,
                label = "画笔",
                isSelected = currentTool == DrawingTool.BRUSH,
                onClick = { onToolSelected(DrawingTool.BRUSH) }
            )
            ToolButton(
                icon = Icons.Default.CleaningServices,
                label = "橡皮擦",
                isSelected = currentTool == DrawingTool.ERASER,
                onClick = { onToolSelected(DrawingTool.ERASER) }
            )
            ToolButton(
                icon = Icons.Default.FormatColorFill,
                label = "填充",
                isSelected = currentTool == DrawingTool.FILL,
                onClick = { onToolSelected(DrawingTool.FILL) }
            )
            ToolButton(
                icon = Icons.Default.SelectAll,
                label = "选择",
                isSelected = currentTool == DrawingTool.SELECT,
                onClick = { onToolSelected(DrawingTool.SELECT) }
            )
            ToolButton(
                icon = Icons.Default.OpenWith,
                label = "移动",
                isSelected = currentTool == DrawingTool.MOVE,
                onClick = { onToolSelected(DrawingTool.MOVE) }
            )

            // Color picker button
            IconButton(
                onClick = onColorPickerClick,
                modifier = Modifier
                    .size(48.dp)
                    .background(currentColor, shape = MaterialTheme.shapes.small)
            ) {}
        }
    }
}

@Composable
fun ToolButton(
    icon: androidx.compose.ui.graphics.vector.ImageVector,
    label: String,
    isSelected: Boolean,
    onClick: () -> Unit
) {
    IconButton(
        onClick = onClick,
        modifier = Modifier
            .size(48.dp)
            .background(
                color = if (isSelected) MaterialTheme.colorScheme.primaryContainer
                else Color.Transparent,
                shape = MaterialTheme.shapes.small
            )
    ) {
        Icon(
            imageVector = icon,
            contentDescription = label,
            tint = if (isSelected) MaterialTheme.colorScheme.primary
            else MaterialTheme.colorScheme.onSurface
        )
    }
}