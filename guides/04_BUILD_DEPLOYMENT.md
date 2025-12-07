# 🚀 Build & Deployment Guide

**Version**: 2.0.0 | **Target**: macOS & Linux

---

## 📦 Building for Production

Tauri simplifies the build process, creating native binaries for each platform.

### 1. Pre-Build Checklist
- [ ] Update version in `package.json` and `src-tauri/tauri.conf.json`.
- [ ] Run full test suite: `npm run test:unit`.
- [ ] Check for TypeScript errors: `npm run typecheck`.
- [ ] Ensure all assets are optimized.

### 2. Build Command
```bash
# Build for the OS you are currently running
npm run tauri build
```

---

## 🐧 Linux Deployment (AppImage)

The primary distribution format for Linux is **AppImage**. It works on almost all distros.

### Build Artifacts
After running `npm run tauri build` on Linux, you will find:
- `src-tauri/target/release/bundle/appimage/CyberFlash_2.0.0_amd64.AppImage`
- `src-tauri/target/release/bundle/deb/cyberflash_2.0.0_amd64.deb` (if configured)

### Requirements
Ensure you have the build dependencies installed:
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

---

## 🍎 macOS Deployment (DMG)

### Build Artifacts
After running `npm run tauri build` on macOS, you will find:
- `src-tauri/target/release/bundle/dmg/CyberFlash_2.0.0_x64.dmg` (Intel)
- `src-tauri/target/release/bundle/dmg/CyberFlash_2.0.0_aarch64.dmg` (Apple Silicon)

### Code Signing & Notarization
To distribute outside the App Store without warnings, you must sign and notarize.

1.  **Set Environment Variables**:
    ```bash
    export APPLE_CERTIFICATE="..."
    export APPLE_CERTIFICATE_PASSWORD="..."
    export APPLE_SIGNING_IDENTITY="..."
    export APPLE_ID="..."
    export APPLE_PASSWORD="..." # App-specific password
    export APPLE_TEAM_ID="..."
    ```

2.  **Build**:
    Tauri will automatically sign if these variables are present.

---

## 🔄 CI/CD (GitHub Actions)

We use GitHub Actions to build for both platforms automatically on release.

```yaml
# .github/workflows/release.yml
name: Release
on:
  push:
    tags:
      - 'v*'

jobs:
  create-release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Create Release
        id: create_release
        uses: actions/create-release@v1
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tag_name: ${{ github.ref }}
          release_name: Release ${{ github.ref }}
          draft: false
          prerelease: false

  build-tauri:
    needs: create-release
    strategy:
      fail-fast: false
      matrix:
        platform: [macos-latest, ubuntu-20.04]
    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v3
      - name: Setup Node
        uses: actions/setup-node@v3
        with:
          node-version: 18
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      - name: Install dependencies (Ubuntu)
        if: matrix.platform == 'ubuntu-20.04'
        run: |
          sudo apt-get update
          sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libayatana-appindicator3-dev librsvg2-dev
      - name: Install frontend dependencies
        run: npm install
      - name: Build Tauri App
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          releaseId: ${{ needs.create-release.outputs.id }}
```
