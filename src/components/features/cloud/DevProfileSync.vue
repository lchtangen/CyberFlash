<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';

const lastSync = ref<string | null>(null);
const loading = ref(false);
const status = ref<string | null>(null);

const syncProfile = async (action: 'upload' | 'download') => {
  loading.value = true;
  status.value = null;
  
  try {
    const data = action === 'upload' ? JSON.stringify({ timestamp: Date.now() }) : undefined;
    const result = await invoke<string>('sync_dev_profile', { action, data });
    status.value = result;
    lastSync.value = new Date().toLocaleTimeString();
  } catch (err: any) {
    status.value = `Error: ${err}`;
  } finally {
    loading.value = false;
  }
};
</script>

<template>
  <GlassCard class="h-full flex flex-col">
    <div class="flex items-center space-x-3 mb-6">
      <div class="p-2 rounded-lg bg-blue-500/20 text-blue-400">
        <span class="material-symbols-rounded text-xl">sync</span>
      </div>
      <div>
        <h3 class="text-lg font-semibold text-white">Dev Profile Sync</h3>
        <p class="text-xs text-text-secondary">Sync your environment across devices</p>
      </div>
    </div>

    <div class="flex-1 flex flex-col justify-center items-center space-y-6">
      <div class="text-center">
        <div class="text-4xl mb-2">☁️</div>
        <p class="text-sm text-text-secondary">
          Keep your ADB paths, favorite ROMs, and theme settings in sync.
        </p>
      </div>

      <div class="grid grid-cols-2 gap-4 w-full">
        <VisionButton 
          @click="syncProfile('upload')" 
          variant="secondary" 
          :disabled="loading"
          class="flex justify-center"
        >
          <span class="material-symbols-rounded mr-2">cloud_upload</span>
          Upload
        </VisionButton>
        
        <VisionButton 
          @click="syncProfile('download')" 
          variant="secondary" 
          :disabled="loading"
          class="flex justify-center"
        >
          <span class="material-symbols-rounded mr-2">cloud_download</span>
          Download
        </VisionButton>
      </div>

      <div v-if="status" class="text-xs text-center p-2 rounded bg-white/5 w-full">
        {{ status }}
      </div>
      
      <div v-if="lastSync" class="text-xs text-text-secondary">
        Last synced: {{ lastSync }}
      </div>
    </div>
  </GlassCard>
</template>
