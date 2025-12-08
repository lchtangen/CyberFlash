<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import DashboardView from './views/DashboardView.vue';
import FlashView from './views/FlashView.vue';
import SettingsView from './views/SettingsView.vue';
import AIAssistantOverlay from './components/features/AIAssistantOverlay.vue';
import CommandPalette from './components/features/CommandPalette.vue';
import DeviceConnectionHub from './components/features/DeviceConnectionHub.vue';
import NotificationCenter from './components/features/NotificationCenter.vue';
import { useSettingsStore } from './stores/settings';
import { useNotificationStore } from './stores/notifications';

const currentView = ref('dashboard');
const isNotificationPanelOpen = ref(false);
const settingsStore = useSettingsStore();
const notificationStore = useNotificationStore();

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
    notificationStore.addNotification(
      'System Online', 
      'CyberFlash V2 is ready. Connected to local daemon.', 
      'success'
    );
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
      
      <nav class="flex-1 p-4 space-y-2">
        <button 
          @click="currentView = 'dashboard'"
          class="w-full text-left px-4 py-3 rounded-xl transition-all duration-200 flex items-center gap-3 font-medium"
          :class="currentView === 'dashboard' ? 'bg-primary text-white shadow-lg shadow-primary/20' : 'text-text-secondary hover:bg-white/5 hover:text-white'"
        >
          <span class="material-symbols-rounded">dashboard</span>
          Dashboard
        </button>
        <button 
          @click="currentView = 'flash'"
          class="w-full text-left px-4 py-3 rounded-xl transition-all duration-200 flex items-center gap-3 font-medium"
          :class="currentView === 'flash' ? 'bg-primary text-white shadow-lg shadow-primary/20' : 'text-text-secondary hover:bg-white/5 hover:text-white'"
        >
          <span class="material-symbols-rounded">flash_on</span>
          Flash
        </button>
        <button 
          @click="currentView = 'settings'"
          class="w-full text-left px-4 py-3 rounded-xl transition-all duration-200 flex items-center gap-3 font-medium"
          :class="currentView === 'settings' ? 'bg-primary text-white shadow-lg shadow-primary/20' : 'text-text-secondary hover:bg-white/5 hover:text-white'"
        >
          <span class="material-symbols-rounded">settings</span>
          Settings
        </button>
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
      
      <div class="flex-1 overflow-y-auto p-6 custom-scrollbar relative">
        <DashboardView v-if="currentView === 'dashboard'" />
        <FlashView v-if="currentView === 'flash'" />
        <SettingsView v-if="currentView === 'settings'" />
      </div>
    </main>

    <!-- Overlays -->
    <AIAssistantOverlay />
    <CommandPalette />
    <NotificationCenter :is-open="isNotificationPanelOpen" @close="isNotificationPanelOpen = false" />
  </div>
</template>

<style>
/* Global styles are handled in style.css */
</style>
