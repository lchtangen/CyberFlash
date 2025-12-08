<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useNotificationStore } from '../../stores/notifications';

const isConfirming = ref(false);
const isWiping = ref(false);
const message = ref<string | null>(null);
const wipeMethod = ref('fastboot'); // 'fastboot' or 'recovery'
const notificationStore = useNotificationStore();

const initiateWipe = () => {
  isConfirming.value = true;
};

const cancelWipe = () => {
  isConfirming.value = false;
};

const confirmWipe = async () => {
  isConfirming.value = false;
  isWiping.value = true;
  message.value = null;

  try {
    const res = await invoke<string>('factory_reset', { serial: null, method: wipeMethod.value });
    message.value = res;
    notificationStore.addNotification({
      title: 'Wipe Successful',
      message: res,
      type: 'success'
    });
  } catch (e) {
    message.value = `Error: ${e}`;
    notificationStore.addNotification({
      title: 'Wipe Failed',
      message: String(e),
      type: 'error'
    });
  } finally {
    isWiping.value = false;
  }
};
</script>

<template>
  <div class="bg-surface/30 border border-white/10 rounded-xl p-6 backdrop-blur-md">
    <div class="flex items-center gap-3 mb-4">
      <div class="w-10 h-10 rounded-full bg-error/20 flex items-center justify-center text-error">
        <span class="material-symbols-rounded">delete_forever</span>
      </div>
      <div>
        <h3 class="text-lg font-bold text-white">Factory Reset</h3>
        <p class="text-xs text-text-secondary">Wipe Data & Cache</p>
      </div>
    </div>

    <div v-if="!isConfirming && !isWiping">
      <div class="flex gap-2 mb-4">
        <button 
          @click="wipeMethod = 'fastboot'"
          class="flex-1 py-2 text-xs rounded border transition-colors"
          :class="wipeMethod === 'fastboot' ? 'bg-white/10 border-white/30 text-white' : 'border-transparent text-text-muted hover:bg-white/5'"
        >
          Fastboot (-w)
        </button>
        <button 
          @click="wipeMethod = 'recovery'"
          class="flex-1 py-2 text-xs rounded border transition-colors"
          :class="wipeMethod === 'recovery' ? 'bg-white/10 border-white/30 text-white' : 'border-transparent text-text-muted hover:bg-white/5'"
        >
          Recovery
        </button>
      </div>

      <button 
        @click="initiateWipe"
        class="w-full py-3 rounded-xl font-bold bg-error/10 text-error border border-error/20 hover:bg-error hover:text-white transition-all duration-200"
      >
        Wipe Device
      </button>
    </div>

    <div v-else-if="isConfirming" class="space-y-3 animate-in fade-in zoom-in duration-200">
      <p class="text-sm text-white text-center font-bold">Are you absolutely sure?</p>
      <p class="text-xs text-text-muted text-center">This will erase all user data. This action cannot be undone.</p>
      
      <div class="flex gap-3">
        <button 
          @click="cancelWipe"
          class="flex-1 py-2 rounded-lg bg-white/10 text-white hover:bg-white/20 transition-colors"
        >
          Cancel
        </button>
        <button 
          @click="confirmWipe"
          class="flex-1 py-2 rounded-lg bg-error text-white hover:bg-error-hover shadow-lg shadow-error/20 transition-colors font-bold"
        >
          YES, WIPE IT
        </button>
      </div>
    </div>

    <div v-else class="text-center py-4">
      <div class="w-8 h-8 border-2 border-error border-t-transparent rounded-full animate-spin mx-auto mb-2"></div>
      <p class="text-error text-sm animate-pulse">Wiping Data...</p>
    </div>

    <div v-if="message" class="mt-3 text-xs text-center font-mono p-2 rounded bg-black/20" :class="message.includes('Error') ? 'text-error' : 'text-success'">
      {{ message }}
    </div>
  </div>
</template>
