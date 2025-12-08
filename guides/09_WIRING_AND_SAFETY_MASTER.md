# 🔌 Master Guide: Wiring, Environment & Safety

**Version**: 1.1.0 | **Status**: In Progress | **Target**: Production Readiness

This document tracks the transition of CyberFlash V2 from a "Simulated Prototype" to a "Live Production Tool". It reflects the current state of the codebase and outlines the remaining steps.

---

## 🔌 Pillar 1: The Great Wiring (Simulation → Reality)

We have successfully moved from `thread::sleep` mocks in `src-tauri/src/commands/automation.rs` to real engine calls.

### 1.1 Phase 1: Downloads (The Supply Chain)
**Status**: 🟢 Verified
**Current State**: `run_phase_1` in `automation.rs` loads `config/downloads.json`, iterates through files, downloads them with real progress tracking, and verifies checksums.
**Pending**:
- [x] Verify `http_client::download_file` handles large files and progress reporting correctly.
- [x] Implement real URL fetching from `config/downloads.json`.

### 1.2 Phase 2: Backup (The Safety Net)
**Status**: 🟢 Verified
**Current State**: `run_phase_2` checks for ADB devices and executes `adb backup` for SMS.
**Pending**:
- [x] Uncomment and verify `adb::adb_backup` in `automation.rs`.
- [x] Ensure `adb_backup` handles user prompts on the device (UI message added).

### 1.3 Phase 3: Bootloader Unlock (The Gatekeeper)
**Status**: 🟢 Verified
**Current State**: `run_phase_3` checks `fastboot getvar unlocked`. If locked, it sends `fastboot flashing unlock`.
**Pending**:
- [x] Implement `run_phase_3` in `automation.rs`.
- [x] Wire up `fastboot::get_var("unlocked")` and `fastboot::oem_unlock()`.

### 1.4 Phases 4-8: Flashing & Post-Install
**Status**: 🟢 Verified
**Current State**:
- **Phase 4 (Firmware)**: Iterates through critical partitions (`modem`, `abl`, etc.) and flashes `.img` files from downloads.
- **Phase 5 (Recovery)**: Flashes `recovery.img` to the recovery partition.
- **Phase 6 (ROM)**: Identifies the ROM zip from `downloads.json` and performs `adb sideload`.
**Pending**:
- [x] Implement `run_phase_4` (Firmware) -> `fastboot::flash_partition`.
- [x] Implement `run_phase_5` (Recovery) -> `fastboot::flash_partition`.
- [x] Implement `run_phase_6` (ROM) -> `adb::sideload`.

---

## 🖥️ Pillar 2: Desktop Environment Setup

Ensuring the app runs on any Linux/macOS/Windows machine without pre-installed tools.

### 2.1 Binary Bundling (Sidecars)
**Status**: 🟢 Implemented
**Objective**: Bundle `adb` and `fastboot` binaries inside the app.

**Action Plan**:
1.  **Download Binaries**: Get platform-tools for Linux, macOS, and Windows.
2.  **Rename**: Follow Tauri convention (e.g., `adb-x86_64-unknown-linux-gnu`).
3.  **Place**: Put them in `src-tauri/bin/`.
4.  **Config**: Update `src-tauri/tauri.conf.json`:
    ```json
    "bundle": {
      "externalBin": ["adb", "fastboot"]
    }
    ```
5.  **Code Update**: Update `src-tauri/src/commands/adb.rs` and `fastboot.rs` to use `tauri::plugin::shell` sidecar logic if not using system path.

### 2.2 Linux Permissions (Udev Rules)
**Status**: 🔴 Not Started
**Objective**: Allow non-root USB access on Linux.

**Action Plan**:
*   **Script**: Create `scripts/install_udev_rules.sh`.
*   **UI Integration**: Add check in `DriverHealthCheck.vue`.

---

## 🛡️ Pillar 3: Validation & Safety Protocols

Preventing the app from running dangerous commands in unsafe conditions.

### 3.1 "Dry Run" Mode (Simulation Toggle)
**Status**: 🔴 Not Started
**Objective**: Allow developers/users to test the UI without a device.

**Action Plan**:
1.  **Config**: Add `dry_run: boolean` to `config/app-config.yaml`.
2.  **Backend Logic**: In `automation.rs`, check this flag before executing real commands.

### 3.2 Device Fingerprinting (Anti-Brick)
**Status**: 🔴 Not Started
**Objective**: Ensure we never flash the wrong ROM to the wrong phone.

**Action Plan**:
*   **Frontend**: In `FlashWizard.vue`, before `startPhase()`:
    ```typescript
    if (deviceStore.deviceModel !== targetDevice) {
        // Block action
    }
    ```

### 3.3 Battery Safety
**Status**: 🔴 Not Started
**Objective**: Prevent flashing if battery < 40%.

**Action Plan**:
*   **Backend**: Add check in `automation.rs` before starting critical phases.
    ```rust
    let level = adb::get_battery_level()?;
    if level < 40 {
        return Err("Battery too low".into());
    }
    ```

---

## ✅ Execution Checklist

- [x] **Wiring**:
    - [x] Phase 1 (Downloads) - Basic wiring done.
    - [x] Phase 2 (Backup) - Enable `adb_backup`.
    - [x] Phase 3 (Unlock) - Implement `run_phase_3`.
    - [x] Phase 4-6 (Flash) - Implement remaining phases.
- [ ] **Environment**:
    - [x] Binaries downloaded and renamed in `src-tauri/bin/`.
    - [x] `tauri.conf.json` updated with `externalBin`.
    - [ ] `install_udev_rules.sh` created.
- [ ] **Safety**:
    - [ ] `dry_run` toggle implemented.
    - [ ] Battery check implemented in `automation.rs`.
    - [ ] Device mismatch check implemented in `FlashWizard.vue`.
