package com.drawconnect.ui.navigation

import androidx.compose.runtime.Composable
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.drawconnect.ui.auth.LoginScreen
import com.drawconnect.ui.canvas.DrawingScreen

sealed class Screen(val route: String) {
    object Login : Screen("login")
    object Drawing : Screen("drawing")
}

@Composable
fun DrawConnectNavigation() {
    val navController = rememberNavController()

    NavHost(
        navController = navController,
        startDestination = Screen.Login.route
    ) {
        composable(Screen.Login.route) {
            LoginScreen(
                onLoginSuccess = {
                    navController.navigate(Screen.Drawing.route) {
                        popUpTo(Screen.Login.route) { inclusive = true }
                    }
                }
            )
        }

        composable(Screen.Drawing.route) {
            DrawingScreen()
        }
    }
}