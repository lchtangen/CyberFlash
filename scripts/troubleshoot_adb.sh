#!/bin/bash

echo "=== ADB Connection Troubleshooting ==="
echo ""

echo "1. Checking USB devices..."
lsusb
echo ""

echo "2. Checking for Qualcomm devices..."
lsusb | grep -i qualcomm
echo ""

echo "3. Checking ADB daemon status..."
adb status-usb
echo ""

echo "4. Trying to detect devices with verbose output..."
adb devices -l
echo ""

echo "5. Checking if device is in bootloader/fastboot..."
fastboot devices -l
echo ""

echo "6. Attempting to get props (without rooting)..."
adb shell getprop ro.product.model 2>/dev/null || echo "Device not accessible via ADB"
echo ""

echo "=== Troubleshooting Tips ==="
echo "✓ Run 'sudo ./scripts/install_udev_rules.sh' to fix permissions"
echo "✓ Make sure USB Debugging is enabled on phone"
echo "✓ Make sure Developer Options are enabled on phone"
echo "✓ Check the USB permission prompt on phone and allow it"
echo "✓ Try a different USB cable (preferably USB 3.0)"
echo "✓ Try a different USB port on the PC"
echo "✓ The device may be in Fastboot/Bootloader mode - check screen"
