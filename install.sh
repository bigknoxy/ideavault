#!/bin/bash

set -e

detect_os() {
    case "$(uname -s)" in
        Linux*)
            echo "linux"
            ;;
        Darwin*)
            echo "macos"
            ;;
        *)
            echo "Unsupported operating system: $(uname -s)" >&2
            exit 1
            ;;
    esac
}

detect_arch() {
    case "$(uname -m)" in
        x86_64* | amd64*)
            echo "x86_64"
            ;;
        aarch64* | arm64*)
            echo "aarch64"
            ;;
        *)
            echo "Unsupported architecture: $(uname -m)" >&2
            exit 1
            ;;
    esac
}

get_download_name() {
    local os="$1"
    local arch="$2"
    
    if [ "$os" = "linux" ]; then
        echo "ideavault-${arch}-unknown-linux-musl.tar.gz"
    elif [ "$os" = "macos" ]; then
        echo "ideavault-${arch}-apple-darwin.tar.gz"
    fi
}

BASE_URL="https://github.com/bigknoxy/ideavault/releases/latest/download"

OS=$(detect_os)
ARCH=$(detect_arch)
FILENAME=$(get_download_name "$OS" "$ARCH")
TEMP_DIR=$(mktemp -d)
DOWNLOAD_URL="${BASE_URL}/${FILENAME}"
CHECKSUM_URL="${BASE_URL}/${FILENAME}.sha256"

echo "Detected OS: $OS"
echo "Detected Architecture: $ARCH"
echo "Downloading: $DOWNLOAD_URL"

cd "$TEMP_DIR"

if ! curl -fsSL -o "$FILENAME" "$DOWNLOAD_URL"; then
    echo "Failed to download $FILENAME" >&2
    rm -rf "$TEMP_DIR"
    exit 1
fi

if curl -fsSL -o "${FILENAME}.sha256" "$CHECKSUM_URL" 2>/dev/null; then
    echo "Validating checksum..."
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum -c "${FILENAME}.sha256" --quiet
    elif command -v shasum >/dev/null 2>&1; then
        shasum -a 256 -c "${FILENAME}.sha256" --quiet
    else
        echo "Warning: No checksum utility available, skipping validation"
    fi
fi

INSTALL_DIR="/usr/local/bin"
TEMP_BINARY="$TEMP_DIR/ideavault"

tar -xzf "$FILENAME"
chmod +x ideavault

if [ -w "$INSTALL_DIR" ]; then
    cp ideavault "$INSTALL_DIR/ideavault"
else
    echo "Installing to $INSTALL_DIR requires sudo..."
    sudo cp ideavault "$INSTALL_DIR/ideavault"
fi

rm -rf "$TEMP_DIR"

echo "Installation successful!"
echo ""
echo "Verifying installation:"
ideavault --help

exit 0
