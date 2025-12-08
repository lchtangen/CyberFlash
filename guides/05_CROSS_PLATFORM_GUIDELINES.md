# 🌐 Cross-Platform App Guidelines (2025-2026)

**Version**: 1.0.0 | **Target OS**: Android 16, macOS Sequoia, Windows 11, Linux (Wayland)

---

## 📱 Android 15/16 Compatibility

### 1. Scoped Storage & Permissions
*   **Requirement**: Apps cannot access `/sdcard` directly.
*   **Solution**: Use `Storage Access Framework` (SAF) or ADB push/pull to `/data/local/tmp` for temporary operations.
*   **Target SDK**: Must target API Level 35 (Android 15).

### 2. A/B Partition Scheme (Virtual A/B)
*   **Requirement**: Support "Seamless Updates" structure.
*   **Logic**: Always flash to *both* slots (`_a` and `_b`) or the inactive slot depending on the operation.
*   **Encryption**: Support FBEv2 (File-Based Encryption) metadata parsing.

---

## 🍎 macOS (Ventura/Sequoia) Guidelines

### 1. Hardened Runtime & Notarization
*   **Requirement**: All binaries must be signed and notarized by Apple.
*   **Entitlements**:
    ```xml
    <key>com.apple.security.device.usb</key>
    <true/>
    <key>com.apple.security.network.client</key>
    <true/>
    ```
*   **USB Access**: macOS 15+ requires explicit user permission for USB accessories to connect. The app must handle the "Allow Accessory to Connect" system prompt gracefully.

### 2. Universal Binaries
*   **Requirement**: Must compile for both `x86_64` (Intel) and `arm64` (Apple Silicon).
*   **Tauri Config**: `bundle.targets: ["all"]`.

---

## 🪟 Windows 10/11 Guidelines

### 1. Driver Management
*   **Requirement**: ADB/Fastboot requires specific drivers (Google USB Driver or OnePlus Driver).
*   **Automation**: The app should bundle `Check-Driver.ps1` to verify if `WinUSB` is loaded for the device VID/PID (`0x2A70` for OnePlus).
*   **WebView2**: Ensure the runtime is present (pre-installed on Win11, may need bootstrap on Win10).

### 2. Execution Policy
*   **Requirement**: PowerShell scripts often fail due to `Restricted` policy.
*   **Solution**: Run internal scripts via Rust `Command` directly, avoiding `.ps1` files where possible, or use `-ExecutionPolicy Bypass`.

---

## 🐧 Linux (Arch, Debian, Ubuntu) Guidelines

### 1. Udev Rules (The #1 Blocker)
*   **Requirement**: Non-root users cannot access USB devices by default.
*   **Solution**: The AppImage should include a "Setup Wizard" that detects missing permissions and offers to write `/etc/udev/rules.d/51-android.rules`.
    ```bash
    SUBSYSTEM=="usb", ATTR{idVendor}=="2a70", MODE="0666", GROUP="plugdev"
    ```

### 2. Wayland Support
*   **Requirement**: Ubuntu 24.04+ and Fedora use Wayland by default.
*   **Tauri**: Ensure `WEBKIT_DISABLE_COMPOSITING_MODE=1` is *not* needed, or handle GPU acceleration flags correctly.

---

## 🎨 UI/UX Standards (2026 Era)

1.  **Dark Mode First**: Default to dark theme (OLED black `#000000` or deep gray `#121212`).
2.  **Adaptive Input**: Support Touch (Windows tablets), Mouse, and Keyboard shortcuts (`Cmd+K` palettes).
3.  **Haptic Feedback**: Trigger haptics on the *phone* via ADB when a desktop action completes (e.g., `adb shell cmd vibrator vibrate 50`).
