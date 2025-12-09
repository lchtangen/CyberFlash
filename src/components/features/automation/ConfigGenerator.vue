<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';
import GlassInput from '../../ui/GlassInput.vue';

const url = ref('');
const generatedConfig = ref('');
const isLoading = ref(false);
const error = ref('');

async function generateConfig() {
  if (!url.value) return;
  
  isLoading.value = true;
  error.value = '';
  generatedConfig.value = '';

  try {
    const result = await invoke<string>('generate_config_from_url', { url: url.value });
    generatedConfig.value = result;
  } catch (e) {
    error.value = String(e);
  } finally {
    isLoading.value = false;
  }
}

function copyToClipboard() {
  navigator.clipboard.writeText(generatedConfig.value);
  // Could add a toast here
}

function useMockUrl() {
  url.value = 'https://mock-xda-developers.com/oneplus-7-pro/lineage-os-21';
}
</script>

<template>
  <GlassCard class="w-full max-w-2xl mx-auto">
    <div class="flex items-center gap-3 mb-6">
      <div class="p-3 rounded-xl bg-primary/20 text-primary">
        <span class="material-symbols-rounded text-2xl">auto_fix</span>
      </div>
      <div>
        <h2 class="text-xl font-bold text-white">AI Config Generator</h2>
        <p class="text-sm text-white/60">Paste an XDA thread URL to auto-generate a Flash Config.</p>
      </div>
    </div>

    <div class="space-y-4">
      <div class="flex gap-2">
        <GlassInput 
          v-model="url" 
          placeholder="https://forum.xda-developers.com/..." 
          class="flex-1"
        />
        <VisionButton @click="generateConfig" :disabled="isLoading || !url">
          <span v-if="isLoading" class="animate-spin material-symbols-rounded">sync</span>
          <span v-else>Generate</span>
        </VisionButton>
      </div>
      
      <div class="text-xs text-white/40 cursor-pointer hover:text-primary transition-colors" @click="useMockUrl">
        Try with mock URL
      </div>

      <div v-if="error" class="p-4 rounded-xl bg-error/10 border border-error/20 text-error text-sm">
        {{ error }}
      </div>

      <div v-if="generatedConfig" class="relative group">
        <div class="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity">
          <button 
            @click="copyToClipboard"
            class="p-2 rounded-lg bg-surface/80 hover:bg-surface text-white/80 hover:text-white transition-colors"
            title="Copy to Clipboard"
          >
            <span class="material-symbols-rounded text-sm">content_copy</span>
          </button>
        </div>
        <pre class="p-4 rounded-xl bg-black/40 border border-white/5 text-sm font-mono text-success overflow-x-auto custom-scrollbar">{{ generatedConfig }}</pre>
      </div>
    </div>
  </GlassCard>
</template>
