import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export const useSettingsStore = defineStore('settings', () => {
  // --- General & UI ---
  const theme = ref<'light' | 'dark' | 'system'>('dark');
  const accentColor = ref('#0A84FF');
  const enableAnimations = ref(true);
  const glassOpacity = ref(80); // 0-100
  const soundEffects = ref(true);
  const hapticFeedback = ref(false);
  const compactMode = ref(false);
  const showTerminalOverlay = ref(true);
  const themePreset = ref('cyber-cyan');

  // --- Appearance Customization ---
  const blurStrength = ref('xl'); // none, sm, md, lg, xl, 2xl, 3xl
  const borderRadius = ref('xl'); // sm, md, lg, xl, 2xl, full
  const shadowIntensity = ref('normal'); // none, sm, normal, lg, xl
  const fontSizeScale = ref(100); // percentage
  const showMeshGradient = ref(true);
  const sidebarTransparency = ref(true);
  const cardHoverEffects = ref(true);
  const buttonGlow = ref(true);
  const textContrast = ref('normal'); // normal, high
  const scrollbarStyle = ref('thin'); // none, thin, default
  const animationSpeed = ref('normal'); // slow, normal, fast
  const layoutDensity = ref('comfortable'); // compact, comfortable, spacious
  const iconStyle = ref('rounded'); // rounded, sharp
  const toastPosition = ref('top-right');
  const tooltipDelay = ref(500);
  const rippleEffects = ref(true);
  const terminalOpacity = ref(90);
  const navBarBlur = ref(true);
  const activeTabStyle = ref('glow'); // glow, underline, pill
  const fontFamily = ref('sans'); // sans, mono, serif

  // --- Flashing Safety ---
  const verifyMd5Checksum = ref(true);
  const checkBatteryLevel = ref(true);
  const minBatteryThreshold = ref(30);
  const backupEFS = ref(true);
  const backupPersist = ref(false);
  const checkDeviceCompatibility = ref(true);
  const preventDowngrade = ref(true);
  const autoUnbrick = ref(false);

  // --- ADB & Fastboot ---
  const adbPath = ref('/usr/bin/adb');
  const fastbootPath = ref('/usr/bin/fastboot');
  const useInternalServer = ref(true);
  const killServerOnExit = ref(true);
  const connectionTimeout = ref(5000);
  const maxRetries = ref(3);
  const usbBufferAlignment = ref(true);
  const forceSlotSwitch = ref('auto'); // auto, a, b

  // --- Post-Flash Automation ---
  const wipeData = ref(false);
  const wipeCache = ref(true);
  const disableVerity = ref(false);
  const disableEncryption = ref(false);
  const flashMagisk = ref(false);
  const magiskVersion = ref('stable');
  const flashGapps = ref(false);
  const rebootAfterFlash = ref(true);
  const bootToRecovery = ref(false);

  // --- Network & Cloud ---
  const checkUpdates = ref(true);
  const sendStats = ref(false);
  const downloadRegion = ref('global');
  const bandwidthLimit = ref(0); // 0 = unlimited
  const proxyEnabled = ref(false);
  const proxyAddress = ref('');

  // --- Developer ---
  const verboseLogging = ref(false);
  const saveLogsToFile = ref(true);
  const mockMode = ref(false);

  // --- AI & Automation ---
  const aiEnabled = ref(true);
  const geminiApiKey = ref('');
  const aiModel = ref('gemini-1.5-flash');
  const autoAnalyzeLogs = ref(true);
  const suggestNextSteps = ref(true);

  const effectiveGeminiApiKey = computed(() => {
    return geminiApiKey.value || import.meta.env.VITE_GEMINI_API_KEY || '';
  });

  // --- Performance & Optimization (New) ---
  const hardwareAcceleration = ref(true);
  const processPriority = ref('high'); // normal, high, realtime
  const maxConcurrentDownloads = ref(3);
  const cacheSizeLimit = ref(1024); // MB
  const autoClearCache = ref(true);
  const lowSpecMode = ref(false);
  const backgroundProcessing = ref(true);
  const networkTimeout = ref(30000); // ms
  const compressionLevel = ref(5); // 0-9
  const useGpuRendering = ref(true);

  function toggleTheme() {
    theme.value = theme.value === 'light' ? 'dark' : 'light';
    updateTheme();
  }

  function updateTheme() {
    if (theme.value === 'dark' || (theme.value === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  }

  // Map for easy save/load
  const stateMap = {
    theme, accentColor, enableAnimations, glassOpacity, soundEffects, hapticFeedback, compactMode, showTerminalOverlay,
    blurStrength, borderRadius, shadowIntensity, fontSizeScale, showMeshGradient, sidebarTransparency, cardHoverEffects, buttonGlow, textContrast, scrollbarStyle, animationSpeed, layoutDensity, iconStyle, toastPosition, tooltipDelay, rippleEffects, terminalOpacity, navBarBlur, activeTabStyle, fontFamily,
    verifyMd5Checksum, checkBatteryLevel, minBatteryThreshold, backupEFS, backupPersist, checkDeviceCompatibility, preventDowngrade, autoUnbrick,
    adbPath, fastbootPath, useInternalServer, killServerOnExit, connectionTimeout, maxRetries, usbBufferAlignment, forceSlotSwitch,
    wipeData, wipeCache, disableVerity, disableEncryption, flashMagisk, magiskVersion, flashGapps, rebootAfterFlash, bootToRecovery,
    checkUpdates, sendStats, downloadRegion, bandwidthLimit, proxyEnabled, proxyAddress,
    verboseLogging, saveLogsToFile, mockMode,
    aiEnabled, geminiApiKey, aiModel, autoAnalyzeLogs, suggestNextSteps,
    hardwareAcceleration, processPriority, maxConcurrentDownloads, cacheSizeLimit, autoClearCache, lowSpecMode, backgroundProcessing, networkTimeout, compressionLevel, useGpuRendering
  };

  async function saveSettings() {
    const state: Record<string, any> = {};
    for (const [key, refVar] of Object.entries(stateMap)) {
        state[key] = refVar.value;
    }
    try {
        await invoke('save_settings', { settings: state });
    } catch (error) {
        console.error('Failed to save settings:', error);
        throw error;
    }
  }

  async function loadSettings() {
    try {
        const saved: any = await invoke('load_settings');
        for (const [key, value] of Object.entries(saved)) {
            if (key in stateMap) {
                // @ts-ignore
                stateMap[key].value = value;
            }
        }
        updateTheme();
    } catch (e) {
        console.error('Failed to load settings:', e);
    }
  }

  return {
    // State
    theme, accentColor, enableAnimations, glassOpacity, soundEffects, hapticFeedback, compactMode, showTerminalOverlay,
    blurStrength, borderRadius, shadowIntensity, fontSizeScale, showMeshGradient, sidebarTransparency, cardHoverEffects, buttonGlow, textContrast, scrollbarStyle, animationSpeed, layoutDensity, iconStyle, toastPosition, tooltipDelay, rippleEffects, terminalOpacity, navBarBlur, activeTabStyle, fontFamily,
    verifyMd5Checksum, checkBatteryLevel, minBatteryThreshold, backupEFS, backupPersist, checkDeviceCompatibility, preventDowngrade, autoUnbrick,
    adbPath, fastbootPath, useInternalServer, killServerOnExit, connectionTimeout, maxRetries, usbBufferAlignment, forceSlotSwitch,
    wipeData, wipeCache, disableVerity, disableEncryption, flashMagisk, magiskVersion, flashGapps, rebootAfterFlash, bootToRecovery,
    checkUpdates, sendStats, downloadRegion, bandwidthLimit, proxyEnabled, proxyAddress,
    verboseLogging, saveLogsToFile, mockMode,
    aiEnabled, geminiApiKey, effectiveGeminiApiKey, aiModel, autoAnalyzeLogs, suggestNextSteps,
    hardwareAcceleration, processPriority, maxConcurrentDownloads, cacheSizeLimit, autoClearCache, lowSpecMode, backgroundProcessing, networkTimeout, compressionLevel, useGpuRendering,
    // Actions
    toggleTheme, updateTheme, saveSettings, loadSettings,
    themePreset
  };
});
