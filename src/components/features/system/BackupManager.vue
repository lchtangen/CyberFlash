<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import { useNotificationStore } from '../../../stores/notifications';

const backupPath = ref('');
const isBackingUp = ref(false);
const message = ref<string | null>(null);
const notificationStore = useNotificationStore();

const options = ref({
  apk: false,
  shared: false,
  system: false,
  all: true
});

const selectBackupLocation = async () => {
  try {
    const path = await save({
      filters: [{
        name: 'Android Backup',
        extensions: ['ab']
      }]
    });
    if (path) {
      backupPath.value = path;
    }
  } catch (e) {
    console.error(e);
    notificationStore.addNotification({
      title: 'Selection Error',
      message: 'Failed to select backup location',
      type: 'error'
    });
  }
};

const startBackup = async () => {
  if (!backupPath.value) return;
  
  isBackingUp.value = true;
  message.value = null;

  try {
    const res = await invoke<string>('adb_backup', {
      backupPath: backupPath.value,
      packages: [], // Empty means -all if options.all is true
      apk: options.value.apk,
      shared: options.value.shared,
      system: options.value.system
    });
    message.value = res;
    notificationStore.addNotification({
      title: 'Backup Started',
      message: 'Check device to confirm backup operation.',
      type: 'success',
      duration: 5000
    });
  } catch (e) {
    message.value = `Error: ${e}`;
    notificationStore.addNotification({
      title: 'Backup Failed',
      message: String(e),
      type: 'error'
    });
  } finally {
    isBackingUp.value = false;
  }
};
</script>

<template>
  <div class="bg-surface/30 border border-white/10 rounded-xl p-6 backdrop-blur-md">
    <div class="flex items-center gap-3 mb-4">
      <div class="w-10 h-10 rounded-full bg-primary/20 flex items-center justify-center text-primary">
        <span class="material-symbols-rounded">backup</span>
      </div>
      <div>
        <h3 class="text-lg font-bold text-white">Backup Manager</h3>
        <p class="text-xs text-text-secondary">Save data before flashing</p>
      </div>
    </div>

    <div class="space-y-3 mb-6">
      <label class="flex items-center gap-3 p-3 rounded-lg bg-black/20 border border-white/5 cursor-pointer hover:bg-white/5 transition-colors">
        <input v-model="options.apk" type="checkbox" class="w-4 h-4 rounded border-white/20 text-primary focus:ring-primary bg-transparent">
        <div>
          <div class="text-sm text-white font-medium">Include APKs</div>
          <div class="text-xs text-text-muted">Backup app installers (-apk)</div>
        </div>
      </label>

      <label class="flex items-center gap-3 p-3 rounded-lg bg-black/20 border border-white/5 cursor-pointer hover:bg-white/5 transition-colors">
        <input v-model="options.shared" type="checkbox" class="w-4 h-4 rounded border-white/20 text-primary focus:ring-primary bg-transparent">
        <div>
          <div class="text-sm text-white font-medium">Shared Storage</div>
          <div class="text-xs text-text-muted">SD Card contents (-shared)</div>
        </div>
      </label>
      
      <label class="flex items-center gap-3 p-3 rounded-lg bg-black/20 border border-white/5 cursor-pointer hover:bg-white/5 transition-colors">
        <input v-model="options.system" type="checkbox" class="w-4 h-4 rounded border-white/20 text-primary focus:ring-primary bg-transparent">
        <div>
          <div class="text-sm text-white font-medium">System Apps</div>
          <div class="text-xs text-text-muted">Include system packages (-system)</div>
        </div>
      </label>
    </div>

    <div class="flex gap-2">
      <button 
        @click="selectBackupLocation"
        class="px-4 py-3 bg-white/10 rounded-lg hover:bg-white/20 text-white transition-colors flex items-center justify-center"
        title="Select Save Location"
      >
        <span class="material-symbols-rounded text-sm">folder</span>
      </button>
      
      <button 
        @click="startBackup"
        :disabled="!backupPath || isBackingUp"
        class="flex-1 py-3 rounded-lg font-bold transition-all duration-200 flex items-center justify-center gap-2"
        :class="isBackingUp ? 'bg-surface/50 text-text-muted cursor-not-allowed' : 'bg-primary text-white hover:bg-primary-hover shadow-lg shadow-primary/20'"
      >
        <span v-if="isBackingUp" class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></span>
        {{ isBackingUp ? 'Backing up...' : 'Start Backup' }}
      </button>
    </div>
    
    <div v-if="backupPath" class="mt-2 text-xs text-text-muted truncate px-1">
      Save to: {{ backupPath }}
    </div>

    <div v-if="message" class="mt-3 text-xs text-center font-mono p-2 rounded bg-black/20" :class="message.includes('Error') ? 'text-error' : 'text-success'">
      {{ message }}
    </div>
  </div>
</template>
