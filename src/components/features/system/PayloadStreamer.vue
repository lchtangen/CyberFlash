<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';
import GlassInput from '../../ui/GlassInput.vue';
import GlassSelect from '../../ui/GlassSelect.vue';

const url = ref('');
const targetFile = ref('boot.img');
const loading = ref(false);
const result = ref<string | null>(null);

const targetOptions = [
  { value: 'boot.img', label: 'boot.img' },
  { value: 'init_boot.img', label: 'init_boot.img' },
  { value: 'vendor_boot.img', label: 'vendor_boot.img' },
  { value: 'recovery.img', label: 'recovery.img' },
  { value: 'dtbo.img', label: 'dtbo.img' }
];

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
    <div class="flex items-center gap-3 mb-6">
      <div class="p-3 rounded-xl bg-warning/20 text-warning">
        <span class="material-symbols-rounded text-2xl">stream</span>
      </div>
      <div>
        <h2 class="text-xl font-bold text-white">Payload Streamer</h2>
        <p class="text-sm text-white/60">Extract images without full download</p>
      </div>
    </div>

    <div class="space-y-6 flex-1">
      <GlassInput 
        v-model="url" 
        label="ROM URL (payload.bin inside zip)" 
        placeholder="https://example.com/rom.zip" 
        icon="link"
      />
      
      <GlassSelect
        v-model="targetFile"
        label="Target File"
        :options="targetOptions"
        icon="description"
      />

      <div v-if="result" class="p-3 rounded-lg text-sm" :class="result.includes('Error') ? 'bg-error/10 border border-error/20 text-error' : 'bg-success/10 border border-success/20 text-success'">
        {{ result }}
      </div>
    </div>

    <div class="mt-6">
      <VisionButton 
        @click="stream" 
        :loading="loading"
        :disabled="!url"
        icon="cloud_download"
        class="w-full"
      >
        Stream Extract
      </VisionButton>
    </div>
  </GlassCard>
</template>
