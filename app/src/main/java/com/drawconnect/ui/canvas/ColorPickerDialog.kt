package com.drawconnect.ui.canvas

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.lazy.grid.items
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Dialog

@Composable
fun ColorPickerDialog(
    currentColor: Color,
    onColorSelected: (Color) -> Unit,
    onDismiss: () -> Unit
) {
    Dialog(onDismissRequest = onDismiss) {
        Surface(
            shape = MaterialTheme.shapes.large,
            tonalElevation = 6.dp
        ) {
            Column(
                modifier = Modifier
                    .padding(16.dp)
                    .fillMaxWidth()
            ) {
                Text(
                    text = "选择颜色",
                    style = MaterialTheme.typography.titleLarge,
                    modifier = Modifier.padding(bottom = 16.dp)
                )

                // Predefined colors
                LazyVerticalGrid(
                    columns = GridCells.Fixed(6),
                    horizontalArrangement = Arrangement.spacedBy(8.dp),
                    verticalArrangement = Arrangement.spacedBy(8.dp),
                    modifier = Modifier.height(300.dp)
                ) {
                    items(predefinedColors) { color ->
                        ColorItem(
                            color = color,
                            isSelected = color == currentColor,
                            onClick = { onColorSelected(color) }
                        )
                    }
                }

                Spacer(modifier = Modifier.height(16.dp))

                // Action buttons
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.End
                ) {
                    TextButton(onClick = onDismiss) {
                        Text("取消")
                    }
                }
            }
        }
    }
}

@Composable
fun ColorItem(
    color: Color,
    isSelected: Boolean,
    onClick: () -> Unit
) {
    Box(
        modifier = Modifier
            .size(48.dp)
            .background(color, CircleShape)
            .border(
                width = if (isSelected) 3.dp else 1.dp,
                color = if (isSelected) MaterialTheme.colorScheme.primary else Color.Gray,
                shape = CircleShape
            )
            .clickable(onClick = onClick)
    )
}

private val predefinedColors = listOf(
    Color.Black,
    Color.White,
    Color.Red,
    Color.Green,
    Color.Blue,
    Color.Yellow,
    Color.Cyan,
    Color.Magenta,
    Color.Gray,
    Color.LightGray,
    Color.DarkGray,
    Color(0xFFFF6B6B), // Light Red
    Color(0xFF4ECDC4), // Turquoise
    Color(0xFF45B7D1), // Sky Blue
    Color(0xFFFFA07A), // Light Salmon
    Color(0xFF98D8C8), // Mint
    Color(0xFFF7DC6F), // Light Yellow
    Color(0xFFBB8FCE), // Light Purple
    Color(0xFFFF9FF3), // Pink
    Color(0xFF54A0FF), // Blue
    Color(0xFF48DBFB), // Light Blue
    Color(0xFF1DD1A1), // Green
    Color(0xFFFF6348), // Orange Red
    Color(0xFFFF4757), // Red
    Color(0xFF5F27CD), // Purple
    Color(0xFF00D2D3), // Cyan
    Color(0xFFFFD32A), // Yellow
    Color(0xFFFF9F1A), // Orange
    Color(0xFFEE5A6F), // Rose
    Color(0xFF341F97), // Dark Purple
    Color(0xFF2E86DE), // Blue
    Color(0xFF10AC84), // Teal
    Color(0xFFC8D6E5), // Light Gray Blue
    Color(0xFF8395A7), // Gray Blue
    Color(0xFF576574), // Dark Gray
    Color(0xFF222F3E)  // Very Dark Gray
)