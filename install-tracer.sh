#!/bin/bash

# Check if an API key was provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 --api-key <API_KEY>"
    exit 1
fi

API_KEY=${1/--api-key=/}

# Define the version of the tracer you want to download
TRACER_VERSION="v0.0.22"
TRACER_URL="https://github.com/davincios/tracer-cli/releases/download/${TRACER_VERSION}/tracer-x86_64-unknown-linux-gnu.tar.gz"

# Download the latest release
echo "Downloading Tracer CLI version ${TRACER_VERSION}..."
wget $TRACER_URL

# Extract the downloaded tarball
echo "Extracting Tracer CLI..."
tar -xzf tracer-x86_64-unknown-linux-gnu.tar.gz

# Execute setup with the API key
echo "Setting up Tracer CLI with provided API key..."
./tracer setup $API_KEY

# Create directory /etc/tracer if it doesn't exist and copy the binary there
sudo mkdir -p /etc/tracer/
sudo cp tracer /etc/tracer/tracer

# Add /etc/tracer to the PATH in .bashrc if not already present
if ! grep -q '/etc/tracer' ~/.bashrc; then
    echo 'export PATH="$PATH:/etc/tracer"' >>~/.bashrc
    echo 'alias tracer="tracer"' >>~/.bashrc
fi

# Apply changes in .bashrc
source ~/.bashrc

# Check tracer version to confirm installation
tracer --version

echo "Tracer CLI has been installed successfully."
