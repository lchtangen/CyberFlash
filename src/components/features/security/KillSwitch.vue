<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';

const revokeAuth = ref(false);
const isKilling = ref(false);

const kill = async () => {
  isKilling.value = true;
  try {
    await invoke('kill_switch', { revokeAuth: revokeAuth.value });
    alert('ADB Server Killed. Connection severed.');
  } catch (e) {
    console.error(e);
  } finally {
    isKilling.value = false;
  }
};
</script>

<template>
  <GlassCard>
    <div class="flex items-center gap-3 mb-6">
      <div class="p-3 rounded-xl bg-error/20 text-error">
        <span class="material-symbols-rounded text-2xl">power_off</span>
      </div>
      <div>
        <h2 class="text-xl font-bold text-white">Kill Switch</h2>
        <p class="text-sm text-white/60">Emergency disconnect.</p>
      </div>
    </div>

    <div class="space-y-4">
      <p class="text-sm text-white/80">
        Immediately stops the ADB server process on this computer. Useful if a script goes rogue or you suspect malware.
      </p>

      <label class="flex items-center gap-3 p-3 rounded-lg bg-white/5 cursor-pointer hover:bg-white/10 transition-colors">
        <input type="checkbox" v-model="revokeAuth" class="w-4 h-4 rounded border-white/20 bg-black/40 text-primary focus:ring-primary">
        <span class="text-sm text-white">Also revoke USB debugging authorization on device (Requires Root)</span>
      </label>

      <VisionButton 
        @click="kill" 
        :loading="isKilling" 
        variant="danger" 
        icon="bolt" 
        class="w-full h-12 text-lg"
      >
        KILL CONNECTION
      </VisionButton>
    </div>
  </GlassCard>
</template>
