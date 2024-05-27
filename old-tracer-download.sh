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
     rm -rf /etc/tracer
fi

# Create directory /etc/tracer and copy the binary there
 mkdir -p /etc/tracer/
 cp tracer /etc/tracer/tracer

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

##################################### FLUENTBIT STARTS HERE
# Todos #1: run fluent-bit in the background thus close the fluent-bit monitoring process with ctrl c or something like that
# Todo #2: [future] update the API key schema in the database to mark the API key as "isInitialized = true"

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

# Creates the Fluent Bit configuration content.
create_fluent_bit_config() {
    cat <<EOF
[SERVICE]
    flush        20
    daemon       Off
    log_level    info
    parsers_file parsers.conf
    plugins_file plugins.conf
    http_server  Off
    http_listen  0.0.0.0
    http_port    2020
    storage.metrics on

[INPUT]
    name cpu
    tag  cpu.local
    interval_sec 30
    
[INPUT]
    name            mem
    tag             mem.local
    interval_sec    30
    
[INPUT]
    name          netif
    tag           netif
    interval_Sec  30
    interval_NSec 0
    interface     eth0


[INPUT]
    name            disk
    tag             disk.local
    interval_sec    30

[OUTPUT]
    name            http
    match           *
    host            app.tracer.bio
    port            443
    uri             /api/fluent-bit-webhook-without-logs
    format          json
    tls             On
    tls.verify      Off
    header          Content-Type application/json
    header          X-Api-Key ${API_KEY}
EOF
}

# Configures Fluent Bit with a dynamic API key.
configure_fluent_bit() {
    echo 'export PATH=$PATH:/opt/fluent-bit/bin' >>~/.bashrc
    source ~/.bashrc

    fluent_bit_version=$(fluent-bit --version | grep -oP '^Fluent Bit v\K[\d.]+')
    required_version="3.0.0" # Set to a base comparison version that follows the semantic versioning pattern

    # Convert version numbers to a comparable format by padding shorter versions with zeros
    # This transforms version numbers into a format that can be directly compared
    ver_to_compare=$(echo "$fluent_bit_version" | awk -F. '{ printf("%d%03d%03d", $1, $2, $3); }')
    required_ver_compare=$(echo "$required_version" | awk -F. '{ printf("%d%03d%03d", $1, $2, $3); }')

    # Now compare the padded numbers
    if [ "$ver_to_compare" -le "$required_ver_compare" ]; then
        echo "Fluent Bit version higher than 3.00 is required, but found $fluent_bit_version"
        exit 1
    fi

    local config_path="/etc/fluent-bit/fluent-bit.conf"
    local config_content=$(create_fluent_bit_config)

    echo "$config_content" |  tee "$config_path" >/dev/null
}

# Updates .bashrc to include Fluent Bit in the PATH
update_bashrc_for_fluent_bit() {
    local fluent_bit_path_entry='export PATH=$PATH:/opt/fluent-bit/bin/fluent-bit'
    # Check if the PATH update is already in .bashrc; if not, append it.
    if ! grep -qF -- "$fluent_bit_path_entry" ~/.bashrc; then
        echo "$fluent_bit_path_entry" >>~/.bashrc
        echo "Fluent Bit PATH added to .bashrc. Please run 'source ~/.bashrc' or restart your terminal session to apply changes."
    else
        echo "Fluent Bit PATH already in .bashrc."
    fi
}

# Starts Fluent Bit in the background and echoes its PID.
start_fluent_bit() {
    fluent-bit -d -c /etc/fluent-bit/fluent-bit.conf >/dev/null 2>&1 &
    FLUENT_BIT_PID=$!
    echo "Fluent Bit started with PID: $FLUENT_BIT_PID."
}

# Waits for a specified duration and then stops Fluent Bit.
stop_fluent_bit_after_duration() {
    local duration=$1 # Duration in seconds before stopping Fluent Bit.
    echo "Waiting for $duration seconds before stopping Fluent Bit..."
    sleep "$duration"

    if [[ -n $FLUENT_BIT_PID ]] && kill "$FLUENT_BIT_PID" 2>/dev/null; then
        echo "Fluent Bit has been stopped automatically after the timeout."
    else
        echo "Failed to stop Fluent Bit. It may have already been stopped, or the PID was incorrect."
    fi
}

# Main function
main() {
    parse_args "$@"

    # installation
    send_event "start_installation" "Start tracer installation"
}

# Check if the operating system is Linux
if [ "$(uname -s)" = "Linux" ]; then
    main "$@"
fi