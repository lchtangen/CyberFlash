#!/bin/bash
# Script to run Tauri dev environment inside VS Code Snap
# This unsets conflicting environment variables that cause crashes with libpthread/WebKit

echo "Cleaning up previous sessions..."
fuser -k -9 1420/tcp 2>/dev/null

echo "Starting Tauri Dev with clean environment..."

# Capture necessary variables
USER_HOME=$HOME
USER_PATH=$PATH
USER_DISPLAY=$DISPLAY
USER_XAUTH=$XAUTHORITY
USER_DBUS=$DBUS_SESSION_BUS_ADDRESS

# Run with clean environment
# We strip all environment variables and only pass the essentials
# This removes GTK_PATH, LD_LIBRARY_PATH, and other Snap-injected variables
env -i HOME="$USER_HOME" \
       PATH="$USER_PATH" \
       DISPLAY="$USER_DISPLAY" \
       XAUTHORITY="$USER_XAUTH" \
       DBUS_SESSION_BUS_ADDRESS="$USER_DBUS" \
       npm run tauri dev
