# 🚀 CyberFlash V2: Master Roadmap (The "Real-Time" Evolution)

This document outlines the 10-Phase strategy to transition CyberFlash V2 from "Functional Prototype" to "Production-Grade Powerhouse". It addresses the user's requirement to replace all mock data with live, real-time systems and introduces 10 critical new features.

---

##  Phase 1: The "Live" Foundation (Activity & Persistence)
**Goal**: Replace hardcoded `RecentActivity` and ensure every action is logged to a persistent local database.

1.  [x] **Backend**: Implement `sqlite` or `sled` database in Rust for persistent history.
2.  [x] **Store**: Create `useHistoryStore` in Vue to sync with the backend.
3.  [x] **Component**: Rewrite `RecentActivity.vue` to use `useHistoryStore`.
4.  [x] **Hook**: Add global interceptors to `adb.rs` and `fastboot.rs` to auto-log successful commands.
5.  [x] **UI**: Add "Clear History" and "Export Logs" to the Activity widget.
6.  [x] **Feature**: Implement "Session Resume" (remember last connected device and view).
7.  [x] **Real-time**: Add WebSocket or Event listener for database changes to update UI instantly.
8.  [x] **Detail View**: Create a modal to view full command output for any history item.
9.  [x] **Filter**: Add search/filter by device serial or command type.
10. [x] **Cleanup**: Remove all mock data arrays from `RecentActivity.vue`.

## Phase 2: The "Real-Time" Flashing Engine
**Goal**: Make `FlashView` a true streaming interface with precise progress bars and error handling.

1.  [x] **Backend**: Implement `AsyncRead` streaming for Fastboot output in `flash.rs`.
2.  [x] **Protocol**: Define a custom Event protocol (`flash:progress`, `flash:partition`, `flash:error`).
3.  [x] **UI**: Create a `TerminalStream` component that renders Fastboot stdout line-by-line in real-time.
4.  [x] **Logic**: Implement "Slot Detection" (A/B) before flashing to prevent bricking.
5.  [ ] **Feature**: Add "Pause/Resume" support for multi-stage scripts.
6.  [ ] **Safety**: Implement MD5 checksum verification before flashing starts.
7.  [ ] **UX**: Add "Time Remaining" estimation based on transfer speed.
8.  [ ] **Recovery**: Add "Emergency Stop" button that attempts to cancel safely.
9.  [ ] **Post-Flash**: Auto-trigger "Reboot to System" dialog upon success.
10. [ ] **Automation**: Convert `flash_all.bat` parsing to a native Rust execution graph.

## Phase 3: Hardware Intelligence (Real Sensors)
**Goal**: Replace `HardwareView` placeholders with actual `dumpsys` and sensor data.

1.  [x] **Backend**: Implement `get_battery_health` (cycles, capacity, voltage) via `dumpsys battery`.
2.  [x] **Backend**: Implement `get_storage_info` (eMMC/UFS life, partition usage).
3.  [x] **Backend**: Implement `get_sensor_list` to detect available hardware sensors.
4.  [x] **UI**: Build `SensorGrid.vue` to show live accelerometer/gyroscope data (if ADB allows).
5.  [ ] **Feature**: Add "Screen Tester" (cycle colors on device to check for dead pixels).
6.  [ ] **Feature**: Add "Touch Tester" (visualize touch inputs).
7.  [ ] **Logic**: Add "Device Health Score" algorithm based on battery/storage metrics.
8.  [ ] **Export**: Generate HTML "Device Health Report".
9.  [ ] **Alerts**: Notification if battery temp exceeds safe threshold during usage.
10. [ ] **Compat**: Handle devices that don't support standard `dumpsys` commands.

## Phase 4: Community Pulse (Real Feeds)
**Goal**: Connect `CommunityFeed` and `SocialView` to real external APIs.

1.  [x] **Backend**: Add `rss` crate to `Cargo.toml`.
2.  [x] **Backend**: Implement `fetch_rss_feed(url)` command in `social.rs`.
3.  [x] **UI**: Update `CommunityFeed.vue` to fetch from XDA, Android Police, and LineageOS feeds.
4.  [x] **UI**: Add "Source Filter" pills (All, XDA, Reddit) to the feed widget.
5.  [ ] **Feature**: Implement "One-Click Download" if the RSS item contains a `.zip` link.
6.  [ ] **Feature**: Add "GitHub Releases" integration for specific repo tracking.
7.  [ ] **Cache**: Implement local caching of feeds to reduce network requests.
8.  [ ] **Share**: Add "Share Config" button that generates a deep link `cyberflash://share/...`.
9.  [ ] **Profile**: Create a "Dev Profile" view that fetches GitHub user data.
10. [ ] **Search**: Add global search across all connected community sources.

## Phase 5: Automation & Scripting (The "Correct Functions")
**Goal**: Enable users to create and run complex multi-step workflows.

1.  [x] **Engine**: Create `AutomationEngine` in Rust to execute JSON-based task lists.
2.  [x] **UI**: Build `WorkflowEditor.vue` (Drag and drop blocks: "Reboot", "Push", "Install").
3.  [x] **Feature**: "Macro Recorder" - Record manual ADB actions and save as a script.
4.  [x] **Variables**: Support variables in scripts (`$DEVICE_SERIAL`, `$DATE`).
5.  [x] **Scheduling**: Allow scripts to run on device connection (e.g., "Auto-Backup Photos").
6.  [x] **Library**: Create a "Standard Library" of common scripts (Debloat, Optimize).
7.  [x] **Safety**: Sandbox script execution to prevent file system damage.
8.  [x] **Sharing**: Export/Import workflows as `.cf-flow` JSON files.
9.  [x] **Logs**: Dedicated execution logs for automation tasks.
10. [x] **Error Handling**: "Stop on Error" vs "Continue" toggle for scripts.

## Phase 6: New Feature - Scrcpy Integration (Screen Mirroring)
**Goal**: Embed high-performance screen mirroring directly into the app.

1.  [x] **Dependency**: Bundle `scrcpy` binary or detect system installation.
2.  [x] **Backend**: Create `scrcpy_manager.rs` to handle the process lifecycle.
3.  [x] **UI**: Create `ScreenMirror.vue` component (using a window overlay or canvas).
4.  [x] **Control**: Map mouse/keyboard events from Vue to `scrcpy` input.
5.  [x] **Feature**: "Record Screen" button in the UI.
6.  [x] **Feature**: "Take Screenshot" button (saving to PC).
7.  [x] **Settings**: Bitrate, FPS, and Resolution controls.
8.  [x] **Audio**: Toggle audio forwarding (Android 11+).
9.  [x] **Wireless**: Support Scrcpy over TCP/IP (Wireless ADB).
10. [x] **Clipboard**: Bi-directional clipboard sync.

## Phase 7: New Feature - Payload Dumper & File Tools
**Goal**: Advanced file manipulation for ROM modders.

1.  [ ] **Backend**: Port or integrate a `payload_dumper` tool (Rust implementation preferred).
2.  [ ] **UI**: Create `PayloadExtractor.vue` (Drag OTA zip -> Select partitions -> Extract).
3.  [ ] **Feature**: `boot.img` Patcher (Integrate Magisk `boot_patch.sh` logic).
4.  [ ] **Explorer**: Build a full `FileExplorer.vue` for `/sdcard/` management.
5.  [ ] **Transfer**: High-speed Push/Pull with progress bars.
6.  [ ] **Editor**: Text editor for modifying config files on the device.
7.  [ ] **Hashing**: SHA256 calculator for local files.
8.  [ ] **Compression**: Zip/Unzip tools for creating flashable zips.
9.  [ ] **Metadata**: Read APK metadata (Package Name, Version, Icon) before install.
10. [ ] **Cleanup**: Temp file manager to clear extraction cache.

## Phase 8: New Feature - Magisk & Root Tools
**Goal**: Specialized tools for rooted devices.

1.  [ ] **Detection**: Robust Root/Magisk detection logic.
2.  [ ] **Repo**: Built-in browser for Magisk Modules (using Alt-Repo API).
3.  [ ] **Installer**: "One-Click Install" for modules (push -> `magisk --install-module`).
4.  [ ] **Manager**: Enable/Disable/Remove modules via ADB root shell.
5.  [ ] **DenyList**: GUI for configuring Zygisk DenyList.
6.  [ ] **Boot**: "Patch & Flash" workflow for rooting a stock device.
7.  [ ] **SafetyNet**: Trigger SafetyNet/Play Integrity check.
8.  [ ] **Props**: `resetprop` GUI for changing system fingerprints.
9.  [ ] **Backup**: Backup Magisk modules and settings.
10. [ ] **Rescue**: "Remove All Modules" script for bootloop recovery.

## Phase 9: New Feature - Cloud & Backup
**Goal**: Data safety and synchronization.

1.  [ ] **Backend**: Integrate Google Drive API or Nextcloud WebDAV.
2.  [ ] **UI**: `CloudSync.vue` dashboard.
3.  [ ] **Feature**: "Full App Backup" (APK + Data via `adb backup` - legacy but useful).
4.  [ ] **Feature**: "Partition Backup" (EFS/IMEI, Modem, Persist).
5.  [ ] **Sync**: Auto-upload backups to the cloud.
6.  [ ] **Restore**: Cloud -> Device restore workflow.
7.  [ ] **Encryption**: AES-256 encrypt backups before upload.
8.  [ ] **Schedule**: Nightly backup scheduler.
9.  [ ] **Contacts/SMS**: Export Contacts/SMS to JSON/VCF.
10. [ ] **Photos**: Incremental photo sync to PC folder.

## Phase 10: New Feature - AI Sentinel & Bootloop Prediction
**Goal**: Proactive system health monitoring using Gemini.

1.  [ ] **Stream**: Real-time `logcat` streaming to the AI engine.
2.  [ ] **Analysis**: Train/Prompt Gemini to recognize "Fatal Exception" patterns.
3.  [ ] **Alert**: "Bootloop Detected" overlay with "Reboot to Recovery" button.
4.  [ ] **Fix**: AI suggests specific fixes based on the error trace.
5.  [ ] **Context**: AI is aware of the currently installed ROM/Kernel.
6.  [ ] **Chat**: "Ask AI about this error" button next to log lines.
7.  [ ] **Optimization**: AI suggests debloat lists based on usage stats.
8.  [ ] **Battery**: AI analyzes wakelocks to find battery drains.
9.  [ ] **Privacy**: "Local Analysis" mode (regex-based) vs "Cloud Analysis" (Gemini).
10. [ ] **Report**: Generate "Incident Report" after a crash.
