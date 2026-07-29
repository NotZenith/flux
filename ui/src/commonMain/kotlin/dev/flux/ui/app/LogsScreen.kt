package dev.flux.ui.app

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.text.selection.SelectionContainer
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import kotlinx.datetime.Clock
import kotlinx.datetime.Instant
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime

data class LogLine(
    val service: String,
    val timestamp: Long,
    val content: String,
    val level: String
)

import androidx.compose.runtime.snapshots.SnapshotStateList

@Composable
fun LogsScreen(logs: SnapshotStateList<LogLine>) {

    Column(modifier = Modifier.fillMaxSize()) {
        TopAppBar(
            title = { Text("Unified Logs") },
            actions = {
                TextButton(onClick = { logs.clear() }) { Text("Clear") }
            }
        )
        
        SelectionContainer(modifier = Modifier.weight(1f).fillMaxWidth()) {
            LazyColumn(
                modifier = Modifier.fillMaxSize().background(Color(0xFF1E1E1E)),
                contentPadding = PaddingValues(8.dp)
            ) {
                items(logs) { log ->
                    LogItem(log)
                }
            }
        }
        
        // Filter Bar
        Surface(tonalElevation = 2.dp) {
            Row(modifier = Modifier.padding(8.dp).fillMaxWidth()) {
                OutlinedTextField(
                    value = "",
                    onValueChange = {},
                    placeholder = { Text("Filter logs...") },
                    modifier = Modifier.weight(1f),
                    singleLine = true
                )
            }
        }
    }
}

@Composable
fun LogItem(log: LogLine) {
    val time = Instant.fromEpochMilliseconds(log.timestamp)
        .toLocalDateTime(TimeZone.currentSystemDefault())
    
    Row(modifier = Modifier.padding(vertical = 2.dp)) {
        Text(
            text = "[${time.hour}:${time.minute}:${time.second}]",
            color = Color.Gray,
            fontFamily = FontFamily.Monospace,
            fontSize = 12.sp
        )
        Spacer(Modifier.width(8.dp))
        Text(
            text = log.service,
            color = Color(0xFF4CAF50),
            fontFamily = FontFamily.Monospace,
            fontSize = 12.sp,
            modifier = Modifier.width(80.dp)
        )
        Spacer(Modifier.width(8.dp))
        Text(
            text = log.content,
            color = Color.White,
            fontFamily = FontFamily.Monospace,
            fontSize = 12.sp
        )
    }
}
