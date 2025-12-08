# 🛠️ DevOps Manual: Workflow, Build & Ops

**Version**: 2.1.0 | **Updated**: December 8, 2025

---

## ☀️ Development Workflow

### 1. Start the Environment
```bash
# Terminal 1: Start the Development Server (Vue + Tauri)
npm run tauri dev

# Terminal 2: Run Tests in Watch Mode
npm run test:unit -- --watch
```

### 2. Code Quality Checks
Before committing, run the full suite:
```bash
npm run lint         # ESLint + Prettier
npm run typecheck    # TypeScript compiler check
npm run test:unit    # Vitest
```

---

## 🧪 Testing Strategy (Vitest)

We use **Vitest** for unit testing. It is compatible with Jest but much faster and native to Vite.

### Component Testing
```typescript
// tests/components/NeonButton.spec.ts
import { mount } from '@vue/test-utils'
import { describe, it, expect } from 'vitest'
import NeonButton from '@/components/ui/NeonButton.vue'

describe('NeonButton', () => {
  it('renders slot content', () => {
    const wrapper = mount(NeonButton, { slots: { default: 'Click Me' } })
    expect(wrapper.text()).toContain('Click Me')
  })
})
```

---

## 📦 Building for Production

### 1. Pre-Build Checklist
- [ ] Update version in `package.json` and `src-tauri/tauri.conf.json`.
- [ ] Run full test suite: `npm run test:unit`.
- [ ] Check for TypeScript errors: `npm run typecheck`.

### 2. Build Command
```bash
# Build for the OS you are currently running
npm run tauri build
```

### 🐧 Linux Deployment (AppImage)
- **Artifacts**: `src-tauri/target/release/bundle/appimage/CyberFlash_2.0.0_amd64.AppImage`
- **Dependencies**: `libwebkit2gtk-4.0-dev`, `build-essential`, `curl`, `wget`, `file`, `libssl-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`.

### 🍎 macOS Deployment (DMG)
- **Artifacts**: `src-tauri/target/release/bundle/dmg/CyberFlash_2.0.0_x64.dmg`
- **Signing**: Export `APPLE_CERTIFICATE`, `APPLE_CERTIFICATE_PASSWORD`, `APPLE_ID`, `APPLE_PASSWORD`, `APPLE_TEAM_ID`.

### 🪟 Windows Deployment (MSI/EXE)
- **Artifacts**: `src-tauri/target/release/bundle/msi/CyberFlash_2.0.0_x64.msi`
- **Requirements**: Microsoft Visual Studio C++ Build Tools.

---

## 🔄 CI/CD (GitHub Actions)

We use GitHub Actions to build for all platforms automatically on release.

```yaml
# .github/workflows/release.yml
name: Release
on:
  push:
    tags:
      - 'v*'
jobs:
  build-tauri:
    strategy:
      matrix:
        platform: [macos-latest, ubuntu-20.04, windows-latest]
    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v3
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      - name: Build Tauri App
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

---

## 🔧 Troubleshooting Guide

### 🔌 ADB & Device Connection

| Issue | Solution |
|-------|----------|
| **Device not found** | Enable USB Debugging. Check cable. Run `adb kill-server && adb start-server`. |
| **Unauthorized** | Check phone screen for popup. Revoke USB debugging authorizations and reconnect. |
| **Linux Permissions** | Add udev rules: `SUBSYSTEM=="usb", ATTR{idVendor}=="2d01", MODE="0666", GROUP="plugdev"` |
| **Windows Drivers** | Install "OnePlus USB Drivers" or "Google USB Driver". Check Device Manager. |

### 🏗️ Build & Runtime

| Issue | Solution |
|-------|----------|
| **Rust dependency error** | `rustup update` then `cargo clean`. |
| **WebView2 not found** | Install `libwebkit2gtk-4.0-dev` (Linux) or WebView2 Runtime (Windows). |
| **AI API Key Invalid** | Check `.env` for `GEMINI_API_KEY`. Ensure permissions for `gemini-pro`. |
