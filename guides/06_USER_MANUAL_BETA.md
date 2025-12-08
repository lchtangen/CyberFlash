# 📖 CyberFlash Tool: User Manual (Beta)

**Version**: 0.9.0 (Beta) | **Device**: OnePlus 7 Pro | **OS**: Windows, macOS, Linux

---

## 👋 Welcome to CyberFlash

CyberFlash is an autonomous ROM flashing tool designed to install **crDroid 12 (Android 16)** on your OnePlus 7 Pro. It replaces complex terminal commands with a simple, guided interface.

---

## 🚀 Getting Started

### Prerequisites
1.  **OnePlus 7 Pro** (Any model: GM1910, GM1911, GM1913, GM1917).
2.  **Battery**: Charged to at least 50%.
3.  **USB Cable**: Original Red cable or high-quality USB-C data cable.
4.  **Backup**: All data on the phone **WILL BE ERASED**.

### Installation
*   **Windows**: Run `CyberFlash-Setup.exe`.
*   **macOS**: Drag `CyberFlash.app` to Applications.
*   **Linux**: Make executable (`chmod +x`) and run the `.AppImage`.

---

## 🎮 Using the Dashboard

### 1. Connection Status
The top-left panel shows your device status.
*   🔴 **Disconnected**: Plug in your phone.
*   🟡 **Unauthorized**: Check your phone screen and tap "Allow USB Debugging".
*   🟢 **Connected**: Ready to flash.

### 2. The "Auto-Flash" Wizard
Click the **"Start Flashing"** button to begin the 8-step process.

#### Step 1: Download Assets
The app will automatically download:
*   `crDroid 12.3` (Android 16)
*   `Firmware H.41` (Mandatory)
*   `TWRP 3.7.1`
*   `Magisk` (Optional)

#### Step 2: Unlock Bootloader
*   The app will reboot your phone to **Fastboot Mode**.
*   **Action Required**: When you see the text on your phone screen, use **Volume Keys** to select "UNLOCK THE BOOTLOADER" and press **Power**.
*   *Warning*: This wipes your data immediately.

#### Step 3: Firmware & Recovery
The app will automatically:
1.  Flash TWRP Recovery.
2.  Reboot into Recovery.
3.  Sideload **Firmware H.41**.
    *   *Why?* This fixes touch screen, WiFi, and sensors for Android 16.

#### Step 4: The Wipe
The app will command the phone to format the storage.
*   *Note*: Encryption is removed to ensure the new ROM can boot.

#### Step 5: Installing crDroid
The main event. The app streams the 1.1GB ROM file to your phone.
*   **Do not unplug** during this process.

---

## 🤖 AI Assistant (Gemini)

Stuck? Click the **"Ask AI"** button or press `Cmd+K`.
*   *Query*: "My phone is stuck on the OnePlus logo."
*   *AI Response*: "It seems you are in a bootloop. This often happens if the cache wasn't wiped. I can run a 'Wipe Cache' command for you. Shall I proceed?"

---

## ❓ FAQ

**Q: Can I relock the bootloader?**
A: **NO**. Relocking the bootloader with a custom ROM (crDroid) installed will **BRICK** your device. Only relock if you return to stock OxygenOS.

**Q: Do I need to download files manually?**
A: No, CyberFlash handles all downloads to ensure you get the correct versions (H.41 firmware, etc.).

**Q: Is this safe?**
A: Flashing always carries risk. CyberFlash includes safety checks (battery level, model verification) to minimize this risk, but you proceed at your own responsibility.
