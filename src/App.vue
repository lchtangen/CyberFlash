<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import DashboardView from './views/DashboardView.vue';
import FlashView from './views/FlashView.vue';
import SettingsView from './views/SettingsView.vue';
import ToolsView from './views/ToolsView.vue';
import SocialView from './views/SocialView.vue';
import HardwareView from './views/HardwareView.vue';
import AIAssistantOverlay from './components/features/ai/AIAssistantOverlay.vue';
import CommandPalette from './components/features/core/CommandPalette.vue';
import DeviceConnectionHub from './components/features/core/DeviceConnectionHub.vue';
import NotificationCenter from './components/features/core/NotificationCenter.vue';
import DriverHealthCheck from './components/features/system/DriverHealthCheck.vue';
import SidebarItem from './components/ui/SidebarItem.vue';
import { useSettingsStore } from './stores/settings';
import { useNotificationStore } from './stores/notifications';
import { invoke } from '@tauri-apps/api/core';

const currentView = ref('dashboard');
const isNotificationPanelOpen = ref(false);
const settingsStore = useSettingsStore();
const notificationStore = useNotificationStore();

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
      { id: 'dashboard', label: 'Dashboard', icon: 'dashboard' }
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
      { id: 'tools', label: 'Tools & Utilities', icon: 'construction' },
      { id: 'hardware', label: 'Hardware Diagnostics', icon: 'memory' },
      { id: 'social', label: 'Community Hub', icon: 'groups' }
    ]
  },
  {
    title: 'System',
    items: [
      { id: 'settings', label: 'Settings', icon: 'settings' }
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
    <aside class="w-64 bg-surface/30 backdrop-blur-xl border-r border-white/10 flex flex-col z-20">
      <div class="p-6 border-b border-white/10">
        <h1 class="text-xl font-bold tracking-tight text-white">CyberFlash</h1>
        <p class="text-xs text-text-secondary mt-1">Version 2.0.1</p>
      </div>
      
      <nav class="flex-1 p-4 space-y-6 overflow-y-auto custom-scrollbar">
        <div v-for="group in navGroups" :key="group.title" class="space-y-2">
          <div class="px-3 text-[10px] font-bold text-white/30 uppercase tracking-widest mb-2">{{ group.title }}</div>
          <SidebarItem 
            v-for="item in group.items"
            :key="item.id"
            :icon="item.icon"
            :label="item.label"
            :active="currentView === item.id"
            :variant="item.variant as any"
            :description="item.description"
            @click="currentView = item.id"
          />
        </div>
      </nav>
      
      <div class="p-4 border-t border-white/10">
        <div class="flex items-center gap-3 px-2">
          <div class="w-2 h-2 rounded-full bg-success shadow-[0_0_8px_rgba(48,209,88,0.6)]"></div>
          <span class="text-xs font-medium text-text-secondary">System Online</span>
        </div>
      </div>
    </aside>

    <!-- Main Content -->
    <main class="flex-1 relative overflow-hidden flex flex-col bg-background">
      <!-- Top Bar -->
      <header class="h-16 border-b border-white/10 flex items-center justify-between px-6 bg-surface/50 backdrop-blur-md z-10">
        <div class="flex items-center gap-4">
          <h2 class="text-lg font-medium text-white capitalize">{{ currentView }}</h2>
        </div>
        
        <DeviceConnectionHub />
        
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
        <DashboardView v-if="currentView === 'dashboard'" @navigate="currentView = $event" />
        <FlashView v-if="currentView === 'flash'" />
        <ToolsView v-if="currentView === 'tools'" />
        <SocialView v-if="currentView === 'social'" />
        <HardwareView v-if="currentView === 'hardware'" />
        <SettingsView v-if="currentView === 'settings'" />
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
