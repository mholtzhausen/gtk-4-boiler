#!/usr/bin/env bash
# Check system dependencies for relm4-kit development.
set -e

echo "Checking system dependencies..."

# Rust / Cargo
if command -v cargo &>/dev/null; then
    echo "  ✓ cargo found: $(cargo --version)"
else
    echo "  ✗ cargo not found. Install Rust: https://rustup.rs"
    exit 1
fi

# GTK4
if pkg-config --exists gtk4; then
    echo "  ✓ gtk4 found: $(pkg-config --modversion gtk4)"
else
    echo "  ✗ gtk4 not found."
    echo "    Install with your package manager:"
    echo "      Debian/Ubuntu: sudo apt install libgtk-4-dev"
    echo "      Fedora:        sudo dnf install gtk4-devel"
    echo "      Arch:          sudo pacman -S gtk4"
    echo "      macOS:         brew install gtk4"
    exit 1
fi

# libadwaita
if pkg-config --exists libadwaita-1; then
    echo "  ✓ libadwaita found: $(pkg-config --modversion libadwaita-1)"
else
    echo "  ✗ libadwaita not found."
    echo "    Install with your package manager:"
    echo "      Debian/Ubuntu: sudo apt install libadwaita-1-dev"
    echo "      Fedora:        sudo dnf install libadwaita-devel"
    echo "      Arch:          sudo pacman -S libadwaita"
    echo "      macOS:         brew install libadwaita"
    exit 1
fi

echo ""
echo "All dependencies found. You're ready to develop!"
