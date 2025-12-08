<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useNotificationStore } from '../../stores/notifications';

const isUnlocking = ref(false);
const message = ref<string | null>(null);
const unlockMethod = ref('flashing'); // 'flashing' or 'oem'
const notificationStore = useNotificationStore();

const startUnlock = async () => {
  isUnlocking.value = true;
  message.value = null;
  try {
    const res = await invoke<string>('bootloader_unlock', { serial: null, method: unlockMethod.value });
    message.value = res;
    notificationStore.addNotification({
      title: 'Unlock Command Sent',
      message: res,
      type: 'success'
    });
  } catch (e) {
    message.value = `Error: ${e}`;
    notificationStore.addNotification({
      title: 'Unlock Failed',
      message: String(e),
      type: 'error'
    });
  } finally {
    isUnlocking.value = false;
  }
};

const startLock = async () => {
  if (!confirm("WARNING: Locking the bootloader will wipe data and may brick the device if the OS is not signed correctly. Continue?")) return;
  
  isUnlocking.value = true;
  message.value = null;
  try {
    const res = await invoke<string>('bootloader_lock', { serial: null });
    message.value = res;
    notificationStore.addNotification({
      title: 'Lock Command Sent',
      message: res,
      type: 'success'
    });
  } catch (e) {
    message.value = `Error: ${e}`;
    notificationStore.addNotification({
      title: 'Lock Failed',
      message: String(e),
      type: 'error'
    });
  } finally {
    isUnlocking.value = false;
  }
};
</script>

<template>
  <div class="bg-surface/30 border border-white/10 rounded-xl p-6 backdrop-blur-md">
    <div class="flex items-center gap-3 mb-4">
      <div class="w-10 h-10 rounded-full bg-warning/20 flex items-center justify-center text-warning">
        <span class="material-symbols-rounded">lock_open</span>
      </div>
      <div>
        <h3 class="text-lg font-bold text-white">Bootloader Unlock</h3>
        <p class="text-xs text-text-secondary">Unlock device for custom ROMs</p>
      </div>
    </div>

    <div class="space-y-4">
      <div class="p-3 bg-warning/10 border border-warning/20 rounded-lg text-warning text-xs">
        <strong>Warning:</strong> Unlocking the bootloader will factory reset your device.
      </div>

      <div class="flex gap-2">
        <button 
          @click="unlockMethod = 'flashing'"
          class="flex-1 py-2 text-xs rounded border transition-colors"
          :class="unlockMethod === 'flashing' ? 'bg-white/10 border-white/30 text-white' : 'border-transparent text-text-muted hover:bg-white/5'"
        >
          Standard (flashing unlock)
        </button>
        <button 
          @click="unlockMethod = 'oem'"
          class="flex-1 py-2 text-xs rounded border transition-colors"
          :class="unlockMethod === 'oem' ? 'bg-white/10 border-white/30 text-white' : 'border-transparent text-text-muted hover:bg-white/5'"
        >
          Legacy (oem unlock)
        </button>
      </div>

      <button 
        @click="startUnlock"
        :disabled="isUnlocking"
        class="w-full py-3 rounded-xl font-bold bg-warning text-black hover:bg-warning-hover transition-all duration-200 shadow-lg shadow-warning/20"
      >
        <span v-if="isUnlocking" class="material-symbols-rounded animate-spin text-sm mr-2">refresh</span>
        {{ isUnlocking ? 'Sending Command...' : 'Unlock Bootloader' }}
      </button>

      <div class="pt-4 border-t border-white/10">
        <button 
          @click="startLock"
          :disabled="isUnlocking"
          class="w-full py-2 rounded-lg text-xs font-medium text-text-muted hover:text-error hover:bg-error/10 transition-colors"
        >
          Re-Lock Bootloader (Dangerous)
        </button>
      </div>
    </div>

    <div v-if="message" class="mt-3 text-xs text-center font-mono p-2 rounded bg-black/20" :class="message.includes('Error') ? 'text-error' : 'text-success'">
      {{ message }}
    </div>
  </div>
</template>
