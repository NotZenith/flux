#!/bin/bash

set -e

echo "🚀 Installing Flux..."

# Detect OS and Architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

if [ "$ARCH" == "x86_64" ]; then
    ARCH="x64"
elif [ "$ARCH" == "aarch64" ] || [ "$ARCH" == "arm64" ]; then
    ARCH="arm64"
fi

VERSION="latest"
BINARY_URL="https://github.com/NotZenith/flux/releases/download/${VERSION}/flux-${OS}-${ARCH}"

# Download binary
echo "Downloading Flux from ${BINARY_URL}..."
curl -fsSL -o /tmp/flux "${BINARY_URL}"

# Install
chmod +x /tmp/flux
sudo mv /tmp/flux /usr/local/bin/flux

echo "✅ Flux successfully installed! Run 'flux --help' to get started."
