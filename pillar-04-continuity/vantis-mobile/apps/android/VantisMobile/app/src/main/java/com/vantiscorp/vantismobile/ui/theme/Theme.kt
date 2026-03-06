package com.vantiscorp.vantismobile.ui.theme

import android.app.Activity
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.SideEffect
import androidx.compose.ui.graphics.toArgb
import androidx.compose.ui.platform.LocalView
import androidx.core.view.WindowCompat

private val DarkColorScheme = darkColorScheme(
    primary = VantisPrimaryLight,
    onPrimary = VantisPrimaryDark,
    secondary = VantisSecondaryLight,
    onSecondary = VantisSecondaryDark,
    error = VantisError,
    onError = VantisErrorDark,
    background = Color(0xFF121212),
    onBackground = Color(0xFFE0E0E0),
    surface = Color(0xFF1E1E1E),
    onSurface = Color(0xFFE0E0E0),
    surfaceVariant = Color(0xFF2C2C2C),
    onSurfaceVariant = Color(0xFFB0B0B0)
)

private val LightColorScheme = lightColorScheme(
    primary = VantisPrimary,
    onPrimary = Color.White,
    secondary = VantisSecondary,
    onSecondary = Color.Black,
    error = VantisError,
    onError = Color.White,
    background = VantisBackground,
    onBackground = Color(0xFF121212),
    surface = VantisSurface,
    onSurface = Color(0xFF121212),
    surfaceVariant = Color(0xFFF5F5F5),
    onSurfaceVariant = Color(0xFF757575)
)

@Composable
fun VantisMobileTheme(
    darkTheme: Boolean = isSystemInDarkTheme(),
    dynamicColor: Boolean = false,
    content: @Composable () -> Unit
) {
    val colorScheme = when {
        darkTheme -> DarkColorScheme
        else -> LightColorScheme
    }
    
    val view = LocalView.current
    if (!view.isInEditMode) {
        SideEffect {
            val window = (view.context as Activity).window
            window.statusBarColor = colorScheme.primary.toArgb()
            WindowCompat.getInsetsController(window, view).isAppearanceLightStatusBars = !darkTheme
        }
    }

    MaterialTheme(
        colorScheme = colorScheme,
        typography = Typography,
        content = content
    )
}