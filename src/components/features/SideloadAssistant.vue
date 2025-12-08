<script setup lang="ts">
import { ref, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useNotificationStore } from '../../stores/notifications';

const filePath = ref('');
const isFlashing = ref(false);
const progress = ref('');
const logs = ref<string[]>([]);
const notificationStore = useNotificationStore();

const unlisten = ref<() => void>();

const selectFile = async () => {
  // In a real app, use tauri dialog open
  // For now, manual input or drag drop logic from RomSelector would go here
  // We'll assume filePath is populated
};

const startSideload = async () => {
  if (!filePath.value) return;
  
  isFlashing.value = true;
  logs.value = [];
  
  // Listen for progress
  unlisten.value = await listen<string>('sideload-progress', (event) => {
    progress.value = event.payload;
    logs.value.push(event.payload);
  });

  try {
    await invoke('adb_sideload', { filePath: filePath.value });
    logs.value.push("Sideload Complete!");
    notificationStore.addNotification({
      title: 'Sideload Complete',
      message: 'OTA package installed successfully.',
      type: 'success'
    });
  } catch (e) {
    logs.value.push(`Error: ${e}`);
    notificationStore.addNotification({
      title: 'Sideload Failed',
      message: String(e),
      type: 'error'
    });
  } finally {
    isFlashing.value = false;
    if (unlisten.value) unlisten.value();
  }
};

onUnmounted(() => {
  if (unlisten.value) unlisten.value();
});
</script>

<template>
  <div class="bg-surface/30 border border-white/10 rounded-xl p-6 backdrop-blur-md">
    <div class="flex items-center gap-3 mb-6">
      <div class="w-10 h-10 rounded-full bg-primary/20 flex items-center justify-center text-primary">
        <span class="material-symbols-rounded">system_update</span>
      </div>
      <div>
        <h3 class="text-xl font-bold text-white">ADB Sideload</h3>
        <p class="text-sm text-text-secondary">Flash OTA zips directly via ADB</p>
      </div>
    </div>

    <div class="space-y-4">
      <div class="relative">
        <input 
          v-model="filePath"
          type="text" 
          placeholder="/path/to/update.zip"
          class="w-full bg-black/20 border border-white/10 rounded-lg px-4 py-3 text-white focus:border-primary/50 focus:outline-none transition-colors"
        />
        <button class="absolute right-2 top-2 p-1.5 bg-white/10 rounded hover:bg-white/20 text-white transition-colors flex items-center justify-center">
          <span class="material-symbols-rounded text-sm">folder_open</span>
        </button>
      </div>

      <button 
        @click="startSideload"
        :disabled="!filePath || isFlashing"
        class="w-full py-3 rounded-xl font-bold transition-all duration-200 flex items-center justify-center gap-2"
        :class="isFlashing ? 'bg-surface/50 text-text-muted cursor-not-allowed' : 'bg-primary text-white hover:bg-primary-hover shadow-lg shadow-primary/20'"
      >
        <span v-if="isFlashing" class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></span>
        {{ isFlashing ? 'Sideloading...' : 'Start Sideload' }}
      </button>

      <div v-if="logs.length > 0" class="bg-black/40 rounded-lg p-3 h-32 overflow-y-auto font-mono text-xs text-gray-300 custom-scrollbar">
        <div v-for="(log, i) in logs" :key="i">{{ log }}</div>
      </div>
    </div>
  </div>
</template>
