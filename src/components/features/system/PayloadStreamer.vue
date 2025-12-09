<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';
import GlassInput from '../../ui/GlassInput.vue';

const url = ref('');
const targetFile = ref('boot.img');
const loading = ref(false);
const result = ref<string | null>(null);

const stream = async () => {
  if (!url.value) return;
  
  loading.value = true;
  result.value = null;
  
  try {
    const res = await invoke<string>('stream_payload_extraction', { 
      url: url.value, 
      targetFile: targetFile.value 
    });
    result.value = res;
  } catch (err: any) {
    result.value = `Error: ${err}`;
  } finally {
    loading.value = false;
  }
};
</script>

<template>
  <GlassCard class="h-full flex flex-col">
    <div class="flex items-center space-x-3 mb-6">
      <div class="p-2 rounded-lg bg-orange-500/20 text-orange-400">
        <span class="material-symbols-rounded text-xl">stream</span>
      </div>
      <div>
        <h3 class="text-lg font-semibold text-white">Payload Streamer</h3>
        <p class="text-xs text-text-secondary">Extract images without full download</p>
      </div>
    </div>

    <div class="space-y-4 flex-1">
      <GlassInput 
        v-model="url" 
        label="ROM URL (payload.bin inside zip)" 
        placeholder="https://..." 
      />
      
      <div class="space-y-2">
        <label class="text-xs font-medium text-text-secondary ml-1">Target File</label>
        <select v-model="targetFile" class="w-full bg-surface/50 border border-white/10 rounded-xl p-3 text-sm text-white focus:outline-none focus:border-primary/50">
          <option value="boot.img">boot.img</option>
          <option value="init_boot.img">init_boot.img</option>
          <option value="vendor_boot.img">vendor_boot.img</option>
          <option value="recovery.img">recovery.img</option>
          <option value="dtbo.img">dtbo.img</option>
        </select>
      </div>

      <div v-if="result" class="p-3 rounded-lg bg-success/10 border border-success/20 text-success text-sm">
        {{ result }}
      </div>
    </div>

    <div class="mt-6">
      <VisionButton 
        @click="stream" 
        :loading="loading"
        :disabled="!url"
        class="w-full"
      >
        Stream Extract
      </VisionButton>
    </div>
  </GlassCard>
</template>
