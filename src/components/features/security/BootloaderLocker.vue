<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';

const confirmed = ref(false);
const isLocking = ref(false);
const message = ref<string | null>(null);

const lock = async () => {
  if (!confirmed.value) return;
  if (!confirm("FINAL WARNING: This will WIPE ALL DATA and re-lock the bootloader. If your OS is not signed correctly, the device will not boot.")) return;
  
  isLocking.value = true;
  message.value = null;
  
  try {
    await invoke<string>('lock_bootloader');
    message.value = 'Command sent. Check device screen to confirm lock.';
  } catch (e) {
    message.value = 'Error: ' + e;
  } finally {
    isLocking.value = false;
  }
};
</script>

<template>
  <GlassCard>
    <div class="flex items-center gap-3 mb-6">
      <div class="p-3 rounded-xl bg-error/20 text-error">
        <span class="material-symbols-rounded text-2xl">lock</span>
      </div>
      <div>
        <h2 class="text-xl font-bold text-white">Bootloader Locker</h2>
        <p class="text-sm text-white/60">Re-lock device security.</p>
      </div>
    </div>

    <div class="space-y-4">
      <div class="bg-error/10 border border-error/20 p-4 rounded-xl">
        <p class="text-error text-xs font-bold mb-2">DANGER ZONE</p>
        <ul class="list-disc list-inside text-xs text-white/80 space-y-1">
          <li>This will <b>WIPE ALL DATA</b> (Factory Reset).</li>
          <li>If you have a custom ROM installed without proper signing keys, your device <b>WILL NOT BOOT</b>.</li>
          <li>Only use this if you are on Stock ROM or a properly signed Custom ROM (e.g. GrapheneOS, CalyxOS).</li>
        </ul>
      </div>

      <label class="flex items-center gap-3 p-3 rounded-lg bg-white/5 cursor-pointer hover:bg-white/10 transition-colors">
        <input type="checkbox" v-model="confirmed" class="w-4 h-4 rounded border-white/20 bg-black/40 text-primary focus:ring-primary">
        <span class="text-sm text-white">I understand the risks and want to proceed.</span>
      </label>

      <VisionButton 
        @click="lock" 
        :loading="isLocking" 
        :disabled="!confirmed"
        variant="danger" 
        icon="lock_reset" 
        class="w-full"
      >
        Lock Bootloader
      </VisionButton>

      <div v-if="message" class="p-3 rounded-lg text-center text-xs font-bold" :class="message.includes('Error') ? 'bg-error/20 text-error' : 'bg-success/20 text-success'">
        {{ message }}
      </div>
    </div>
  </GlassCard>
</template>
