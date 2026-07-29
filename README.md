<div align="center">
  <img src="assets/logo.png" alt="Flux Logo" width="120" />
  <h1>Flux</h1>
  <p><b>High-performance local development orchestrator and observability hub.</b></p>

  <p>
    <a href="https://github.com/NotZenith/flux/actions"><img src="https://img.shields.io/github/actions/workflow/status/NotZenith/flux/ci.yml?branch=main" alt="Build Status" /></a>
    <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT" /></a>
    <a href="https://github.com/NotZenith/flux/releases"><img src="https://img.shields.io/github/v/release/NotZenith/flux" alt="Latest Release" /></a>
    <a href="https://github.com/NotZenith/flux/stargazers"><img src="https://img.shields.io/github/stars/NotZenith/flux?style=flat&label=Stars" alt="GitHub stars" /></a>
  </p>
</div>

---

## ⚡ The Problem: Local Development Chaos

Modern development environments are scattered. You have 12 terminal tabs running microservices, `docker-compose logs` scrolling past at terminal velocity, and port conflicts you can't trace. When something fails between two services, you're stuck digging through disconnected log files or trying to set up complex proxies.

**Flux** solves this. It's a single binary that orchestrates your entire local stack and provides a world-class observability plane.

## 🚀 Key Features

- **Unified Log Plane:** Search, filter, and group logs by request ID across multiple services in real-time.
- **FluxProxy:** A transparent interceptor that visualizes HTTP/gRPC/Redis traffic between your local services.
- **State Snapshots:** Instantly save and restore the state of your local databases and files.
- **Resource Intelligence:** Real-time CPU, Memory, and Network monitoring per service.
- **Blazing Fast UI:** A high-performance desktop UI built in Kotlin/Compose Multiplatform that handles millions of logs without lag.
- **Zero Config:** Works with your existing `docker-compose.yml`, `package.json`, or custom shell scripts.

## 🛠 Installation

### macOS / Linux
```bash
curl -fsSL https://getflux.dev/install.sh | sh
```

### Windows (PowerShell)
```powershell
iwr https://getflux.dev/install.ps1 | iex
```

## 📖 Getting Started

1. **Initialize:** `flux init`
2. **Start your stack:** `flux start`
3. **Inspect traffic:** Open the Flux Desktop app.
4. **Snapshot state:** `flux snap "pre-migration"`

## 🏗 Architecture

Flux is split into two main components:
1. **Core (Rust):** A high-performance engine that handles service management, log ingestion, and proxying.
2. **UI (Compose Multiplatform):** A modern desktop application for visualization and control.

## 🤝 Contributing

We love contributors! Check out our [Contribution Guide](CONTRIBUTING.md) to get started.

## 📄 License

Flux is 100% Open Source under the [MIT License](LICENSE).
