#!/bin/bash

# TODOS:
# - [x] check pre-requisite binaries are there 
# - [ ] check versions of dynamic libraries
# - [x] check internet/server is accessible
#       curl does this implicitly
# - [x] move config to a .config or .tracerbio directory instead of /etc
# - [x] add a function to check if the API key is valid
#       tracer binary does this implicitly
# - [x] check which shell is running (bash/zsh/older) and configure accordingly
#       


# Define the version of the tracer you want to download
#---  PARAMETERS  --------------------------------------------------------------
#   DESCRIPTION:  Parameters used in the rest of this script
#-------------------------------------------------------------------------------
SCRIPT_VERSION="v0.0.1"
TRACER_VERSION="v0.0.70"
TRACER_LINUX_URL="https://github.com/davincios/tracer-cli/releases/download/${TRACER_VERSION}/tracer-x86_64-unknown-linux-gnu.tar.gz"
TRACER_MACOS_AARCH_URL="https://github.com/davincios/tracer-cli/releases/download/${TRACER_VERSION}/tracer-aarch64-apple-darwin.tar.gz"
TRACER_MACOS_UNIVERSAL_URL="https://github.com/davincios/tracer-cli/releases/download/${TRACER_VERSION}/tracer-universal-apple-darwin.tar.gz"

TRACER_HOME="$HOME/.tracerbio"
LOGFILE_NAME="tracer-installer.log"
CONFIGFILE_NAME="apikey.conf"

LOGFILE="$TRACER_HOME/$LOGFILE_NAME" 
CONFIGFILE="$TRACER_HOME/$CONFIGFILE_NAME"
PACKAGE_NAME="" # set later
BINDIRS=("$HOME/bin" "$HOME/.local/bin" "$TRACER_HOME/bin")
BINDIR="" # set later

API_KEY="" # set later

FLUENT_BIT_INSTALL_SCRIPT="https://raw.githubusercontent.com/fluent/fluent-bit/master/install.sh"

#---  VARIABLES  ---------------------------------------------------------------
#          NAME:  Red|Gre|Yel|Bla|RCol
#   DESCRIPTION:  Utility variables for pretty printing etc
#-------------------------------------------------------------------------------
# if tput is available use colours. 
if tput setaf 1 >/dev/null 2>&1; then
  Red=$(tput setaf 1); Gre=$(tput setaf 2); Yel=$(tput setaf 3); Blu=$(tput setaf 4); Bla=$(tput setaf 0);
  RCol=$(tput sgr0)
  ExitTrap="" # placeholder for resetting advanced functionality
else
  Red=""; Gre=""; Yel=""; Bla=""; Blu=""; RCol=""
  ExitTrap=""
fi

# init var
tsnow=""

#---  FUNCTIONS  ---------------------------------------------------------------
#          NAME:  print[scr|log|error]
#   DESCRIPTION:  Some more utility functions for printing stuff... zzz 
#                 scr prints to the screen, 
#                 log to the log,  
#                 error sticks a big red error in front and prints to both
#    PARAMETERS:  $1 is whatever is to be printed
#-------------------------------------------------------------------------------
tsupd()         { command -v date >/dev/null 2>&1 && tsnow=$(date +%F,%T%t;); }
printlog()      { tsupd; echo -e "${tsnow} - $*" >> "$LOGFILE"; }

printmsg()      { printf '%s\n' "$*"; printlog "$*"; }
printnolog()      { printf '%s\n' "$*"; }
printindmsg()   { printf '         %s\n' "$*"; printlog "         $*"; }

# with newlines
printsucc()     { printf '%s\n' "${Gre}Success:${RCol} $*"; printlog "SUCCESS: $*"; }
printinfo()     { printf '%s\n' "${Blu}Info:   ${RCol} $*"; printlog "INFO:    $*"; }
printwarn()     { printf '%s\n' "${Yel}Warning:${RCol} $*"; printlog "WARNING: $*"; }
printerror()    { printf "%s\n" "${Red}Error:  ${RCol} $*";   printlog "ERROR:   $*"; }

# partials
printpmsg()      { printf '%s' "$*"; printlog "$*"; }
printpsucc()     { printf '%s' "${Gre}Success:${RCol} $*"; printlog "SUCCESS: $*"; }
printpinfo()     { printf '%s' "${Blu}Info:   ${RCol} $*"; printlog "INFO:    $*"; }
printpwarn()     { printf '%s' "${Yel}Warning:${RCol} $*"; printlog "WARNING: $*"; }
printperror()    { printf "%s" "${Red}Error:  ${RCol} $*";   printlog "ERROR:   $*"; }


function check_prereqs() {
  # Curl is not optional due to event sending function below
  hardreqs=(tar curl sed chmod echo cat source grep sleep uname basename)

  local thingsNotFound=0;
  local notFoundList=();
  for thing in "${hardreqs[@]}"; do
    command -v "$thing" >/dev/null 2>&1 || { 
      thingsNotFound=$(( $thingsNotFound + 1))
      notFoundList+=("$thing")
    }
  done
  if [[ $thingsNotFound -ne 0 ]]; then
    printerror  "This installation script requires the following commands to be available on your system: "
    for thing in "${notFoundList[@]}"; do
      printindmsg " - ${Yel}${thing}${RCol}"
    done
    printindmsg "Please install them or ensure they are on your PATH and try again."
    exit 1 
  fi
  printinfo "All required commands found on path." # in case the user had the error before
}


function print_header() {
  printnolog " "
  printnolog "⠀⠀⠀⠀⠀⠀⠀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀│ "    
  printnolog "⠀⢷⣦⣦⣄⣄⣔⣿⣿⣆⣄⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀│ Tracer.bio CLI Installer"    
  printnolog "⠀⠀⠻⣿⣿⣿⣿⣿⣿⣿⣿⠛⣿⣷⣦⡄⡀⠀⠀⠀⠀⠀⠀⠀⠀│ "    
  printnolog "⠀⠀⠀⠈⠻⣻⣿⣿⣿⣿⣿⣷⣷⣿⣿⣿⣷⣧⡄⡀⠀⠀⠀⠀⠀│ Script version: ${Blu}${SCRIPT_VERSION}${RCol}"    
  printnolog "⠀⠀⠀⠀⠀⠀⠘⠉⠃⠑⠁⠃⠋⠋⠛⠟⢿⢿⣿⣷⣦⡀⠀⠀⠀│ Tracer version: ${Blu}${TRACER_VERSION}${RCol}"    
  printnolog "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⠙⠻⠿⣧⠄⠀│ "    
  printnolog "⠀          ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠀⠀│ "
  printnolog " "
}

function print_help() {
  printindmsg ""
  printindmsg "Example Usage: "
  printindmsg "  ${Gre}$0 <your_api_key>${RCol}"
  printindmsg ""
  printindmsg "To obtain your API key, log in to your console at ${Blu}https://app.tracer.bio${RCol}"
}




#-------------------------------------------------------------------------------
#          NAME:  check_os
#   DESCRIPTION:  Check the OS and set the appropriate download URL
#-------------------------------------------------------------------------------
check_os() {
  OS=$(uname -s)
  case "$OS" in
  Linux*) 
      printinfo "Detected Linux OS."
      TRACER_URL=$TRACER_LINUX_URL ;;
  Darwin*)
      # Differentiating between ARM and x86_64 architectures on macOS
      ARCH=$(uname -m)
      if [ "$ARCH" = "arm64" ]; then
          printinfo "Detected macOS ARM64 architecture"
          TRACER_URL=$TRACER_MACOS_AARCH_URL
      else
          printinfo "Detected macOS universal architecture"
          TRACER_URL=$TRACER_MACOS_UNIVERSAL_URL
      fi
      ;;
  *)
      printerror "Detected unsupported operating system: $OS. Aborting."
      exit 1
      ;;
  esac
}


#-------------------------------------------------------------------------------
#          NAME:  check_args
#   DESCRIPTION:  Checks if an API key was provided
#-------------------------------------------------------------------------------
check_args() {
  # Check if an API key was provided
  if [ "$#" -ne 1 ]; then
      printerror "Incorrect number of arguments. To run this installer, please provide your Tracer API key"
      print_help
      exit 1
  fi
  API_KEY=$1

}

#-------------------------------------------------------------------------------
#          NAME:  check_args
#   DESCRIPTION:  Gets name of just the file from the download url
#-------------------------------------------------------------------------------
function get_package_name() {
  PACKAGE_NAME=$(basename "$TRACER_URL")
}

function configure_bindir() {
  local dirfound=0
  for dir in "${BINDIRS[@]}"; do
    if [ -d "$dir" ]; then
      if [[ :$PATH: == *:$dir:* ]]; then
        dirfound=1
        BINDIR=$dir
        printinfo "Local bin directory ${Blu}$dir${RCol} found. Tracer will be installed there."
        break
      fi
    fi
  done
  if [ $dirfound -eq 0 ]; then
    BINDIR=${TRACER_HOME}/bin
    printwarn "No local bin directory found. Tracer will be installed in ${Blu}$BINDIR${RCol}."
    mkdir -p "$BINDIR"
    if [ $? -ne 0 ]; then
      printerror "Failed to create ${Blu}$BINDIR${RCol} directory. Please check your permissions and try again."
      exit 1
    fi
    update_rc
  fi
}

#-------------------------------------------------------------------------------
#          NAME:  make_temp_dir
#   DESCRIPTION:  Creates a temporary directory to support installation
#-------------------------------------------------------------------------------
function make_temp_dir() {
  TRACER_TEMP_DIR=$(mktemp -d)
  if [ $? -ne 0 ]; then
      printerror "Failed to create temporary directory. Please check your permissions and try again."
      exit 1
  fi
  printinfo "Temporary directory ${Blu}$TRACER_TEMP_DIR${RCol} created."
}

#-------------------------------------------------------------------------------
#          NAME:  download_tracer
#   DESCRIPTION:  Downloads and extracts the Tracer binary
#-------------------------------------------------------------------------------
function download_tracer() {
  DLTARGET="$TRACER_TEMP_DIR/package"
  EXTRACTTARGET="$TRACER_TEMP_DIR/extracted"

  mkdir -p "$DLTARGET"
  mkdir -p "$EXTRACTTARGET"

  printpinfo "Downloading package..."
  curl -sSL --progress-bar -o "${DLTARGET}/${PACKAGE_NAME}" "$TRACER_URL"
  if [ $? -ne 0 ]; then
      printerror "Failed to download Tracer. Please check your internet connection and try again."
      exit 1
  fi
  printmsg " done."
  printpinfo "Extracting package..."
  tar -xzf "${DLTARGET}/${PACKAGE_NAME}" -C "$EXTRACTTARGET"
  printmsg " done."
  chmod +x "${EXTRACTTARGET}/tracer"
  if [ $? -ne 0 ]; then
      printerror "Failed to set executable permissions on extracted binary. Please check your permissions and mount flags."
      exit 1
  fi

  # move binary to bin dir
  mv "${EXTRACTTARGET}/tracer" "$BINDIR/tracer"
  if [ $? -ne 0 ]; then
      printerror "Failed to move Tracer binary to ${Blu}$BINDIR${RCol}. Please check your permissions and try again."
      exit 1
  fi
  printsucc "Tracer binary moved to ${Blu}$BINDIR${RCol}."
}


#-------------------------------------------------------------------------------
#          NAME:  configure_tracer
#   DESCRIPTION:  Configures the Tracer installation
#-------------------------------------------------------------------------------
function configure_tracer() {
  # check whether a .tracerbio directory exists in the user's home directory and create it if it doesn't
  if [ ! -d "$TRACER_HOME" ]; then
      mkdir -p "$TRACER_HOME"
      # if this failed to create the directory, print an error message and exit
      if [ $? -ne 0 ]; then
          printerror "Failed to create $HOME/.tracerbio directory. Please check your permissions and try again."
          exit 1
      fi
      printsucc "Tracer config directory ${Blu}$TRACER_HOME${RCol} created."
  else 
      printinfo "Tracer config directory ${Blu}$TRACER_HOME${RCol} exists."
  fi
  printinfo "Installation log will be written to ${Blu}$LOGFILE${RCol}."

  # check whether an API key file exists in the user's .tracerbio directory and create it if it doesn't
  if [ ! -f "$CONFIGFILE" ]; then
      echo "$API_KEY" > "$CONFIGFILE"
      # if this failed to create the file, print an error message and exit
      if [ $? -ne 0 ]; then
          # TODO: allow for this to be overriden by an existing TRACER_CONFDIR variable or something
          printerror "Failed to create ${Blu}${CONFIGFILE}${RCol}. Please check your permissions and try again."
          exit 1
      fi
  else 
      printinfo "File ${Blu}$HOME/.tracerbio/$CONFIGFILE_NAME${RCol} exists."
      # compare API key with existing API key
      existing_api_key=$(cat "$CONFIGFILE")
      if [ "$existing_api_key" != "$API_KEY" ]; then
          printerror  "API key does not match existing API key. Current: ${Blu}$existing_api_key${RCol}, new: ${Blu}$API_KEY${RCol}."
          printindmsg "Either run ${Red}rm $CONFIGFILE${RCol} to delete the existing API key,"
          printindmsg "or run this command again with the current API key."
          exit 1
      else 
          printinfo "API key ${Blu}$API_KEY${RCol} matches existing."
      fi
  fi

}

#-------------------------------------------------------------------------------
#          NAME:  setup_tracer
#   DESCRIPTION:  Sets up binary with API key
#-------------------------------------------------------------------------------
setup_tracer() {
  output=$(tracer setup "$API_KEY" 2>&1)
  status_code=$?
  if [ $status_code -ne 0 ]; then
      printerror "Failed to configure Tracer binary. Output of tracer command:"

      while IFS= read -r line; do
          printindmsg "$line"
      done <<< "$output"

      exit 1
  fi

  printsucc "Tracer binary configured."
}

#-------------------------------------------------------------------------------
#          NAME:  update_rc
#   DESCRIPTION:  Ensures paths are configured for active shell
#-------------------------------------------------------------------------------
update_rc() {
  # TODO: the implementation in the fluent bit section needs to be combined with this one
  # check current shell
  if [ -n "$ZSH_VERSION" ]; then
    RC_FILE="$HOME/.zshrc"
  elif [ -n "$BASH_VERSION" ]; then
    RC_FILE="$HOME/.bashrc"
  else
    RC_FILE="$HOME/.bash_profile"
  fi

  # if custom bin dir had to be added to PATH, add it to .bashrc
  echo "export PATH=\$PATH:$BINDIR" >>"$RC_FILE"
  export PATH="$PATH:$BINDIR"
  printsucc "Added ${Blu}$BINDIR${RCol} to PATH variable in ${Blu}$RC_FILE${RCol} and added to current session."
}

#-------------------------------------------------------------------------------
#          NAME:  cleanup
#   DESCRIPTION:  Removes temporary directories and resets terminal
#-------------------------------------------------------------------------------
cleanup() {
    rm -rf "$TRACER_TEMP_DIR"
    if [ $? -ne 0 ]; then
        printerror "Failed to remove temporary directory ${Blu}$TRACER_TEMP_DIR${RCol}."
    fi
    printmsg ""
    printmsg ""
    printsucc "Temporary directory ${Blu}$TRACER_TEMP_DIR${RCol} removed."
    $ExitTrap
}

trap cleanup EXIT

#-------------------------------------------------------------------------------
#          NAME:  pause_before_continuing
#   DESCRIPTION:  Prompts user to press any key to continue for fluent-bit 
#-------------------------------------------------------------------------------
function pause_before_continuing() {
  printinfo "About to install Fluent Bit. This will require root privileges. "
  printindmsg "To inspect the script that's about to run, navigate here: "
  printindmsg ""
  printindmsg "${Blu}$FLUENT_BIT_INSTALL_SCRIPT${RCol}."
  printindmsg ""
  printindmsg "Press any key to continue, or ctrl-c to cancel"
  read -n 1 -s
}  

#-------------------------------------------------------------------------------
#          NAME:  send_event
#   DESCRIPTION:  Sends an event notification to a specified endpoint and logs 
#                 the response.
#-------------------------------------------------------------------------------
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

#-------------------------------------------------------------------------------
#          NAME:  install_fluent_bit
#   DESCRIPTION:  Uses fluent-bit vendor installer to deploy fluent-bit
#-------------------------------------------------------------------------------
install_fluent_bit() {
    # Using curl to download and execute the install script
    curl -sSL ${FLUENT_BIT_INSTALL_SCRIPT} | sh
}

#-------------------------------------------------------------------------------
#          NAME:  create_fluent_bit_config
#   DESCRIPTION:  Creates the Fluent Bit configuration content.
#-------------------------------------------------------------------------------
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

#-------------------------------------------------------------------------------
#          NAME:  update_bashrc_for_fluent_bit
#   DESCRIPTION:  Adds Fluent Bit to PATH in .bashrc
#-------------------------------------------------------------------------------
update_bashrc_for_fluent_bit() {
    
    local fluent_bit_path_entry="export PATH=$PATH:/opt/fluent-bit/bin/fluent-bit"
    # Check if the PATH update is already in .bashrc; if not, append it.
    if ! grep -qF -- "$fluent_bit_path_entry" ~/.bashrc; then
        printinfo "Adding $fluent_bit_path_entry to .bashrc."
        echo "$fluent_bit_path_entry" >>~/.bashrc
    else
        printinfo "Fluent Bit PATH already in .bashrc."
    fi

    # source config for subsequent commands
    $fluent_bit_path_entry
}

#-------------------------------------------------------------------------------
#          NAME:  configure_fluent_bit
#   DESCRIPTION:  Configures Fluent Bit with a dynamic API key.
#-------------------------------------------------------------------------------
configure_fluent_bit() {
    update_bashrc_for_fluent_bit

    printpinfo "Checking fluent-bit version..."
    fluent_bit_version=$(fluent-bit --version | grep -oP '^Fluent Bit v\K[\d.]+')
    printmsg "version ${Blu}$fluent_bit_version${RCol} found."
    required_version="3.0.0" # Set to a base comparison version that follows the semantic versioning pattern

    # Convert version numbers to a comparable format by padding shorter versions with zeros
    # This transforms version numbers into a format that can be directly compared
    ver_to_compare=$(echo "$fluent_bit_version" | awk -F. '{ printf("%d%03d%03d", $1, $2, $3); }')
    required_ver_compare=$(echo "$required_version" | awk -F. '{ printf("%d%03d%03d", $1, $2, $3); }')

    # Now compare the padded numbers
    if [ "$ver_to_compare" -le "$required_ver_compare" ]; then
        printerror "Fluent Bit version higher than 3.00 is required. Aborting."
        exit 1
    fi

    local config_path="/etc/fluent-bit/fluent-bit.conf"
    printinfo "Setting fluent-bit config in ${Blu}$config_path${RCol}."
    local config_content=$(create_fluent_bit_config)

    echo "$config_content" | sudo tee "$config_path" >/dev/null
}


#-------------------------------------------------------------------------------
#          NAME:  start_fluent_bit
#   DESCRIPTION:  Starts Fluent Bit in the background and echoes its PID.
#-------------------------------------------------------------------------------
start_fluent_bit() {
    printpinfo "Starting Fluent Bit..."
    fluent-bit -d -c /etc/fluent-bit/fluent-bit.conf >/dev/null 2>&1 &
    FLUENT_BIT_PID=$!
    printpmsg " started with PID: $FLUENT_BIT_PID."
}

#-------------------------------------------------------------------------------
#          NAME:  stop_fluent_bit_after_duration
#   DESCRIPTION:  Stops Fluent Bit after a specified duration in seconds. 
#-------------------------------------------------------------------------------
stop_fluent_bit_after_duration() {
    local duration=$1 # Duration in seconds before stopping Fluent Bit.
    printpinfo "Fluent Bit will be stopped automatically in $duration seconds..."
    sleep "$duration"

    if [[ -n $FLUENT_BIT_PID ]] && kill "$FLUENT_BIT_PID" 2>/dev/null; then
        echo "Fluent Bit has been stopped automatically after the timeout."
    else
        echo "Failed to stop Fluent Bit. It may have already been stopped, or the PID was incorrect."
    fi
}


#-------------------------------------------------------------------------------
#          NAME:  main
#   DESCRIPTION:  The main function
#-------------------------------------------------------------------------------
main() {

    print_header
    check_args "$@"
    configure_tracer
    check_os
    check_prereqs
    get_package_name
    configure_bindir

    send_event "start_installation" "Start Tracer installation for key: ${API_KEY}"
    make_temp_dir
    download_tracer
    setup_tracer

    printsucc "Tracer CLI has been successfully installed."
    send_event "finished_installation" "Successfully installed Tracer for key: ${API_KEY}"

    pause_before_continuing

    # fluentbit installation
    send_event "start_installation" "Start Fluent Bit installation for key: ${API_KEY}"
    
    printnolog " " # newline
    printinfo "Starting Fluent Bit installation."

    # paint all the foreign output blue
    printnolog "${Blu}"
    install_fluent_bit
    printnolog "${RCol}"

    configure_fluent_bit
    update_bashrc_for_fluent_bit

    # start fluent-bit
    start_fluent_bit
    send_event "finished_installation" "Successfully installed Fluent Bit for key: ${API_KEY}"
    stop_fluent_bit_after_duration 604800 # 1 week
    send_event "finished_installation" "Fluent Bit has been stopped"
}

main "$@"
