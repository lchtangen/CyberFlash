#!/bin/bash

echo "=== OnePlus 7 Pro Connection Test ==="
echo ""

echo "Testing ADB connection..."
adb devices
echo ""

echo "Testing Fastboot connection..."
fastboot devices
echo ""

echo "If your device appears above, connection is working!"
