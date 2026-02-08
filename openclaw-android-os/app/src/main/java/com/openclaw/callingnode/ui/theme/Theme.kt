package com.openclaw.callingnode.ui.theme

import android.os.Build
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext

// ── OpenClaw Brand Colors ──
val OpenClawRed = Color(0xFFE53935)
val OpenClawRedDark = Color(0xFFB71C1C)
val OpenClawOrange = Color(0xFFFF6D00)
val OpenClawTeal = Color(0xFF00897B)
val OpenClawDarkBg = Color(0xFF121212)
val OpenClawSurfaceDark = Color(0xFF1E1E1E)
val OpenClawSurfaceLight = Color(0xFFFAFAFA)

private val DarkColorScheme = darkColorScheme(
    primary = OpenClawRed,
    onPrimary = Color.White,
    primaryContainer = OpenClawRedDark,
    onPrimaryContainer = Color.White,
    secondary = OpenClawTeal,
    onSecondary = Color.White,
    secondaryContainer = Color(0xFF004D40),
    onSecondaryContainer = Color.White,
    tertiary = OpenClawOrange,
    onTertiary = Color.White,
    background = OpenClawDarkBg,
    onBackground = Color.White,
    surface = OpenClawSurfaceDark,
    onSurface = Color.White,
    surfaceVariant = Color(0xFF2C2C2C),
    onSurfaceVariant = Color(0xFFCACACA),
    error = Color(0xFFCF6679),
    onError = Color.Black
)

private val LightColorScheme = lightColorScheme(
    primary = OpenClawRed,
    onPrimary = Color.White,
    primaryContainer = Color(0xFFFFCDD2),
    onPrimaryContainer = OpenClawRedDark,
    secondary = OpenClawTeal,
    onSecondary = Color.White,
    secondaryContainer = Color(0xFFB2DFDB),
    onSecondaryContainer = Color(0xFF004D40),
    tertiary = OpenClawOrange,
    onTertiary = Color.White,
    background = Color.White,
    onBackground = Color(0xFF1C1B1F),
    surface = OpenClawSurfaceLight,
    onSurface = Color(0xFF1C1B1F),
    surfaceVariant = Color(0xFFE7E0EC),
    onSurfaceVariant = Color(0xFF49454F),
    error = Color(0xFFB3261E),
    onError = Color.White
)

@Composable
fun OpenClawCallingNodeTheme(
    darkTheme: Boolean = isSystemInDarkTheme(),
    dynamicColor: Boolean = false,
    content: @Composable () -> Unit
) {
    val colorScheme = when {
        dynamicColor && Build.VERSION.SDK_INT >= Build.VERSION_CODES.S -> {
            val context = LocalContext.current
            if (darkTheme) dynamicDarkColorScheme(context) else dynamicLightColorScheme(context)
        }
        darkTheme -> DarkColorScheme
        else -> LightColorScheme
    }

    MaterialTheme(
        colorScheme = colorScheme,
        typography = Typography(),
        content = content
    )
}
