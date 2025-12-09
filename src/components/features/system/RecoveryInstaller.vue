<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { useNotificationStore } from '../../../stores/notifications';

const recoveryPath = ref('');
const isFlashing = ref(false);
const message = ref<string | null>(null);
const targetSlot = ref('current'); // current, a, b, all
const notificationStore = useNotificationStore();

const selectRecovery = async () => {
  try {
    const file = await open({
      filters: [{ name: 'Recovery Image', extensions: ['img'] }]
    });
    if (file) {
      recoveryPath.value = Array.isArray(file) ? file[0] : file;
    }
  } catch (e) {
    console.error(e);
    notificationStore.addNotification({
      title: 'Selection Error',
      message: 'Failed to select file',
      type: 'error'
    });
  }
};

const flashRecovery = async () => {
  if (!recoveryPath.value) return;
  isFlashing.value = true;
  message.value = null;

  try {
    const res = await invoke<string>('flash_recovery', { 
      path: recoveryPath.value, 
      slot: targetSlot.value,
      serial: null 
    });
    message.value = res;
    notificationStore.addNotification({
      title: 'Flash Successful',
      message: res,
      type: 'success'
    });
  } catch (e) {
    message.value = `Error: ${e}`;
    notificationStore.addNotification({
      title: 'Flash Failed',
      message: String(e),
      type: 'error'
    });
  } finally {
    isFlashing.value = false;
  }
};

const bootRecovery = async () => {
  if (!recoveryPath.value) return;
  isFlashing.value = true;
  message.value = null;

  try {
    const res = await invoke<string>('boot_recovery', { 
      path: recoveryPath.value, 
      serial: null 
    });
    message.value = res;
    notificationStore.addNotification({
      title: 'Boot Command Sent',
      message: res,
      type: 'success'
    });
  } catch (e) {
    message.value = `Error: ${e}`;
    notificationStore.addNotification({
      title: 'Boot Failed',
      message: String(e),
      type: 'error'
    });
  } finally {
    isFlashing.value = false;
  }
};
</script>

<template>
  <div class="bg-surface/30 border border-white/10 rounded-xl p-6 backdrop-blur-md">
    <div class="flex items-center gap-3 mb-4">
      <div class="w-10 h-10 rounded-full bg-primary/20 flex items-center justify-center text-primary">
        <span class="material-symbols-rounded">medical_services</span>
      </div>
      <div>
        <h3 class="text-lg font-bold text-white">Recovery Installer</h3>
        <p class="text-xs text-text-secondary">Flash TWRP / OrangeFox</p>
      </div>
    </div>

    <div class="space-y-4">
      <!-- File Selection -->
      <div 
        @click="selectRecovery"
        class="border border-dashed border-white/20 rounded-lg p-4 text-center cursor-pointer hover:bg-white/5 transition-colors"
      >
        <div v-if="!recoveryPath" class="text-text-muted text-sm">
          Click to select recovery.img
        </div>
        <div v-else class="text-white text-xs font-mono break-all">
          {{ recoveryPath }}
        </div>
      </div>

      <!-- Slot Selection -->
      <div class="flex gap-2">
        <select v-model="targetSlot" class="bg-black/20 border border-white/10 rounded-lg px-3 py-2 text-xs text-white focus:outline-none focus:border-primary/50 w-full">
          <option value="current">Current Slot (Auto)</option>
          <option value="all">Both Slots (A + B)</option>
          <option value="a">Slot A Only</option>
          <option value="b">Slot B Only</option>
        </select>
      </div>

      <!-- Actions -->
      <div class="grid grid-cols-2 gap-3">
        <button 
          @click="bootRecovery"
          :disabled="!recoveryPath || isFlashing"
          class="py-2 rounded-lg bg-white/10 text-white hover:bg-white/20 transition-colors text-sm font-medium"
        >
          Boot Only
        </button>
        <button 
          @click="flashRecovery"
          :disabled="!recoveryPath || isFlashing"
          class="py-2 rounded-lg bg-primary text-white hover:bg-primary-hover shadow-lg shadow-primary/20 transition-colors text-sm font-bold"
        >
          Flash Recovery
        </button>
      </div>
    </div>

    <div v-if="message" class="mt-3 text-xs text-center font-mono p-2 rounded bg-black/20" :class="message.includes('Error') ? 'text-error' : 'text-success'">
      {{ message }}
    </div>
  </div>
</template>
