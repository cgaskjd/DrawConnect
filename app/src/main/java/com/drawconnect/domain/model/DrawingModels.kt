package com.drawconnect.domain.model

import androidx.compose.ui.graphics.Color

/**
 * 绘画工具类型
 */
enum class DrawingTool {
    BRUSH,      // 画笔
    ERASER,     // 橡皮擦
    FILL,       // 填充
    SELECT,     // 选择
    MOVE        // 移动
}

/**
 * 绘画路径数据
 */
data class DrawingPath(
    val points: List<DrawingPoint>,
    val color: Color,
    val strokeWidth: Float,
    val tool: DrawingTool
)

/**
 * 绘画点数据
 */
data class DrawingPoint(
    val x: Float,
    val y: Float,
    val pressure: Float = 1f
)

/**
 * 画布状态
 */
data class CanvasState(
    val width: Int = 1080,
    val height: Int = 1920,
    val backgroundColor: Color = Color.White,
    val paths: List<DrawingPath> = emptyList()
)

/**
 * 绘画设置
 */
data class DrawingSettings(
    val currentTool: DrawingTool = DrawingTool.BRUSH,
    val currentColor: Color = Color.Black,
    val strokeWidth: Float = 5f,
    val eraserWidth: Float = 20f
)