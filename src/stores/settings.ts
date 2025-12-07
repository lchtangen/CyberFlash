import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useSettingsStore = defineStore('settings', () => {
  // --- General & UI ---
  const theme = ref<'light' | 'dark' | 'system'>('dark');
  const accentColor = ref('blue');
  const enableAnimations = ref(true);
  const glassOpacity = ref(80); // 0-100
  const soundEffects = ref(true);
  const hapticFeedback = ref(false);
  const compactMode = ref(false);
  const showTerminalOverlay = ref(true);

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
  const aiModel = ref('claude-sonnet-4.5');
  const autoAnalyzeLogs = ref(true);
  const suggestNextSteps = ref(true);

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

  return {
    // State
    theme, accentColor, enableAnimations, glassOpacity, soundEffects, hapticFeedback, compactMode, showTerminalOverlay,
    blurStrength, borderRadius, shadowIntensity, fontSizeScale, showMeshGradient, sidebarTransparency, cardHoverEffects, buttonGlow, textContrast, scrollbarStyle, animationSpeed, layoutDensity, iconStyle, toastPosition, tooltipDelay, rippleEffects, terminalOpacity, navBarBlur, activeTabStyle, fontFamily,
    verifyMd5Checksum, checkBatteryLevel, minBatteryThreshold, backupEFS, backupPersist, checkDeviceCompatibility, preventDowngrade, autoUnbrick,
    adbPath, fastbootPath, useInternalServer, killServerOnExit, connectionTimeout, maxRetries, usbBufferAlignment, forceSlotSwitch,
    wipeData, wipeCache, disableVerity, disableEncryption, flashMagisk, magiskVersion, flashGapps, rebootAfterFlash, bootToRecovery,
    checkUpdates, sendStats, downloadRegion, bandwidthLimit, proxyEnabled, proxyAddress,
    verboseLogging, saveLogsToFile, mockMode,
    aiEnabled, geminiApiKey, aiModel, autoAnalyzeLogs, suggestNextSteps,
    // Actions
    toggleTheme, updateTheme
  };
});
