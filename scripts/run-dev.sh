#!/bin/bash
# Script to run Tauri dev environment inside VS Code Snap
# This unsets conflicting environment variables that cause crashes with libpthread/WebKit

echo "Setting up clean environment for VS Code Snap..."

# Reset XDG_DATA_DIRS to system defaults (ignoring Snap paths)
export XDG_DATA_DIRS=/usr/local/share:/usr/share:/var/lib/snapd/desktop
export GSETTINGS_SCHEMA_DIR=/usr/share/glib-2.0/schemas

# Unset Snap/GTK specific variables
unset GTK_PATH
unset GTK_EXE_PREFIX
unset GIO_MODULE_DIR
unset GDK_PIXBUF_MODULE_FILE
unset GTK_MODULES
unset GTK_IM_MODULE_FILE
unset GDK_BACKEND
unset XDG_DATA_HOME
unset LD_PRELOAD

# Force LD_LIBRARY_PATH to system paths to avoid Snap libraries
export LD_LIBRARY_PATH=/usr/lib/x86_64-linux-gnu:/lib/x86_64-linux-gnu:/usr/lib:/lib

echo "Cleaning up previous sessions..."
# Kill any process running on port 1420
fuser -k 1420/tcp || true

# Clean cargo build artifacts to prevent permission errors
echo "Cleaning cargo build artifacts..."
rm -rf src-tauri/target/debug/build/temp-app*
rm -rf src-tauri/target/debug/build/tauri*
rm -f src-tauri/target/debug/cyberflash-v2

echo "Starting Tauri Dev..."
npm run tauri dev
