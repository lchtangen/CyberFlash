<template>
  <div class="h-full flex flex-col space-y-6 p-6">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-2xl font-bold text-white">Cloud & Backup</h2>
        <p class="text-white/60">Backup apps and data to local storage or cloud</p>
      </div>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <!-- App Backup -->
      <GlassCard class="p-6">
        <h3 class="text-lg font-bold text-white mb-4">App Backup</h3>
        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-white/60 mb-2">Package Name</label>
            <GlassInput v-model="backupPackage" placeholder="com.example.app" />
          </div>
          <VisionButton 
            @click="backupApp" 
            variant="primary" 
            class="w-full"
            :disabled="loading"
          >
            <Icon v-if="loading" icon="mdi:loading" class="animate-spin mr-2" />
            <Icon v-else icon="mdi:cloud-upload" class="mr-2" />
            Backup App Data
          </VisionButton>
        </div>
      </GlassCard>

      <!-- Restore -->
      <GlassCard class="p-6">
        <h3 class="text-lg font-bold text-white mb-4">Restore Backup</h3>
        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-white/60 mb-2">Backup File (.ab)</label>
            <div class="flex space-x-2">
              <GlassInput v-model="restorePath" placeholder="/path/to/backup.ab" readonly />
              <VisionButton @click="selectBackup" variant="secondary">Browse</VisionButton>
            </div>
          </div>
          <VisionButton 
            @click="restoreApp" 
            variant="warning" 
            class="w-full"
            :disabled="loading"
          >
            <Icon v-if="loading" icon="mdi:loading" class="animate-spin mr-2" />
            <Icon v-else icon="mdi:cloud-download" class="mr-2" />
            Restore Data
          </VisionButton>
        </div>
      </GlassCard>
    </div>
    
    <!-- Status Log -->
    <GlassCard class="flex-1 p-4 font-mono text-xs overflow-y-auto">
      <div v-for="(log, i) in logs" :key="i" class="text-white/70 mb-1">
        {{ log }}
      </div>
    </GlassCard>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { Icon } from '@iconify/vue';
import GlassCard from '../../ui/GlassCard.vue';
import GlassInput from '../../ui/GlassInput.vue';
import VisionButton from '../../ui/VisionButton.vue';

const backupPackage = ref('');
const restorePath = ref('');
const loading = ref(false);
const logs = ref<string[]>([]);

const log = (msg: string) => logs.value.push(`[${new Date().toLocaleTimeString()}] ${msg}`);

const backupApp = async () => {
  if (!backupPackage.value) return;
  loading.value = true;
  log(`Starting backup for ${backupPackage.value}... Please confirm on device.`);
  
  try {
    const path = await invoke<string>('backup_app_data', { package: backupPackage.value });
    log(`Backup successful: ${path}`);
  } catch (e) {
    log(`Backup failed: ${e}`);
  } finally {
    loading.value = false;
  }
};

const selectBackup = async () => {
  const selected = await open({
    multiple: false,
    filters: [{ name: 'Android Backup', extensions: ['ab'] }]
  });
  
  if (selected && typeof selected === 'string') {
    restorePath.value = selected;
  }
};

const restoreApp = async () => {
  if (!restorePath.value) return;
  loading.value = true;
  log(`Starting restore from ${restorePath.value}... Please confirm on device.`);
  
  try {
    const msg = await invoke<string>('restore_app_data', { backupPath: restorePath.value });
    log(msg);
  } catch (e) {
    log(`Restore failed: ${e}`);
  } finally {
    loading.value = false;
  }
};
</script>
