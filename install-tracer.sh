#!/bin/bash

# Check if an API key was provided
if [ "$#" -ne 1 ]; then
    echo ""
    echo "Welcome to Tracer. To complete the installation, please create an API key at:"
    echo "https://app.tracer.bio"
    echo "and follow the brief instructions during sign-up"
    echo ""
    exit 1
fi

API_KEY=$1

# Define the version of the tracer you want to download
TRACER_VERSION="v0.0.70"
TRACER_LINUX_URL="https://github.com/davincios/tracer-cli/releases/download/${TRACER_VERSION}/tracer-x86_64-unknown-linux-gnu.tar.gz"
TRACER_MACOS_AARCH_URL="https://github.com/davincios/tracer-cli/releases/download/${TRACER_VERSION}/tracer-aarch64-apple-darwin.tar.gz"
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
tar -xzf tracer.tar.gz
chmod +x tracer

# Create configuration directory in the user's home directory
CONFIG_DIR="$HOME/.config/tracer"
mkdir -p "$CONFIG_DIR"
cp tracer "$CONFIG_DIR/tracer"

# Execute setup with the API key
"$CONFIG_DIR/tracer" setup "$API_KEY"

# Function to update configuration in config files
update_config() {
    local config_file=$1
    local bin_path=$2
    local alias_name=$3

    # Remove existing configuration if present
    sed -i "/\/.config\/${alias_name}/d" "$config_file"
    sed -i "/alias ${alias_name}=\"${alias_name}\"/d" "$config_file"

    # Add new configuration
    echo "export PATH=\"\$PATH:${bin_path}\"" >>"$config_file"
    echo "alias ${alias_name}=\"${alias_name}\"" >>"$config_file"
}

# Update .bashrc and .zshrc for tracer
BASH_CONFIG="$HOME/.bashrc"
ZSH_CONFIG="$HOME/.zshrc"
update_config "$BASH_CONFIG" "$HOME/.config/tracer" "tracer"
update_config "$ZSH_CONFIG" "$HOME/.config/tracer" "tracer"

# Immediately update the PATH for the current session
export PATH="$PATH:$HOME/.config/tracer"

echo -e "To complete setup, please source your shell configuration files or open a new terminal session:\nsource $BASH_CONFIG\nsource $ZSH_CONFIG"

echo "Tracer CLI has been installed successfully."

# Cleanup function to remove downloaded and extracted files
cleanup() {
    echo "Cleaning up installation files..."
    rm -f tracer.tar.gz tracer
}

# Call cleanup function before exiting
trap cleanup EXIT

send_event() {
    local event_status="$1"
    local message="$2"
    local response

    response=$(curl -s -w "%{http_code}" -o - \
        --request POST \
        --header "x-api-key: ${API_KEY}" \
        --header 'Content-Type: application/json' \
        --data '{
            "logs": [
                {
                    "message": "'"${message}"'",
                    "event_type": "process_status",
                    "process_type": "installation",
                    "process_status": "'"${event_status}"'"
                }
            ]
        }' \
        "http://app.tracer.bio/api/fluent-bit-webhook")
}

# Installs Fluent Bit.
install_fluent_bit() {
    # Using curl to download and execute the install script
    curl -sSL https://raw.githubusercontent.com/fluent/fluent-bit/master/install.sh | sh

    # Set the Fluent Bit path
    FLUENT_BIT_PATH="/opt/fluent-bit/bin"
    
    # Add Fluent Bit to the PATH in .bashrc and .zshrc
    update_config "$BASH_CONFIG" "$FLUENT_BIT_PATH" "fluent-bit"
    update_config "$ZSH_CONFIG" "$FLUENT_BIT_PATH" "fluent-bit"
    
    # Immediately update the PATH for the current session
    export PATH=$PATH:$FLUENT_BIT_PATH
}

# Main function
main() {
    # installation
    send_event "installation_start" "Start tracer installation"
    install_fluent_bit
    source ~/.bashrc  # Source .bashrc to apply changes
    tracer help
    send_event "installation_finished" "Installation completed"
}

# Check if the operating system is Linux
if [ "$(uname -s)" = "Linux" ]; then
    main "$@"
fi
