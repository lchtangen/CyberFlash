<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const partitions = ref<Record<string, string>>({});
const loading = ref(false);
const error = ref<string | null>(null);

const fetchPartitions = async () => {
  loading.value = true;
  error.value = null;
  try {
    partitions.value = await invoke('get_var_all', { serial: null });
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
};

onMounted(fetchPartitions);

// Helper to format size if available (often fastboot returns raw hex or blocks)
// For now, we just display raw values as returned by getvar all
</script>

<template>
  <div class="bg-surface/30 border border-white/10 rounded-xl p-4 backdrop-blur-md">
    <div class="flex justify-between items-center mb-4">
      <h3 class="text-lg font-semibold text-white">Partition Layout</h3>
      <button 
        @click="fetchPartitions" 
        class="p-2 rounded-lg hover:bg-white/10 text-text-secondary hover:text-white transition-colors"
        :disabled="loading"
      >
        <span class="material-symbols-rounded text-sm" :class="{ 'animate-spin': loading }">refresh</span>
      </button>
    </div>

    <div v-if="error" class="p-3 bg-error/10 border border-error/20 rounded-lg text-error text-sm mb-4">
      {{ error }}
    </div>

    <div v-if="loading" class="flex justify-center py-8">
      <div class="w-6 h-6 border-2 border-primary border-t-transparent rounded-full animate-spin"></div>
    </div>

    <div v-else class="space-y-2 max-h-60 overflow-y-auto custom-scrollbar">
      <div v-for="(value, key) in partitions" :key="key" class="flex justify-between text-sm p-2 hover:bg-white/5 rounded">
        <span class="text-text-secondary font-mono">{{ key }}</span>
        <span class="text-white font-mono truncate max-w-[50%]">{{ value }}</span>
      </div>
      <div v-if="Object.keys(partitions).length === 0" class="text-center text-text-muted py-4">
        No partition data available. Ensure device is in Fastboot mode.
      </div>
    </div>
  </div>
</template>
