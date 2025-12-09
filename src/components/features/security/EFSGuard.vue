<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';

const isBackingUp = ref(false);
const backupPath = ref<string | null>(null);
const error = ref<string | null>(null);

const startBackup = async () => {
  if (!confirm("This will backup critical EFS partitions (IMEI/Radio). Ensure device is rooted and authorized.")) return;
  
  isBackingUp.value = true;
  backupPath.value = null;
  error.value = null;
  
  try {
    const path = await invoke<string>('backup_efs_partitions');
    backupPath.value = path;
  } catch (e) {
    error.value = String(e);
  } finally {
    isBackingUp.value = false;
  }
};
</script>

<template>
  <GlassCard>
    <div class="flex items-center gap-3 mb-6">
      <div class="p-3 rounded-xl bg-error/20 text-error">
        <span class="material-symbols-rounded text-2xl">shield</span>
      </div>
      <div>
        <h2 class="text-xl font-bold text-white">EFS Guard</h2>
        <p class="text-sm text-white/60">Backup IMEI & Radio partitions.</p>
      </div>
    </div>

    <div class="space-y-4">
      <div class="bg-warning/10 border border-warning/20 p-4 rounded-xl">
        <p class="text-warning text-xs font-bold mb-1">CRITICAL WARNING</p>
        <p class="text-white/80 text-xs">
          Losing EFS partitions results in permanent loss of signal/IMEI. 
          Always backup before flashing custom ROMs.
        </p>
      </div>

      <VisionButton 
        @click="startBackup" 
        :loading="isBackingUp" 
        variant="danger" 
        icon="save" 
        class="w-full"
      >
        Backup EFS Now
      </VisionButton>

      <div v-if="backupPath" class="bg-success/10 border border-success/20 p-3 rounded-lg">
        <p class="text-success text-xs font-bold">Backup Successful!</p>
        <p class="text-white/60 text-xs break-all mt-1">Saved to device: {{ backupPath }}</p>
      </div>

      <div v-if="error" class="bg-error/10 border border-error/20 p-3 rounded-lg">
        <p class="text-error text-xs font-bold">Backup Failed</p>
        <p class="text-white/60 text-xs mt-1">{{ error }}</p>
      </div>
    </div>
  </GlassCard>
</template>
