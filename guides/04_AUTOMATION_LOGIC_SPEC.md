# ⚙️ Automation Logic Specification: The "Auto-Flash" Engine

**Version**: 2.0.0 | **Based On**: OnePlus 7 Pro crDroid 12 Manual Guide | **Status**: Engineering Draft

---

## 🎯 Objective
Translate the manual **8-Phase Flashing Process** into autonomous Rust/Tauri logic. The app must perform these steps without user intervention where possible, or guide them interactively.

---

## 🔄 Phase 1: Asset Acquisition (The Downloader)

**Manual Task**: User downloads 4 specific files.
**Automation Logic**:
1.  **Manifest Fetch**: App fetches `config/downloads.json` (remote) to get latest URLs.
2.  **Checksum Verification**: Rust calculates SHA256 of downloaded files.
3.  **File Naming**: Renames files to standard internal names (e.g., `firmware.zip`, `recovery.img`).

### Rust Struct
```rust
struct FlashAssets {
    rom: PathBuf,      // crDroid_12_v12.3.zip
    firmware: PathBuf, // Firmware_H.41.zip
    recovery: PathBuf, // twrp-3.7.1.img
    magisk: Option<PathBuf>,
}
```

---

## 🔄 Phase 2 & 3: Device Prep & Unlock

**Manual Task**: Enable ADB, `fastboot oem unlock`.
**Automation Logic**:
1.  **State Detection**:
    *   Check `adb devices`. If empty, prompt user for USB Debugging.
    *   Check `adb shell getprop ro.boot.flash.locked`.
2.  **Unlock Sequence**:
    *   If locked: `adb reboot bootloader` -> `fastboot oem unlock`.
    *   **UI Prompt**: "Look at your phone. Press Vol Down, then Power."

---

## 🔄 Phase 4: Recovery Injection

**Manual Task**: `fastboot flash recovery_a/b ...`
**Automation Logic**:
1.  **Slot Detection**: `fastboot getvar current-slot`.
2.  **Dual Slot Flash**:
    ```rust
    Command::new("fastboot").args(&["flash", "recovery_a", path_to_twrp]);
    Command::new("fastboot").args(&["flash", "recovery_b", path_to_twrp]);
    ```
3.  **Boot Recovery**: `fastboot boot path_to_twrp`.

---

## 🔄 Phase 5: Firmware H.41 (CRITICAL)

**Manual Task**: Sideload H.41 zip in TWRP.
**Automation Logic**:
1.  **Mode Check**: Verify device is in `recovery` or `sideload` mode.
    *   If `recovery`: `adb shell twrp sideload`.
2.  **Sideload Execution**:
    *   `adb sideload firmware.zip`.
    *   **Progress Parsing**: Parse `serving: 'firmware.zip' (~45%)` from stdout to update UI progress bar.

---

## 🔄 Phase 6: The Great Wipe

**Manual Task**: Advanced Wipe + Format Data.
**Automation Logic**:
1.  **Scripted Wipe**:
    *   Instead of UI touches, use `adb shell twrp wipe data`, `adb shell twrp wipe system`, etc.
2.  **Format Data**:
    *   `adb shell twrp format data`.
    *   *Note*: This might reboot recovery. Handle reconnection logic.

---

## 🔄 Phase 7: ROM Installation

**Manual Task**: Sideload crDroid zip.
**Automation Logic**:
1.  **Re-enable Sideload**: `adb shell twrp sideload`.
2.  **Stream Sideload**: `adb sideload crdroid.zip`.
    *   **AI Monitor**: Watch for "Error 7" or "Status 1". If detected, abort and trigger "AI Error Recovery".

---

## 🔄 Phase 8: Root & Post-Install

**Manual Task**: Sideload Magisk, Reboot.
**Automation Logic**:
1.  **Optional Step**: If user checked "Root" in UI.
2.  **Sideload**: `adb sideload magisk.apk`.
3.  **Final Reboot**: `adb reboot`.

---

## 🧩 Error Handling Matrix

| Error | Detection | Automated Fix |
|-------|-----------|---------------|
| **Device Not Found** | `List of devices attached` is empty | Reset ADB Server, Show "Check Cable" UI |
| **Sideload Fail** | `protocol fault (no status)` | Restart ADB, Re-enter Sideload Mode |
| **Verification Fail** | `signature verification failed` | Prompt user to disable verification in TWRP or re-download |
