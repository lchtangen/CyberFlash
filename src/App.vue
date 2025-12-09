<script setup lang="ts">
import { ref, onMounted, watch, onUnmounted } from 'vue';
import DashboardView from './views/DashboardView.vue';
import FlashView from './views/FlashView.vue';
import SettingsView from './views/SettingsView.vue';
import ToolsView from './views/ToolsView.vue';
import SocialView from './views/SocialView.vue';
import HardwareView from './views/HardwareView.vue';
import LabView from './views/LabView.vue';
import AIAssistantOverlay from './components/features/ai/AIAssistantOverlay.vue';
import CommandPalette from './components/features/core/CommandPalette.vue';
import SmartContextBar from './components/features/core/SmartContextBar.vue';
import NotificationCenter from './components/features/core/NotificationCenter.vue';
import DriverHealthCheck from './components/features/system/DriverHealthCheck.vue';
import SidebarItem from './components/ui/SidebarItem.vue';
import { useSettingsStore } from './stores/settings';
import { useNotificationStore } from './stores/notifications';
import { useDeviceStore } from './stores/device';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

const currentView = ref('dashboard');
const isNotificationPanelOpen = ref(false);
const settingsStore = useSettingsStore();
const notificationStore = useNotificationStore();
const deviceStore = useDeviceStore();
const unlisten = ref<() => void>();

onMounted(async () => {
  // Start the backend monitor
  await invoke('start_usb_monitor');

  // Listen for updates
  unlisten.value = await listen<any[]>('device-status-update', (event) => {
    const devices = event.payload;
    if (devices.length > 0) {
      const activeDevice = devices[0];
      deviceStore.setConnected(true);
      deviceStore.connectionType = activeDevice.connection_type;
      deviceStore.deviceModel = activeDevice.serial;
    } else {
      deviceStore.setConnected(false);
      deviceStore.connectionType = null;
    }
  });
});

onUnmounted(() => {
  if (unlisten.value) unlisten.value();
});

const handleDockAction = async (action: string) => {
  if (action === 'reboot_device') {
    try {
      await invoke('adb_reboot', { mode: 'system' });
      notificationStore.addNotification({ title: 'Rebooting', message: 'Device is restarting...', type: 'info' });
    } catch (e) {
      notificationStore.addNotification({ title: 'Reboot Failed', message: String(e), type: 'error' });
    }
  } else if (action === 'toggle_logs') {
    notificationStore.addNotification({ title: 'Logs', message: 'Log view toggled', type: 'info' });
  }
};

const navGroups = [
  {
    title: 'Overview',
    items: [
      { id: 'dashboard', label: 'Dashboard', icon: 'dashboard', color: 'text-blue-400' }
    ]
  },
  {
    title: 'Operations',
    items: [
      { id: 'flash', label: 'Flash Firmware', icon: 'flash_on', variant: 'primary', description: 'Start New Process' }
    ]
  },
  {
    title: 'Toolbox',
    items: [
      { id: 'tools', label: 'Tools & Utilities', icon: 'construction', color: 'text-orange-400' },
      { id: 'hardware', label: 'Hardware Diagnostics', icon: 'memory', color: 'text-purple-400' },
      { id: 'social', label: 'Community Hub', icon: 'groups', color: 'text-cyan-400' },
      { id: 'lab', label: 'Experimental Lab', icon: 'science', variant: 'warning' }
    ]
  },
  {
    title: 'System',
    items: [
      { id: 'settings', label: 'Settings', icon: 'settings', color: 'text-gray-400' }
    ]
  }
];

const applyTheme = () => {
  const root = document.documentElement;
  
  // 1. Glass Opacity
  root.style.setProperty('--glass-opacity', (settingsStore.glassOpacity / 100).toString());
  
  // 2. Blur Strength
  const blurMap: Record<string, string> = {
    'none': '0px', 'sm': '4px', 'md': '12px', 'lg': '16px', 'xl': '24px', '2xl': '40px', '3xl': '64px'
  };
  root.style.setProperty('--blur-strength', blurMap[settingsStore.blurStrength] || '20px');
  
  // 3. Border Radius
  const radiusMap: Record<string, string> = {
    'sm': '0.25rem', 'md': '0.5rem', 'lg': '0.75rem', 'xl': '1rem', '2xl': '1.5rem', 'full': '9999px'
  };
  root.style.setProperty('--radius-panel', radiusMap[settingsStore.borderRadius] || '1rem');
  
  // 4. Font Scale
  root.style.setProperty('--font-scale', (settingsStore.fontSizeScale / 100).toString());
  
  // 5. Mesh Gradient Visibility
  root.style.setProperty('--mesh-opacity', settingsStore.showMeshGradient ? '1' : '0');
  
  // 6. Accent Color (Primary)
  root.style.setProperty('--color-primary', settingsStore.accentColor);
  
  // Convert Hex to RGB for Tailwind Opacity Support
  const hex = settingsStore.accentColor.replace('#', '');
  if (hex.length === 6) {
    const r = parseInt(hex.substring(0, 2), 16);
    const g = parseInt(hex.substring(2, 4), 16);
    const b = parseInt(hex.substring(4, 6), 16);
    root.style.setProperty('--color-primary-rgb', `${r} ${g} ${b}`);
  }
  
  // 7. Animation Speed
  const speedMap: Record<string, string> = { 'slow': '0.5s', 'normal': '0.3s', 'fast': '0.15s' };
  root.style.setProperty('--transition-speed', speedMap[settingsStore.animationSpeed] || '0.3s');
};

// Watch for changes in store and apply immediately
watch(
  () => [
    settingsStore.glassOpacity,
    settingsStore.blurStrength,
    settingsStore.borderRadius,
    settingsStore.fontSizeScale,
    settingsStore.showMeshGradient,
    settingsStore.accentColor,
    settingsStore.animationSpeed
  ],
  applyTheme,
  { deep: true }
);

onMounted(async () => {
  await settingsStore.loadSettings();
  document.documentElement.classList.add('dark');
  applyTheme();
  
  // Welcome Notification
  setTimeout(() => {
    notificationStore.addNotification({
      title: 'System Online',
      message: 'CyberFlash V2 is ready. Connected to local daemon.',
      type: 'success'
    });
  }, 1000);
});
</script>

<template>
  <div class="min-h-screen mesh-gradient-bg text-text-primary font-sans overflow-hidden relative flex">
    <!-- Sidebar -->
        <!-- Sidebar -->
    <aside class="w-72 m-3 mr-0 rounded-2xl border border-white/10 bg-surface/40 backdrop-blur-2xl flex flex-col shadow-2xl shadow-black/50 z-20 overflow-hidden transition-all duration-500 hover:bg-surface/50">
      <div class="p-6 flex items-center gap-3 border-b border-white/5">
        <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-primary to-secondary flex items-center justify-center shadow-lg shadow-primary/20 relative group cursor-pointer overflow-hidden">
           <div class="absolute inset-0 bg-white/20 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
           <span class="material-symbols-rounded text-white text-2xl drop-shadow-md">bolt</span>
        </div>
        <div>
          <h1 class="text-lg font-bold text-white tracking-tight leading-none">CyberFlash</h1>
          <span class="text-[10px] font-mono text-primary bg-primary/10 px-1.5 py-0.5 rounded border border-primary/20">V2.5 PRO</span>
        </div>
      </div>
      
      <nav class="flex-1 overflow-y-auto p-4 space-y-6 custom-scrollbar">
        <div v-for="group in navGroups" :key="group.title" class="space-y-2">
          <div class="px-3 text-[10px] font-bold text-white/30 uppercase tracking-widest mb-2">{{ group.title }}</div>
          <SidebarItem 
            v-for="item in group.items"
            :key="item.id"
            :icon="item.icon"
            :label="item.label"
            :active="currentView === item.id"
            :variant="item.variant"
            :description="item.description"
            :color="item.color"
            @click="currentView = item.id"
          />
        </div>
      </nav>
      
      <div class="p-4 border-t border-white/5 bg-black/20">
        <div class="flex items-center gap-3 px-2 py-1 rounded-lg hover:bg-white/5 transition-colors cursor-pointer group">
          <div class="relative">
             <div class="w-2 h-2 rounded-full bg-success shadow-[0_0_8px_rgba(48,209,88,0.6)] group-hover:animate-pulse"></div>
             <div class="absolute inset-0 w-2 h-2 rounded-full bg-success animate-ping opacity-20"></div>
          </div>
          <div class="flex flex-col">
             <span class="text-xs font-medium text-white/80 group-hover:text-white transition-colors">System Online</span>
             <span class="text-[10px] text-white/30">Ready for operations</span>
          </div>
        </div>
      </div>
    </aside>

    <!-- Main Content -->
    <main class="flex-1 m-3 rounded-2xl border border-white/10 bg-surface/30 backdrop-blur-xl flex flex-col relative overflow-hidden shadow-2xl shadow-black/50 z-10">
      <!-- Top Bar -->
      <header class="h-16 border-b border-white/5 flex items-center justify-between px-6 bg-white/5 backdrop-blur-md z-10">
        <div class="flex items-center gap-4">
          <h2 class="text-lg font-medium text-white capitalize">{{ currentView }}</h2>
        </div>
        
        <SmartContextBar />
        
        <div class="flex items-center gap-4">
          <button 
            @click="isNotificationPanelOpen = !isNotificationPanelOpen"
            class="p-2 rounded-full hover:bg-white/10 text-text-secondary hover:text-white transition-colors relative"
            :class="{ 'text-white bg-white/10': isNotificationPanelOpen }"
          >
            <span class="material-symbols-rounded">notifications</span>
            <span v-if="notificationStore.unreadCount > 0" class="absolute top-1.5 right-1.5 w-2 h-2 bg-primary rounded-full border border-surface"></span>
          </button>
          <button 
            @click="currentView = 'settings'"
            class="p-2 rounded-full hover:bg-white/10 text-text-secondary hover:text-white transition-colors"
            :class="{ 'text-primary bg-white/10': currentView === 'settings' }"
          >
            <span class="material-symbols-rounded">settings</span>
          </button>
        </div>
      </header>
      
      <div class="flex-1 overflow-y-auto p-6 pb-32 custom-scrollbar relative">
        <Transition name="page-transition" mode="out-in">
          <div :key="currentView" class="h-full">
            <DashboardView v-if="currentView === 'dashboard'" @navigate="currentView = $event" />
            <FlashView v-if="currentView === 'flash'" />
            <ToolsView v-if="currentView === 'tools'" />
            <SocialView v-if="currentView === 'social'" />
            <HardwareView v-if="currentView === 'hardware'" />
            <LabView v-if="currentView === 'lab'" />
            <SettingsView v-if="currentView === 'settings'" />
          </div>
        </Transition>
      </div>
    </main>

    <!-- Overlays -->
    <AIAssistantOverlay />
    <CommandPalette />
    <DriverHealthCheck />
    <NotificationCenter :is-open="isNotificationPanelOpen" @close="isNotificationPanelOpen = false" />
  </div>
</template>
<style>
/* Global styles are handled in style.css */
</style>
