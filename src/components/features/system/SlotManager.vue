<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useNotificationStore } from '../../../stores/notifications';

const currentSlot = ref<'a' | 'b' | null>(null); // Ideally fetch this from getvar all
const loading = ref(false);
const message = ref<string | null>(null);
const notificationStore = useNotificationStore();

const setSlot = async (slot: 'a' | 'b') => {
  loading.value = true;
  message.value = null;
  try {
    const res = await invoke<string>('set_active_slot', { slot, serial: null });
    message.value = res;
    currentSlot.value = slot;
    notificationStore.addNotification({
      title: 'Slot Changed',
      message: `Active slot set to ${slot.toUpperCase()}`,
      type: 'success'
    });
  } catch (e) {
    message.value = `Error: ${e}`;
    notificationStore.addNotification({
      title: 'Slot Change Failed',
      message: String(e),
      type: 'error'
    });
  } finally {
    loading.value = false;
  }
};
</script>

<template>
  <div class="bg-surface/30 border border-white/10 rounded-xl p-4 backdrop-blur-md">
    <h3 class="text-lg font-semibold text-white mb-4">A/B Slot Manager</h3>
    
    <div class="flex gap-4">
      <button 
        @click="setSlot('a')"
        class="flex-1 py-3 rounded-xl border transition-all duration-200 font-bold text-lg"
        :class="currentSlot === 'a' ? 'bg-primary text-white border-primary shadow-lg shadow-primary/20' : 'bg-surface/50 text-text-secondary border-white/10 hover:bg-white/5'"
        :disabled="loading"
      >
        Slot A
      </button>
      
      <button 
        @click="setSlot('b')"
        class="flex-1 py-3 rounded-xl border transition-all duration-200 font-bold text-lg"
        :class="currentSlot === 'b' ? 'bg-primary text-white border-primary shadow-lg shadow-primary/20' : 'bg-surface/50 text-text-secondary border-white/10 hover:bg-white/5'"
        :disabled="loading"
      >
        Slot B
      </button>
    </div>

    <div v-if="message" class="mt-3 text-xs text-center font-mono" :class="message.includes('Error') ? 'text-error' : 'text-success'">
      {{ message }}
    </div>
  </div>
</template>
