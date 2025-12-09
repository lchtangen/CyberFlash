# CyberFlash V2: OnePlus 7 Pro Automation Protocol
> **Based on**: `REFERENCE_ONEPLUS_7PRO_MANUAL.md` (v1.2 Final)
> **Target**: OnePlus 7 Pro (`guacamole` / `guacamoleb`)
> **Goal**: NetHunter/OxygenOS → crDroid 12 (Android 16)

This document outlines how the **CyberFlash FlashWizard** automates the exact procedure defined in the reference manual. Each phase in the software corresponds 1:1 with the manual's critical steps.

---

## 🔄 Phase Mapping & Execution Logic

### Phase 1: Pre-Flight Analysis
**Manual Reference**: `SAFETY & PREREQUISITES`
- **Manual Step**: Check Battery ≥ 50%, Verify ADB Connection, Verify Device Model.
- **Automation**:
    - `adb devices`: Verifies connection state.
    - `dumpsys battery`: Enforces >50% charge rule.
    - `getprop ro.product.device`: Ensures device is `guacamole` or `guacamoleb`.

### Phase 2: Environment Prep
**Manual Reference**: `PHASE 1: DOWNLOAD ALL REQUIRED FILES`
- **Manual Step**: Download ROM, Firmware H.41, TWRP 3.7.1, Magisk v27.0.
- **Automation**:
    - Fetches inventory from `config/oneplus7pro_inventory.json`.
    - **Downloads**:
        1.  `crDroidAndroid-16.0-20251129-guacamole-v12.3.zip` (Android 16)
        2.  `OPM7.181205.001_H41.zip` (Firmware H.41 - **MANDATORY**)
        3.  `twrp-3.7.1_12-0-guacamole.img` (Recovery)
        4.  `Magisk-v27.0.apk` (Root)
    - **Verification**: Performs MD5/SHA256 checksum validation on all files.

### Phase 3: Smart Data Preservation
**Manual Reference**: `PHASE 2: DEVICE PREPARATION` (Step 4)
- **Manual Step**: Backup essential data (`/sdcard/DCIM`, etc).
- **Automation**:
    - `adb shell pm list packages`: Backs up list of installed apps.
    - `adb pull`: Backs up DCIM and critical folders to host machine.

### Phase 4: Bootloader Operations
**Manual Reference**: `PHASE 3: BOOTLOADER UNLOCK`
- **Manual Step**: Reboot to bootloader, `fastboot oem unlock`.
- **Automation**:
    - `adb reboot bootloader`: Enters Fastboot mode.
    - `fastboot getvar unlocked`: Checks status.
    - `fastboot oem unlock`: Triggers unlock prompt (User interaction required on device).

### Phase 5: Slot Management
**Manual Reference**: `PHASE 4: TWRP RECOVERY INSTALLATION` (Prep)
- **Manual Step**: Ensure consistent slot state.
- **Automation**:
    - `fastboot getvar current-slot`: Detects active slot.
    - `fastboot --set-active=a`: Standardizes installation to Slot A for consistency (optional but recommended for clean flash).
    - `fastboot erase boot`: Cleans old boot partition.

### Phase 6: Recovery & Firmware
**Manual Reference**: `PHASE 4` & `PHASE 5`
- **Manual Step**: Flash TWRP, Reboot Recovery, Sideload Firmware H.41.
- **Automation**:
    1.  **Flash TWRP**: `fastboot flash recovery_a twrp-3.7.1...img` & `recovery_b`.
    2.  **Reboot**: `fastboot reboot recovery`.
    3.  **Sideload H.41**:
        - Prompts user to enable ADB Sideload in TWRP.
        - Executes `adb sideload OPM7.181205.001_H41.zip`.
        - **CRITICAL**: This ensures Touch/WiFi/Sensors work on Android 16.

### Phase 7: ROM Installation
**Manual Reference**: `PHASE 6` & `PHASE 7`
- **Manual Step**: Wipe Data, Format Data, Sideload crDroid 12.
- **Automation**:
    1.  **Wipe**: Prompts user to "Format Data" in TWRP (Encryption removal).
    2.  **Sideload ROM**:
        - Re-enables ADB Sideload.
        - Executes `adb sideload crDroidAndroid-16.0...zip`.
    3.  **Sideload GApps** (Optional): Sideloads NikGapps if selected.

### Phase 8: Security Patching (Retention)
**Manual Reference**: `PHASE 7` (Step 9)
- **Manual Step**: Update TWRP to FBEv2 (Retention).
- **Automation**:
    - **Re-flash TWRP**: Sideloads `twrp-3.7.1_12-0-guacamole.img` again after ROM flash.
    - This prevents the stock ROM recovery from overwriting TWRP and ensures FBEv2 compatibility.

### Phase 9: Root & Extensions
**Manual Reference**: `PHASE 8: POST-INSTALLATION`
- **Manual Step**: Install Magisk APK.
- **Automation**:
    1.  **Reboot System**: `adb reboot`.
    2.  **Wait for Boot**: Polls for ADB connection (Android 16 boot).
    3.  **Push APK**: `adb push Magisk-v27.0.apk /sdcard/Download/`.
    4.  **Instruction**: Guides user to install APK via Files app and run "Direct Install".

### Phase 10: Finalization
**Manual Reference**: `SUCCESS CHECKLIST`
- **Manual Step**: Verify Version, WiFi, Root.
- **Automation**:
    - Displays success summary.
    - Verifies `ro.build.version.release` == `16`.
    - Verifies `ro.product.device` == `guacamole`.

---

## ⚠️ Critical Deviations / Notes
*   **Format Data**: The Wizard cannot automate the `yes` confirmation in TWRP "Format Data". This remains a manual user interaction step, prompted by the UI.
*   **Sideload Mode**: The Wizard relies on the user tapping "Advanced -> ADB Sideload" in TWRP when prompted, as TWRP does not support remote sideload triggering from normal recovery mode.

## 📋 File Inventory (Configured)
The Wizard uses the exact files specified in the manual:
*   **ROM**: `crDroidAndroid-16.0-20251129-guacamole-v12.3.zip`
*   **Firmware**: `OPM7.181205.001_H41.zip`
*   **Recovery**: `twrp-3.7.1_12-0-guacamole.img`
*   **Root**: `Magisk-v27.0.apk`
