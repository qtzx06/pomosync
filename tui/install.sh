#!/bin/bash
# a script to build and install pomosync locally.

set -e # exit immediately if a command exits with a non-zero status. 

# --- helper functions for colored output ---
color_red() {
    echo -e "\033[0;31m$1\033[0m"
}

color_green() {
    echo -e "\033[0;32m$1\033[0m"
}

color_yellow() {
    echo -e "\033[0;33m$1\033[0m"
}

# --- dependency check ---
if ! command -v cargo &> /dev/null || ! command -v rustc &> /dev/null; then
    color_red "error: rust isn't installed."
    color_yellow "pomosync needs the rust toolchain (rustc and cargo) to build."
    echo "you can get it by following the official instructions."
    echo "for most linux and macos systems, you can just run this:"
    echo
    color_green "    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo
    echo "after installing, restart your terminal and run this script again."
    exit 1
fi

# --- build and install ---
echo "rust toolchain found. building pomosync in release mode..."
cargo build --release

INSTALL_DIR="$HOME/.local/bin"
BINARY_NAME="pomosync"
SOURCE_PATH="target/release/tui"

echo "build complete. installing binary to $INSTALL_DIR/$BINARY_NAME..."

# create the installation directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# copy the binary and rename it
cp "$SOURCE_PATH" "$INSTALL_DIR/$BINARY_NAME"

color_green "installation successful!"
echo

# --- path configuration ---
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    color_yellow "important: to run 'pomosync' from anywhere, '$INSTALL_DIR' must be in your path."
    
    SHELL_CONFIG_FILE=""
    read -p "do you use bash or zsh? (b/z) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Bb]$ ]]; then
        SHELL_CONFIG_FILE="$HOME/.bashrc"
    elif [[ $REPLY =~ ^[Zz]$ ]]; then
        SHELL_CONFIG_FILE="$HOME/.zshrc"
    else
        color_red "invalid selection. please add the path manually."
        exit 1
    fi

    read -p "add '$INSTALL_DIR' to your $SHELL_CONFIG_FILE now? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo -e "\n# add pomosync to path" >> "$SHELL_CONFIG_FILE"
        echo "export PATH=\"
$HOME/.local/bin:$PATH\"
" >> "$SHELL_CONFIG_FILE"
        color_green "path added. please restart your terminal to apply the changes."
        echo "you can then run the app by just typing:"
        color_green "    pomosync"
    else
        color_yellow "okay. to add it manually, add this line to the bottom of your $SHELL_CONFIG_FILE:"
        echo
        color_green "    export PATH=\"
$HOME/.local/bin:$PATH\"
"
        echo
        color_yellow "then, restart your terminal."
    fi
else
    echo "you can run the app by just typing:"
    color_green "    pomosync"
fi
