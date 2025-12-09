<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';
import GlassInput from '../../ui/GlassInput.vue';

const configName = ref('');
const configContent = ref('');
const shareId = ref<string | null>(null);
const loading = ref(false);
const error = ref<string | null>(null);

const shareConfig = async () => {
  if (!configName.value || !configContent.value) return;
  
  loading.value = true;
  error.value = null;
  shareId.value = null;
  
  try {
    const id = await invoke<string>('share_config', { 
      name: configName.value, 
      content: configContent.value 
    });
    shareId.value = id;
  } catch (err: any) {
    error.value = err.toString();
  } finally {
    loading.value = false;
  }
};

const copyId = () => {
  if (shareId.value) {
    navigator.clipboard.writeText(shareId.value);
  }
};
</script>

<template>
  <GlassCard class="h-full flex flex-col">
    <div class="flex items-center space-x-3 mb-6">
      <div class="p-2 rounded-lg bg-warning/20 text-warning">
        <span class="material-symbols-rounded text-xl">share</span>
      </div>
      <div>
        <h3 class="text-lg font-semibold text-white">Config Sharer</h3>
        <p class="text-xs text-text-secondary">Share your flash setups with others</p>
      </div>
    </div>

    <div class="space-y-4 flex-1">
      <GlassInput 
        v-model="configName" 
        label="Config Name" 
        placeholder="e.g., My Gaming Setup" 
      />
      
      <div class="space-y-1">
        <label class="text-xs font-medium text-text-secondary ml-1">Config Content (YAML)</label>
        <textarea 
          v-model="configContent"
          class="w-full h-32 bg-surface/50 border border-white/10 rounded-xl p-3 text-sm text-white font-mono focus:outline-none focus:border-primary/50 resize-none"
          placeholder="Paste your flash_config.yaml content here..."
        ></textarea>
      </div>

      <div v-if="error" class="text-error text-sm">{{ error }}</div>

      <div v-if="shareId" class="p-3 rounded-lg bg-success/10 border border-success/20 flex justify-between items-center">
        <div>
          <div class="text-xs text-success font-medium">Config Shared!</div>
          <div class="text-sm text-white font-mono">{{ shareId }}</div>
        </div>
        <button @click="copyId" class="p-2 hover:bg-white/10 rounded-lg transition-colors text-white">
          <span class="material-symbols-rounded">content_copy</span>
        </button>
      </div>
    </div>

    <div class="mt-6">
      <VisionButton 
        @click="shareConfig" 
        :loading="loading"
        :disabled="!configName || !configContent"
        class="w-full"
      >
        Generate Share Link
      </VisionButton>
    </div>
  </GlassCard>
</template>
