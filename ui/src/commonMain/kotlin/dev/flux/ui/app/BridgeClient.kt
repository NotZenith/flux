package dev.flux.ui.app

import io.ktor.client.*
import io.ktor.client.engine.cio.*
import io.ktor.client.plugins.websocket.*
import io.ktor.http.*
import io.ktor.websocket.*
import kotlinx.coroutines.*
import kotlinx.serialization.json.*
import kotlinx.serialization.*

@Serializable
data class BridgeMessage(
    val type: String,
    val data: JsonElement
)

class FluxBridgeClient(
    private val onLog: (LogLine) -> Unit,
    private val onTraffic: (TrafficEntry) -> Unit
) {
    private val client = HttpClient(CIO) {
        install(WebSockets)
    }

    fun connect() {
        CoroutineScope(Dispatchers.Default).launch {
            while (isActive) {
                try {
                    client.webSocket(method = HttpMethod.Get, host = "127.0.0.1", port = 9999, path = "/") {
                        println("Connected to Flux Bridge")
                        for (frame in incoming) {
                            if (frame is Frame.Text) {
                                val text = frame.readText()
                                processMessage(text)
                            }
                        }
                    }
                } catch (e: Exception) {
                    println("Bridge connection failed: ${e.message}. Retrying in 5s...")
                    delay(5000)
                }
            }
        }
    }

    private fun processMessage(text: String) {
        val json = Json { ignoreUnknownKeys = true }
        val msg = json.decodeFromString<BridgeMessage>(text)
        when (msg.type) {
            "Log" -> {
                val data = json.decodeFromJsonElement<LogData>(msg.data)
                onLog(LogLine(data.service_id.take(8), data.timestamp, data.content, "info"))
            }
            "Traffic" -> {
                val data = json.decodeFromJsonElement<TrafficData>(msg.data)
                onTraffic(TrafficEntry(data.method, data.path, 0, 0, data.host))
            }
        }
    }
}

@Serializable
data class LogData(val service_id: String, val timestamp: Long, val content: String)

@Serializable
data class TrafficData(val method: String, val path: String, val host: String)
