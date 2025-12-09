<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';

const activeSlot = ref<'a' | 'b'>('a');
const loading = ref(false);
const message = ref<string | null>(null);

const switchSlot = async (slot: 'a' | 'b') => {
  loading.value = true;
  message.value = null;
  
  try {
    const res = await invoke<string>('switch_dual_boot_slot', { slot });
    message.value = res;
    activeSlot.value = slot;
  } catch (err: any) {
    message.value = `Error: ${err}`;
  } finally {
    loading.value = false;
  }
};
</script>

<template>
  <GlassCard class="h-full flex flex-col">
    <div class="flex items-center space-x-3 mb-6">
      <div class="p-2 rounded-lg bg-teal-500/20 text-teal-400">
        <span class="material-symbols-rounded text-xl">splitscreen</span>
      </div>
      <div>
        <h3 class="text-lg font-semibold text-white">Dual Boot Manager</h3>
        <p class="text-xs text-text-secondary">Manage A/B Slots</p>
      </div>
    </div>

    <div class="flex-1 flex flex-col items-center justify-center space-y-6">
      <div class="flex items-center space-x-8">
        <div 
          class="flex flex-col items-center space-y-2 cursor-pointer transition-all duration-300"
          :class="activeSlot === 'a' ? 'scale-110 opacity-100' : 'opacity-50 hover:opacity-80'"
          @click="switchSlot('a')"
        >
          <div class="w-16 h-16 rounded-2xl flex items-center justify-center text-2xl font-bold border-2"
            :class="activeSlot === 'a' ? 'bg-primary/20 border-primary text-primary' : 'bg-surface/30 border-white/10 text-white'"
          >
            A
          </div>
          <span class="text-sm font-medium">Slot A</span>
        </div>

        <div class="h-px w-12 bg-white/10"></div>

        <div 
          class="flex flex-col items-center space-y-2 cursor-pointer transition-all duration-300"
          :class="activeSlot === 'b' ? 'scale-110 opacity-100' : 'opacity-50 hover:opacity-80'"
          @click="switchSlot('b')"
        >
          <div class="w-16 h-16 rounded-2xl flex items-center justify-center text-2xl font-bold border-2"
            :class="activeSlot === 'b' ? 'bg-primary/20 border-primary text-primary' : 'bg-surface/30 border-white/10 text-white'"
          >
            B
          </div>
          <span class="text-sm font-medium">Slot B</span>
        </div>
      </div>

      <div v-if="message" class="text-sm text-white bg-white/5 px-3 py-1 rounded-lg">
        {{ message }}
      </div>
    </div>
  </GlassCard>
</template>
