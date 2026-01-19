package com.drawconnect.ui.canvas

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.gestures.detectDragGestures
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.*
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.input.pointer.pointerInput
import com.drawconnect.domain.model.DrawingPath
import com.drawconnect.domain.model.DrawingPoint
import com.drawconnect.domain.model.DrawingSettings
import com.drawconnect.domain.model.DrawingTool

@Composable
fun DrawingCanvas(
    paths: List<DrawingPath>,
    settings: DrawingSettings,
    backgroundColor: Color,
    onPathAdded: (DrawingPath) -> Unit,
    modifier: Modifier = Modifier
) {
    var currentPath by remember { mutableStateOf<MutableList<DrawingPoint>>(mutableListOf()) }
    var isDrawing by remember { mutableStateOf(false) }

    Canvas(
        modifier = modifier
            .fillMaxSize()
            .pointerInput(settings) {
                detectDragGestures(
                    onDragStart = { offset ->
                        isDrawing = true
                        currentPath = mutableListOf(
                            DrawingPoint(offset.x, offset.y)
                        )
                    },
                    onDrag = { change, _ ->
                        if (isDrawing) {
                            currentPath.add(
                                DrawingPoint(change.position.x, change.position.y)
                            )
                        }
                    },
                    onDragEnd = {
                        if (isDrawing && currentPath.isNotEmpty()) {
                            val path = DrawingPath(
                                points = currentPath.toList(),
                                color = when (settings.currentTool) {
                                    DrawingTool.ERASER -> backgroundColor
                                    else -> settings.currentColor
                                },
                                strokeWidth = when (settings.currentTool) {
                                    DrawingTool.ERASER -> settings.eraserWidth
                                    else -> settings.strokeWidth
                                },
                                tool = settings.currentTool
                            )
                            onPathAdded(path)
                            currentPath = mutableListOf()
                            isDrawing = false
                        }
                    }
                )
            }
    ) {
        // Draw background
        drawRect(color = backgroundColor)

        // Draw all saved paths
        paths.forEach { drawingPath ->
            drawPath(drawingPath)
        }

        // Draw current path being drawn
        if (currentPath.isNotEmpty()) {
            val tempPath = DrawingPath(
                points = currentPath,
                color = when (settings.currentTool) {
                    DrawingTool.ERASER -> backgroundColor
                    else -> settings.currentColor
                },
                strokeWidth = when (settings.currentTool) {
                    DrawingTool.ERASER -> settings.eraserWidth
                    else -> settings.strokeWidth
                },
                tool = settings.currentTool
            )
            drawPath(tempPath)
        }
    }
}

private fun androidx.compose.ui.graphics.drawscope.DrawScope.drawPath(
    drawingPath: DrawingPath
) {
    if (drawingPath.points.size < 2) return

    val path = Path().apply {
        val firstPoint = drawingPath.points.first()
        moveTo(firstPoint.x, firstPoint.y)

        for (i in 1 until drawingPath.points.size) {
            val point = drawingPath.points[i]
            lineTo(point.x, point.y)
        }
    }

    drawPath(
        path = path,
        color = drawingPath.color,
        style = Stroke(
            width = drawingPath.strokeWidth,
            cap = StrokeCap.Round,
            join = StrokeJoin.Round
        )
    )
}