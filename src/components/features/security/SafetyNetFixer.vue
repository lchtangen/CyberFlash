<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';

const isInstalling = ref(false);
const message = ref<string | null>(null);

const installFix = async () => {
  isInstalling.value = true;
  message.value = null;
  try {
    await invoke('install_safetynet_fix');
    message.value = 'Success! Universal SafetyNet Fix installed. Please reboot.';
  } catch (e) {
    message.value = 'Error: ' + e;
  } finally {
    isInstalling.value = false;
  }
};
</script>

<template>
  <GlassCard>
    <div class="flex items-center gap-3 mb-6">
      <div class="p-3 rounded-xl bg-success/20 text-success">
        <span class="material-symbols-rounded text-2xl">verified_user</span>
      </div>
      <div>
        <h2 class="text-xl font-bold text-white">SafetyNet Fixer</h2>
        <p class="text-sm text-white/60">Pass Play Integrity API checks.</p>
      </div>
    </div>

    <div class="space-y-4">
      <p class="text-sm text-white/80">
        Automatically downloads and installs the <b>Universal SafetyNet Fix (Displax Mod)</b> Magisk module.
        This helps hide root from banking apps and Google Pay.
      </p>

      <VisionButton 
        @click="installFix" 
        :loading="isInstalling" 
        icon="download" 
        class="w-full"
      >
        Install Fix
      </VisionButton>

      <div v-if="message" class="p-3 rounded-lg text-center text-xs font-bold" :class="message.includes('Error') ? 'bg-error/20 text-error' : 'bg-success/20 text-success'">
        {{ message }}
      </div>
    </div>
  </GlassCard>
</template>
