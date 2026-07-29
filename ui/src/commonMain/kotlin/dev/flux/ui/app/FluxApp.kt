package dev.flux.ui.app

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector

@Composable
fun FluxApp() {
    var currentScreen by remember { mutableStateOf(Screen.Dashboard) }

    MaterialTheme(
        colorScheme = darkColorScheme()
    ) {
        Surface(modifier = Modifier.fillMaxSize()) {
            Row(modifier = Modifier.fillMaxSize()) {
                NavigationRail(
                    modifier = Modifier.width(80.dp)
                ) {
                    FluxNavRailItem(
                        selected = currentScreen == Screen.Dashboard,
                        onClick = { currentScreen = Screen.Dashboard },
                        icon = Icons.Default.Dashboard,
                        label = "Dashboard"
                    )
                    FluxNavRailItem(
                        selected = currentScreen == Screen.Logs,
                        onClick = { currentScreen = Screen.Logs },
                        icon = Icons.Default.List,
                        label = "Logs"
                    )
                    FluxNavRailItem(
                        selected = currentScreen == Screen.Traffic,
                        onClick = { currentScreen = Screen.Traffic },
                        icon = Icons.Default.SwapHoriz,
                        label = "Traffic"
                    )
                    FluxNavRailItem(
                        selected = currentScreen == Screen.Snapshots,
                        onClick = { currentScreen = Screen.Snapshots },
                        icon = Icons.Default.CameraAlt,
                        label = "Snapshots"
                    )
                    Spacer(Modifier.weight(1f))
                    FluxNavRailItem(
                        selected = false,
                        onClick = { /* Settings */ },
                        icon = Icons.Default.Settings,
                        label = "Settings"
                    )
                }

                Box(modifier = Modifier.weight(1f)) {
                    when (currentScreen) {
                        Screen.Dashboard -> DashboardScreen()
                        Screen.Logs -> LogsScreen()
                        Screen.Traffic -> TrafficScreen()
                        Screen.Snapshots -> SnapshotsScreen()
                    }
                }
            }
        }
    }
}

@Composable
fun FluxNavRailItem(
    selected: Boolean,
    onClick: () -> Unit,
    icon: ImageVector,
    label: String
) {
    NavigationRailItem(
        selected = selected,
        onClick = onClick,
        icon = { Icon(icon, contentDescription = label) },
        label = { Text(label, style = MaterialTheme.typography.labelSmall) }
    )
}

enum class Screen {
    Dashboard, Logs, Traffic, Snapshots
}

@Composable fun DashboardScreen() { Text("Dashboard Screen", modifier = Modifier.padding(16.dp)) }
@Composable fun LogsScreen() { Text("Unified Logs", modifier = Modifier.padding(16.dp)) }
@Composable fun TrafficScreen() { Text("Traffic Inspector", modifier = Modifier.padding(16.dp)) }
@Composable fun SnapshotsScreen() { Text("State Snapshots", modifier = Modifier.padding(16.dp)) }
