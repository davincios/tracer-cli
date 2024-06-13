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

# Check if /etc/tracer exists and remove it if it does
if [ -d "/etc/tracer" ]; then
    sudo rm -rf /etc/tracer
fi

# Create directory /etc/tracer and copy the binary there
sudo mkdir -p /etc/tracer/
sudo cp tracer /etc/tracer/tracer

# Execute setup with the API key
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
}

# Update .bashrc and .zshrc
BASH_CONFIG="$HOME/.bashrc"
ZSH_CONFIG="$HOME/.zshrc"
update_config "$BASH_CONFIG"
update_config "$ZSH_CONFIG"

echo -e "To complete setup, please source your shell configuration files or open a new terminal session:\nsource $BASH_CONFIG\nsource $ZSH_CONFIG"

echo "Tracer CLI has been installed successfully."

# Cleanup function to remove downloaded and extracted files
cleanup() {
    echo "Cleaning up installation files..."
    rm -f tracer.tar.gz tracer
}

# Call cleanup function before exiting
trap cleanup EXIT

# Sends an event notification to a specified endpoint and logs the response.
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
}

# Main function
main() {
    parse_args "$@"

    # installation
    send_event "start_installation" "Start tracer installation"

    # start fluent-bit
    send_event "finished_installation" "Successfully installed Fluent Bit"
    stop_fluent_bit_after_duration 604800 # 1 week
    send_event "finished_installation" "Fluent Bit has been stopped"
}

# Check if the operating system is Linux
if [ "$(uname -s)" = "Linux" ]; then
    main "$@"
fi
