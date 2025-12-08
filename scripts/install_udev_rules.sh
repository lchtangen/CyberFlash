#!/bin/bash
# Script to install Android Udev Rules for Linux
# This fixes "no permissions" or "device not found" issues.

echo "🤖 Installing Android Udev Rules..."

RULES_FILE="/etc/udev/rules.d/51-android.rules"

# Check for root
if [ "$EUID" -ne 0 ]; then 
  echo "❌ Please run as root (sudo)."
  exit 1
fi

echo "📝 Writing rules to $RULES_FILE..."

cat > $RULES_FILE <<EOF
# OnePlus
SUBSYSTEM=="usb", ATTR{idVendor}=="2a70", MODE="0666", GROUP="plugdev"
# Google
SUBSYSTEM=="usb", ATTR{idVendor}=="18d1", MODE="0666", GROUP="plugdev"
# Qualcomm (EDL/Download Mode)
SUBSYSTEM=="usb", ATTR{idVendor}=="05c6", MODE="0666", GROUP="plugdev"
# General Android
SUBSYSTEM=="usb", ATTR{idVendor}=="2a70", MODE="0666", GROUP="plugdev"
EOF

echo "🔄 Reloading udev rules..."
udevadm control --reload-rules
udevadm trigger

echo "🔄 Restarting ADB server..."
# Kill any running adb server (system or bundled)
killall adb 2>/dev/null

echo "✅ Done! Please unplug and replug your device."
echo "   Make sure to check your phone screen for the 'Allow USB Debugging' prompt."
