<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useNotificationStore } from '../../stores/notifications';
import GlassCard from '../ui/GlassCard.vue';
import VisionButton from '../ui/VisionButton.vue';

const notificationStore = useNotificationStore();
const loading = ref<string | null>(null);

const reboot = async (mode: string) => {
  loading.value = mode;
  try {
    await invoke('reboot_device', { mode });
    notificationStore.addNotification({
      title: 'Reboot Initiated',
      message: `Rebooting to ${mode}...`,
      type: 'success'
    });
  } catch (e) {
    notificationStore.addNotification({
      title: 'Reboot Failed',
      message: String(e),
      type: 'error'
    });
  } finally {
    loading.value = null;
  }
};

const tools = [
  { 
    id: 'magisk', 
    name: 'Install APK', 
    icon: 'install_mobile', 
    action: async () => {
      try {
        // In a real app, we'd open a file dialog here
        // await invoke('install_apk', { filePath: '...' });
        notificationStore.addNotification({ title: 'APK Installer', message: 'Drag and drop APKs here (Coming Soon)', type: 'info' });
      } catch (e) {
        notificationStore.addNotification({ title: 'Error', message: String(e), type: 'error' });
      }
    }
  },
  { 
    id: 'battery', 
    name: 'Check Battery', 
    icon: 'battery_charging_full', 
    action: async () => {
      try {
        const level = await invoke('check_battery_level');
        notificationStore.addNotification({ title: 'Battery Level', message: `Device battery is at ${level}%`, type: 'info' });
      } catch (e) {
        notificationStore.addNotification({ title: 'Error', message: String(e), type: 'error' });
      }
    }
  },
  { 
    id: 'storage', 
    name: 'Storage Info', 
    icon: 'sd_storage', 
    action: async () => {
      try {
        const info = await invoke('get_storage_info');
        notificationStore.addNotification({ title: 'Storage Info', message: String(info).substring(0, 100) + '...', type: 'info' });
      } catch (e) {
        notificationStore.addNotification({ title: 'Error', message: String(e), type: 'error' });
      }
    }
  },
  { 
    id: 'packages', 
    name: 'List Packages', 
    icon: 'apps', 
    action: async () => {
      try {
        const pkgs: string[] = await invoke('list_packages');
        notificationStore.addNotification({ title: 'Package List', message: `Found ${pkgs.length} installed packages`, type: 'info' });
      } catch (e) {
        notificationStore.addNotification({ title: 'Error', message: String(e), type: 'error' });
      }
    }
  },
  { 
    id: 'screenshot', 
    name: 'Screenshot', 
    icon: 'screenshot', 
    action: async () => {
      try {
        // Save to Downloads folder by default
        const path = '/home/soulmaster/Downloads/screenshot_' + Date.now() + '.png';
        await invoke('adb_screenshot', { outputPath: path });
        notificationStore.addNotification({ title: 'Screenshot Saved', message: `Saved to ${path}`, type: 'success' });
      } catch (e) {
        notificationStore.addNotification({ title: 'Error', message: String(e), type: 'error' });
      }
    }
  },
  { 
    id: 'root_check', 
    name: 'Check Root', 
    icon: 'admin_panel_settings', 
    action: async () => {
      try {
        const isRoot = await invoke('check_root_status');
        notificationStore.addNotification({ 
          title: 'Root Status', 
          message: isRoot ? 'Device is ROOTED' : 'Device is NOT rooted', 
          type: isRoot ? 'success' : 'warning' 
        });
      } catch (e) {
        notificationStore.addNotification({ title: 'Error', message: String(e), type: 'error' });
      }
    }
  },
  { 
    id: 'wireless', 
    name: 'Wireless ADB', 
    icon: 'wifi_tethering', 
    action: async () => {
      try {
        const res = await invoke('enable_wireless_debugging');
        const ip = await invoke('get_ip_address');
        notificationStore.addNotification({ title: 'Wireless ADB', message: `${res}. IP: ${ip}`, type: 'success' });
      } catch (e) {
        notificationStore.addNotification({ title: 'Error', message: String(e), type: 'error' });
      }
    }
  },
  { 
    id: 'density', 
    name: 'Get Density', 
    icon: 'aspect_ratio', 
    action: async () => {
      try {
        const density = await invoke('get_display_density');
        notificationStore.addNotification({ title: 'Display Density', message: `Density: ${density} DPI`, type: 'info' });
      } catch (e) {
        notificationStore.addNotification({ title: 'Error', message: String(e), type: 'error' });
      }
    }
  },
  { 
    id: 'backup_persist', 
    name: 'Backup Persist', 
    icon: 'save', 
    action: async () => {
      try {
        const res = await invoke('backup_partition_image', { partition: 'persist', outputName: 'persist_backup' });
        notificationStore.addNotification({ title: 'Backup Success', message: String(res), type: 'success' });
      } catch (e) {
        notificationStore.addNotification({ title: 'Backup Failed', message: String(e), type: 'error' });
      }
    }
  },
  { 
    id: 'unlock_screen', 
    name: 'Remove Lock', 
    icon: 'lock_open', 
    action: async () => {
      try {
        const res = await invoke('remove_lock_files');
        notificationStore.addNotification({ title: 'Lock Removed', message: String(res), type: 'warning' });
      } catch (e) {
        notificationStore.addNotification({ title: 'Error', message: String(e), type: 'error' });
      }
    }
  },
  { 
    id: 'camera2', 
    name: 'Enable Cam2', 
    icon: 'camera', 
    action: async () => {
      try {
        const res = await invoke('enable_camera2');
        notificationStore.addNotification({ title: 'Camera2 API', message: String(res), type: 'success' });
      } catch (e) {
        notificationStore.addNotification({ title: 'Error', message: String(e), type: 'error' });
      }
    }
  },
  { 
    id: 'user_apps', 
    name: 'User Apps', 
    icon: 'apps_outage', 
    action: async () => {
      try {
        const apps: string[] = await invoke('list_third_party_apps');
        notificationStore.addNotification({ title: 'User Apps', message: `Found ${apps.length} user apps. Check logs.`, type: 'info' });
        console.log('User Apps:', apps);
      } catch (e) {
        notificationStore.addNotification({ title: 'Error', message: String(e), type: 'error' });
      }
    }
  },
  { 
    id: 'battery_detail', 
    name: 'Battery Stats', 
    icon: 'battery_alert', 
    action: async () => {
      try {
        const details = await invoke('get_battery_details');
        notificationStore.addNotification({ title: 'Battery Details', message: 'Check Terminal for full stats', type: 'info' });
        // We should probably log this to the terminal store if possible, but console for now
        console.log(details);
      } catch (e) {
        notificationStore.addNotification({ title: 'Error', message: String(e), type: 'error' });
      }
    }
  },
];
</script>

<template>
  <GlassCard class="h-full flex flex-col">
    <div class="flex items-center justify-between mb-6">
      <h3 class="text-lg font-semibold text-white flex items-center gap-2">
        <span class="material-symbols-rounded text-primary">grid_view</span>
        CyberDeck
      </h3>
      <div class="text-xs text-text-secondary font-mono uppercase tracking-wider">Quick Actions</div>
    </div>

    <div class="grid grid-cols-2 gap-4 mb-6">
      <!-- Power Controls -->
      <div class="col-span-2 p-4 bg-surface/30 rounded-xl border border-white/5">
        <div class="text-xs text-text-secondary uppercase tracking-wider mb-3 font-medium">Power Menu</div>
        <div class="grid grid-cols-3 gap-3">
          <VisionButton 
            @click="reboot('system')" 
            :loading="loading === 'system'"
            variant="secondary" 
            size="sm"
            icon="restart_alt"
          >
            System
          </VisionButton>
          <VisionButton 
            @click="reboot('recovery')" 
            :loading="loading === 'recovery'"
            variant="secondary" 
            size="sm"
            icon="build"
          >
            Recovery
          </VisionButton>
          <VisionButton 
            @click="reboot('bootloader')" 
            :loading="loading === 'bootloader'"
            variant="secondary" 
            size="sm"
            icon="developer_board"
          >
            Bootloader
          </VisionButton>
        </div>
      </div>

      <!-- Tool Grid -->
      <button 
        v-for="tool in tools" 
        :key="tool.id"
        @click="tool.action"
        class="p-4 bg-surface/30 hover:bg-primary/20 border border-white/5 hover:border-primary/50 rounded-xl transition-all duration-200 group flex flex-col items-center justify-center gap-2 text-center min-h-[100px]"
      >
        <span class="material-symbols-rounded text-3xl text-text-secondary group-hover:text-primary transition-colors">{{ tool.icon }}</span>
        <span class="text-xs font-medium text-text-secondary group-hover:text-white transition-colors">{{ tool.name }}</span>
      </button>
    </div>

    <!-- Slot Manager Integration -->
    <div class="mt-auto p-4 bg-surface/30 rounded-xl border border-white/5 flex items-center justify-between">
      <div class="flex items-center gap-3">
        <span class="material-symbols-rounded text-text-secondary">layers</span>
        <div>
          <div class="text-sm font-medium text-white">Active Slot</div>
          <div class="text-xs text-text-secondary">Current: <span class="text-primary font-mono">A</span></div>
        </div>
      </div>
      <div class="flex gap-2">
        <VisionButton @click="invoke('set_active_slot', { slot: 'a', serial: null })" variant="ghost" size="sm">A</VisionButton>
        <VisionButton @click="invoke('set_active_slot', { slot: 'b', serial: null })" variant="ghost" size="sm">B</VisionButton>
      </div>
    </div>
  </GlassCard>
</template>
