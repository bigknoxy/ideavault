#!/bin/bash

set -e

DRY_RUN=false
KEEP_DATA=false
FORCE=false

INSTALL_DIR="/usr/local/bin"
DATA_DIR="$HOME/.local/share/ideavault"
CACHE_DIR="$HOME/.cache/ideavault"

usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Uninstall IdeaVault from your system.

OPTIONS:
    --dry-run    Preview what will be removed without actually removing anything
    --keep-data  Keep data directory (default: removes data)
    --force      Skip confirmation prompts
    -h, --help   Show this help message

EXAMPLES:
    $0                    # Interactive uninstall
    $0 --dry-run          # Preview what will be removed
    $0 --keep-data        # Uninstall but keep data
    $0 --force            # Uninstall without confirmation

EOF
    exit 0
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --keep-data)
            KEEP_DATA=true
            shift
            ;;
        --force)
            FORCE=true
            shift
            ;;
        -h|--help)
            usage
            ;;
        *)
            echo "Unknown option: $1" >&2
            echo "Use --help for usage information" >&2
            exit 1
            ;;
    esac
done

BINARY_PATH="$INSTALL_DIR/ideavault"
items_to_remove=()

echo "=========================================="
echo "  IdeaVault Uninstall"
echo "=========================================="
echo ""

if [ -f "$BINARY_PATH" ]; then
    echo "[FOUND] Binary: $BINARY_PATH"
    items_to_remove+=("$BINARY_PATH")
else
    echo "[NOT FOUND] Binary: $BINARY_PATH"
fi

if [ -d "$DATA_DIR" ]; then
    echo "[FOUND] Data directory: $DATA_DIR"
    if [ "$KEEP_DATA" = true ]; then
        echo "       (will be kept due to --keep-data flag)"
    else
        items_to_remove+=("$DATA_DIR")
    fi
else
    echo "[NOT FOUND] Data directory: $DATA_DIR"
fi

if [ -d "$CACHE_DIR" ]; then
    echo "[FOUND] Cache directory: $CACHE_DIR"
    items_to_remove+=("$CACHE_DIR")
else
    echo "[NOT FOUND] Cache directory: $CACHE_DIR"
fi

echo ""

if [ ${#items_to_remove[@]} -eq 0 ]; then
    echo "Nothing to remove. IdeaVault is not fully installed or already uninstalled."
    exit 0
fi

echo "The following items will be removed:"
echo ""
for item in "${items_to_remove[@]}"; do
    echo "  - $item"
done
echo ""

if [ "$DRY_RUN" = true ]; then
    echo "DRY RUN MODE - No changes made"
    echo ""
    echo "To actually uninstall, run without --dry-run flag"
    exit 0
fi

if [ "$FORCE" = false ]; then
    read -p "Do you want to continue? [y/N] " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Uninstall cancelled."
        exit 0
    fi
fi

echo "Uninstalling..."
echo ""

for item in "${items_to_remove[@]}"; do
    if [ -f "$item" ]; then
        rm -f "$item"
        echo "[REMOVED] File: $item"
    elif [ -d "$item" ]; then
        rm -rf "$item"
        echo "[REMOVED] Directory: $item"
    fi
done

echo ""
echo "=========================================="
echo "  Uninstall Complete!"
echo "=========================================="
echo ""
echo "Thank you for using IdeaVault."
echo ""

if [ "$KEEP_DATA" = false ]; then
    echo "All data has been removed."
else
    echo "Data directory was kept at: $DATA_DIR"
    echo "To remove it manually, run: rm -rf $DATA_DIR"
fi
