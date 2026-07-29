package dev.flux.ui.app

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp

data class TrafficEntry(
    val method: String,
    val path: String,
    val status: Int,
    val duration: Int,
    val service: String
)

@Composable
fun TrafficScreen() {
    val traffic = remember {
        listOf(
            TrafficEntry("GET", "/api/v1/user", 200, 45, "auth-svc"),
            TrafficEntry("POST", "/api/v1/login", 401, 120, "auth-svc"),
            TrafficEntry("GET", "/products/123", 200, 12, "catalog-svc"),
            TrafficEntry("PUT", "/orders/new", 201, 89, "order-svc")
        )
    }

    var selectedEntry by remember { mutableStateOf<TrafficEntry?>(null) }

    Row(modifier = Modifier.fillMaxSize()) {
        Column(modifier = Modifier.weight(1f)) {
            TopAppBar(title = { Text("Traffic Inspector") })
            LazyColumn {
                items(traffic) { entry ->
                    TrafficItem(entry, onClick = { selectedEntry = entry })
                    Divider()
                }
            }
        }

        if (selectedEntry != null) {
            VerticalDivider()
            Column(modifier = Modifier.width(400.dp).padding(16.dp)) {
                Text("Details", style = MaterialTheme.typography.titleLarge)
                Spacer(Modifier.height(16.dp))
                DetailRow("Method", selectedEntry!!.method)
                DetailRow("Path", selectedEntry!!.path)
                DetailRow("Status", selectedEntry!!.status.toString())
                DetailRow("Duration", "${selectedEntry!!.duration}ms")
                DetailRow("Service", selectedEntry!!.service)
                
                Spacer(Modifier.height(24.dp))
                Text("Payload", fontWeight = FontWeight.Bold)
                Surface(
                    modifier = Modifier.fillMaxWidth().height(200.dp),
                    color = Color.Black.copy(alpha = 0.1f)
                ) {
                    Text("{ \"id\": 123, \"name\": \"Test User\" }", modifier = Modifier.padding(8.dp))
                }
            }
        }
    }
}

@Composable
fun TrafficItem(entry: TrafficEntry, onClick: () -> Unit) {
    Row(
        modifier = Modifier.fillMaxWidth().clickable(onClick = onClick).padding(12.dp),
        horizontalArrangement = Arrangement.SpaceBetween
    ) {
        Row {
            Text(
                entry.method,
                fontWeight = FontWeight.Bold,
                color = if (entry.method == "POST") Color.Yellow else Color.Cyan
            )
            Spacer(Modifier.width(8.dp))
            Text(entry.path)
        }
        Row {
            Text(
                entry.status.toString(),
                color = if (entry.status < 400) Color.Green else Color.Red
            )
            Spacer(Modifier.width(16.dp))
            Text("${entry.duration}ms", color = Color.Gray)
        }
    }
}

@Composable
fun DetailRow(label: String, value: String) {
    Row(modifier = Modifier.padding(vertical = 4.dp)) {
        Text(label, modifier = Modifier.width(80.dp), color = Color.Gray)
        Text(value, fontWeight = FontWeight.Medium)
    }
}
