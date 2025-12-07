#!/bin/bash
# Script to run Tauri dev environment inside VS Code Snap
# This unsets conflicting environment variables that cause crashes with libpthread/WebKit

echo "Setting up clean environment for VS Code Snap..."

# Reset XDG_DATA_DIRS to system defaults (ignoring Snap paths)
export XDG_DATA_DIRS=/usr/local/share:/usr/share:/var/lib/snapd/desktop

# Unset Snap/GTK specific variables
unset GTK_PATH
unset GTK_EXE_PREFIX
unset GIO_MODULE_DIR
unset GDK_PIXBUF_MODULE_FILE
unset LD_LIBRARY_PATH

echo "Cleaning up previous sessions..."
# Kill any process running on port 1420
fuser -k 1420/tcp || true

# Clean cargo build artifacts to prevent permission errors
echo "Cleaning cargo build artifacts..."
rm -rf src-tauri/target/debug/build/temp-app*
rm -rf src-tauri/target/debug/build/tauri*

echo "Starting Tauri Dev..."
npm run tauri dev
