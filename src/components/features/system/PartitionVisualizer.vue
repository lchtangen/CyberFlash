<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import GlassInput from '../../ui/GlassInput.vue';

const partitions = ref<Record<string, string>>({});
const loading = ref(false);
const error = ref<string | null>(null);
const searchQuery = ref('');

const fetchPartitions = async () => {
  loading.value = true;
  error.value = null;
  try {
    partitions.value = await invoke('get_var_all', { serial: null });
  } catch (e) {
    error.value = String(e);
    // Mock data for dev
    if (import.meta.env.DEV && !partitions.value) {
      partitions.value = {
        'product': 'OnePlus7Pro',
        'serialno': '892374892',
        'secure': 'yes',
        'unlocked': 'yes',
        'partition-size:boot_a': '0x4000000',
        'partition-type:boot_a': 'raw',
        'partition-size:system_a': '0x100000000',
        'partition-type:system_a': 'ext4',
        'version-baseband': '1.2.3.4',
        'version-bootloader': '4.5.6'
      };
    }
  } finally {
    loading.value = false;
  }
};

const filteredItems = computed(() => {
  if (!searchQuery.value) return partitions.value;
  const query = searchQuery.value.toLowerCase();
  const result: Record<string, string> = {};
  
  for (const [key, value] of Object.entries(partitions.value)) {
    if (key.toLowerCase().includes(query) || value.toLowerCase().includes(query)) {
      result[key] = value;
    }
  }
  return result;
});

const getCategoryColor = (key: string) => {
  if (key.includes('partition')) return 'text-primary';
  if (key.includes('version')) return 'text-success';
  if (key.includes('secure') || key.includes('lock')) return 'text-warning';
  return 'text-text-secondary';
};

onMounted(fetchPartitions);
</script>

<template>
  <GlassCard class="flex flex-col h-full min-h-[400px]">
    <div class="flex justify-between items-center mb-6">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-purple-500/10 flex items-center justify-center border border-purple-500/20 shadow-[0_0_15px_rgba(168,85,247,0.1)]">
          <span class="material-symbols-rounded text-purple-400 text-xl">dns</span>
        </div>
        <div>
          <h3 class="text-lg font-bold text-white tracking-wide">System Variables</h3>
          <p class="text-xs text-text-secondary font-medium">Bootloader configuration & partitions</p>
        </div>
      </div>
      
      <button 
        @click="fetchPartitions" 
        class="p-2 rounded-lg bg-white/5 hover:bg-white/10 border border-white/5 text-text-secondary hover:text-white transition-all active:scale-95"
        :disabled="loading"
        title="Refresh Variables"
      >
        <span class="material-symbols-rounded text-lg" :class="{ 'animate-spin': loading }">refresh</span>
      </button>
    </div>

    <!-- Search Bar -->
    <div class="mb-4">
      <GlassInput 
        v-model="searchQuery" 
        icon="search" 
        placeholder="Search variables (e.g. 'partition', 'version')..." 
        class="w-full"
      />
    </div>

    <div v-if="error" class="p-4 bg-error/10 border border-error/20 rounded-xl text-error text-sm mb-4 flex items-center gap-3">
      <span class="material-symbols-rounded">error</span>
      {{ error }}
    </div>

    <div v-if="loading && !Object.keys(partitions).length" class="flex-1 flex flex-col items-center justify-center py-12 opacity-50">
      <div class="w-10 h-10 border-2 border-primary border-t-transparent rounded-full animate-spin mb-4"></div>
      <p class="text-sm text-text-secondary">Reading device variables...</p>
    </div>

    <div v-else class="flex-1 overflow-hidden relative bg-black/20 rounded-xl border border-white/5">
      <div class="absolute inset-0 overflow-y-auto custom-scrollbar p-2 space-y-1">
        <div 
          v-for="(value, key) in filteredItems" 
          :key="key" 
          class="flex items-center justify-between text-sm p-3 hover:bg-white/5 rounded-lg group transition-colors border border-transparent hover:border-white/5"
        >
          <div class="flex items-center gap-3 overflow-hidden">
            <span class="material-symbols-rounded text-xs opacity-50 group-hover:opacity-100 transition-opacity">data_object</span>
            <span class="font-mono text-xs font-bold truncate" :class="getCategoryColor(String(key))">{{ key }}</span>
          </div>
          <span class="text-white/80 font-mono text-xs truncate max-w-[50%] bg-white/5 px-2 py-1 rounded select-all">{{ value }}</span>
        </div>
        
        <div v-if="Object.keys(filteredItems).length === 0" class="text-center text-text-muted py-12 flex flex-col items-center">
          <span class="material-symbols-rounded text-4xl mb-2 opacity-20">find_in_page</span>
          <p>No variables found matching your search.</p>
        </div>
      </div>
    </div>
    
    <!-- Footer Stats -->
    <div class="mt-4 flex items-center justify-between text-[10px] text-text-muted font-mono uppercase tracking-wider px-1">
      <span>Total Vars: {{ Object.keys(partitions).length }}</span>
      <span>Filtered: {{ Object.keys(filteredItems).length }}</span>
    </div>
  </GlassCard>
</template>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 10px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.2);
}
</style>
