#!/usr/bin/env bash
set -e

REPO="cowboy8625/rusty-rain"
PKG_NAME="rusty-rain"
ARCH=$(dpkg --print-architecture)

echo "📦 Detecting latest release for $PKG_NAME..."

# Get latest tag
TAG=$(curl -s https://api.github.com/repos/${REPO}/releases/latest \
    | grep -Po '"tag_name": "\K.*?(?=")')

if [ -z "$TAG" ]; then
    echo "❌ Could not determine latest tag from GitHub API."
    echo "Please check: https://github.com/${REPO}/releases"
    exit 1
fi

DEB_FILE="${PKG_NAME}_${TAG}-1_${ARCH}.deb"
ASSET_URL="https://github.com/${REPO}/releases/download/${TAG}/${DEB_FILE}"

echo "🔍 Checking if .deb asset exists: $ASSET_URL"
if curl --head --silent --fail "$ASSET_URL" > /dev/null; then
    echo "⬇️ Downloading $DEB_FILE..."
    wget -q --show-progress "$ASSET_URL" -O "$DEB_FILE"
    echo "⚙️ Installing..."
    sudo apt install "./$DEB_FILE"
    rm "$DEB_FILE"
    echo "✅ Installed $PKG_NAME $TAG"
else
    echo "⚠️ No .deb package found for:"
    echo "    Version: $TAG"
    echo "    Arch: $ARCH"
    echo
    echo "You can open an issue so we can add support:"
    echo "    https://github.com/${REPO}/issues/new?title=Missing%20Debian%20package%20for%20$TAG%20($ARCH)"
    echo
    read -p "Do you want to try installing via Cargo instead? (y/N): " choice
    case "$choice" in
        y|Y)
            if command -v cargo >/dev/null; then
                echo "📦 Installing via Cargo..."
                cargo install "$PKG_NAME" --version "$TAG" || cargo install "$PKG_NAME"
                echo "✅ Installed via Cargo"
            else
                echo "❌ Cargo is not installed."
                echo "Please install Rust from https://rustup.rs/"
                exit 1
            fi
            ;;
        *)
            echo "❌ Installation aborted."
            exit 1
            ;;
    esac
fi
