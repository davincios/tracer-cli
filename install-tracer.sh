#!/bin/bash

# Check if an API key was provided
if [ "$#" -ne 2 ] || [ "$1" != "--api-key" ]; then
    echo "Usage: $0 --api-key <API_KEY>"
    exit 1
fi

API_KEY=$2

# Define the version of the tracer you want to download
TRACER_VERSION="v0.0.32"
TRACER_LINUX_URL="https://github.com/davincios/tracer-cli/releases/download/${TRACER_VERSION}/tracer-x86_64-unknown-linux-gnu.tar.gz"
TRACER_MACOS_AARCH_URL="https://github.com/davincios/tracer-cli/releases/download/v0.0.32/tracer-aarch64-apple-darwin.tar.gz"
TRACER_MACOS_UNIVERSAL_URL="https://github.com/davincios/tracer-cli/releases/download/${TRACER_VERSION}/tracer-universal-apple-darwin.tar.gz"

# Determine OS and set the appropriate download URL
OS=$(uname -s)
case "$OS" in
Linux*) TRACER_URL=$TRACER_LINUX_URL ;;
Darwin*)
    # Differentiating between ARM and x86_64 architectures on macOS
    ARCH=$(uname -m)
    if [ "$ARCH" = "arm64" ]; then
        TRACER_URL=$TRACER_MACOS_AARCH_URL
    else
        TRACER_URL=$TRACER_MACOS_UNIVERSAL_URL
    fi
    ;;
*)
    echo "Unsupported operating system: $OS"
    exit 1
    ;;
esac

# Check for wget or curl and set download command
if command -v wget >/dev/null; then
    DOWNLOAD_COMMAND="wget -q -O tracer.tar.gz"
elif command -v curl >/dev/null; then
    DOWNLOAD_COMMAND="curl -L -o tracer.tar.gz"
else
    echo "wget or curl is required to download Tracer CLI."
    exit 1
fi

# Download the latest release
echo "Downloading Tracer CLI version ${TRACER_VERSION}..."
$DOWNLOAD_COMMAND "$TRACER_URL"

# Extract the downloaded tarball
echo "Extracting Tracer CLI..."
tar -xzf tracer.tar.gz
chmod +x tracer

# Check if /etc/tracer exists and remove it if it does
if [ -d "/etc/tracer" ]; then
    sudo rm -rf /etc/tracer
fi

# Create directory /etc/tracer and copy the binary there
sudo mkdir -p /etc/tracer/
sudo cp tracer /etc/tracer/tracer

# Execute setup with the API key
echo "Setting up Tracer CLI with provided API key..."
./tracer setup "$API_KEY"

# Function to update tracer configuration in config files
update_config() {
    local config_file=$1
    # Remove existing tracer configuration if present
    sed -i '' '/\/etc\/tracer/d' "$config_file"
    sed -i '' '/alias tracer="tracer"/d' "$config_file"

    # Add new tracer configuration
    echo 'export PATH="$PATH:/etc/tracer"' >>"$config_file"
    echo 'alias tracer="tracer"' >>"$config_file"
    echo "Updated $config_file with tracer configuration."
}

# Update .bashrc and .zshrc
BASH_CONFIG="$HOME/.bashrc"
ZSH_CONFIG="$HOME/.zshrc"
update_config "$BASH_CONFIG"
update_config "$ZSH_CONFIG"

echo "To complete setup, please source your shell configuration files or open a new terminal session:"
echo "source $BASH_CONFIG"
echo "source $ZSH_CONFIG"

# Check tracer version to confirm installation
tracer help

echo "Tracer CLI has been installed successfully."
