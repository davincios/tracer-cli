#!/bin/bash

# Check if an API key was provided
if [ "$#" -ne 2 ] || [ "$1" != "--api-key" ]; then
    echo "Usage: $0 --api-key <API_KEY>"
    exit 1
fi

API_KEY=$2

# Define the version of the tracer you want to download
TRACER_VERSION="v0.0.23"
TRACER_URL="https://github.com/davincios/tracer-cli/releases/download/${TRACER_VERSION}/tracer-x86_64-unknown-linux-gnu.tar.gz"

# Check for wget, use curl if not available
DOWNLOAD_COMMAND="wget"
command -v wget >/dev/null 2>&1 || DOWNLOAD_COMMAND="curl -L -O"

# Download the latest release
echo "Downloading Tracer CLI version ${TRACER_VERSION}..."
$DOWNLOAD_COMMAND "$TRACER_URL"

# Extract the downloaded tarball
echo "Extracting Tracer CLI..."
tar -xzf tracer-x86_64-unknown-linux-gnu.tar.gz

# Execute setup with the API key
echo "Setting up Tracer CLI with provided API key..."
./tracer setup "$API_KEY"

# Create directory /etc/tracer if it doesn't exist and copy the binary there
sudo mkdir -p /etc/tracer/
sudo cp tracer /etc/tracer/tracer

# Determine the shell and select the appropriate config file
if [ -n "$ZSH_VERSION" ]; then
    CONFIG_FILE="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
    CONFIG_FILE="$HOME/.bashrc"
else
    echo "Unsupported shell. Please add Tracer CLI to your PATH manually."
    exit 1
fi

# Add /etc/tracer to the PATH in the appropriate config file if not already present
if ! grep -q '/etc/tracer' "$CONFIG_FILE"; then
    echo 'export PATH="$PATH:/etc/tracer"' >>"$CONFIG_FILE"
    echo 'alias tracer="tracer"' >>"$CONFIG_FILE"
    source "$CONFIG_FILE"
fi

# Note for user to source the config file
echo "Please source your $CONFIG_FILE or open a new terminal session to complete setup."

# Check tracer version to confirm installation
tracer --version

echo "Tracer CLI has been installed successfully."
