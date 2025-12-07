# 🔧 Troubleshooting Guide

**Version**: 2.0.0 | **Updated**: December 7, 2025

---

## 🔌 ADB & Device Connection

### Issue: "Device not found"
**Symptoms**: The device list is empty in the app.
**Solutions**:
1.  **Check USB Debugging**: Ensure "USB Debugging" is enabled in Developer Options on the phone.
2.  **Check Cable**: Use the original OnePlus cable. Try a different USB port (USB 2.0 ports are often more stable for flashing).
3.  **Restart ADB Server**:
    ```bash
    adb kill-server
    adb start-server
    ```
4.  **Check Permissions (Linux)**: Ensure you have `udev` rules set up.
    ```bash
    # /etc/udev/rules.d/51-android.rules
    SUBSYSTEM=="usb", ATTR{idVendor}=="2d01", MODE="0666", GROUP="plugdev"
    ```

### Issue: "Unauthorized"
**Symptoms**: Device shows up but status is "Unauthorized".
**Solutions**:
1.  Check your phone screen. A popup asking "Allow USB debugging?" should appear.
2.  Check "Always allow from this computer" and tap "Allow".
3.  If no popup, revoke USB debugging authorizations in Developer Options and reconnect.

---

## 🏗️ Build & Development

### Issue: "Rust dependency error"
**Symptoms**: `cargo build` fails.
**Solutions**:
1.  Update Rust: `rustup update`.
2.  Clean build cache: `cargo clean`.
3.  Ensure system dependencies are installed (see Framework Guide).

### Issue: "WebView2 not found" (Windows/Linux)
**Symptoms**: App launches but shows a white screen or error.
**Solutions**:
1.  **Linux**: Ensure `libwebkit2gtk-4.0-dev` is installed.
2.  **Logs**: Run with verbose logging: `RUST_LOG=debug npm run tauri dev`.

---

## 🤖 AI & Gemini

### Issue: "API Key Invalid"
**Symptoms**: AI features return 401/403 errors.
**Solutions**:
1.  Check `.env` file for `GEMINI_API_KEY`.
2.  Ensure the key has permissions for the `gemini-pro` model.
3.  Verify you haven't exceeded your quota.

### Issue: "Response Timeout"
**Symptoms**: AI takes too long to reply.
**Solutions**:
1.  Check your internet connection.
2.  The Rust backend has a 30s timeout by default. Complex queries might need longer.
