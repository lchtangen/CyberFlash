<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';
import GlassInput from '../../ui/GlassInput.vue';

const partition = ref('userdata');
const size = ref('');
const loading = ref(false);
const result = ref<string | null>(null);
const error = ref<string | null>(null);

const resize = async () => {
  if (!size.value) return;
  
  loading.value = true;
  result.value = null;
  error.value = null;
  
  try {
    const res = await invoke<string>('resize_partition', { 
      partition: partition.value, 
      sizeMb: parseInt(size.value) 
    });
    result.value = res;
  } catch (err: any) {
    error.value = err.toString();
  } finally {
    loading.value = false;
  }
};
</script>

<template>
  <GlassCard class="h-full flex flex-col">
    <div class="flex items-center space-x-3 mb-6">
      <div class="p-2 rounded-lg bg-red-500/20 text-red-400">
        <span class="material-symbols-rounded text-xl">aspect_ratio</span>
      </div>
      <div>
        <h3 class="text-lg font-semibold text-white">Partition Resizer</h3>
        <p class="text-xs text-text-secondary">Dynamic partition resizing (Dangerous)</p>
      </div>
    </div>

    <div class="space-y-4 flex-1">
      <div class="p-3 rounded-lg bg-warning/10 border border-warning/20 text-warning text-xs">
        <span class="font-bold">WARNING:</span> This operation can cause data loss. Ensure you have a backup. Device must be in Recovery mode.
      </div>

      <div class="space-y-2">
        <label class="text-xs font-medium text-text-secondary ml-1">Partition</label>
        <select v-model="partition" class="w-full bg-surface/50 border border-white/10 rounded-xl p-3 text-sm text-white focus:outline-none focus:border-primary/50">
          <option value="userdata">userdata</option>
          <option value="system">system</option>
          <option value="vendor">vendor</option>
        </select>
      </div>

      <GlassInput 
        v-model="size" 
        label="New Size (MB)" 
        placeholder="e.g., 64000" 
        type="number"
      />

      <div v-if="error" class="text-error text-sm">{{ error }}</div>
      <div v-if="result" class="text-success text-sm">{{ result }}</div>
    </div>

    <div class="mt-6">
      <VisionButton 
        @click="resize" 
        :loading="loading"
        :disabled="!size"
        variant="danger"
        class="w-full"
      >
        Resize Partition
      </VisionButton>
    </div>
  </GlassCard>
</template>
