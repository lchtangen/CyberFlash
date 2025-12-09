<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';

interface CommunityRom {
  id: string;
  name: string;
  device: string;
  version: string;
  author: string;
  download_url: string;
  description: string;
  likes: number;
}

const roms = ref<CommunityRom[]>([]);
const loading = ref(false);
const error = ref<string | null>(null);

const fetchRoms = async () => {
  loading.value = true;
  error.value = null;
  try {
    roms.value = await invoke('fetch_community_repos');
  } catch (err: any) {
    error.value = err.toString();
  } finally {
    loading.value = false;
  }
};

const downloadRom = (url: string) => {
  // In a real app, this would trigger the download manager
  console.log('Downloading:', url);
  alert(`Started download for ${url}`);
};

onMounted(() => {
  fetchRoms();
});
</script>

<template>
  <GlassCard class="h-full flex flex-col">
    <div class="flex justify-between items-center mb-6">
      <div class="flex items-center space-x-3">
        <div class="p-2 rounded-lg bg-primary/20 text-primary">
          <span class="material-symbols-rounded text-xl">public</span>
        </div>
        <div>
          <h3 class="text-lg font-semibold text-white">Community ROM Repo</h3>
          <p class="text-xs text-text-secondary">Discover and download community builds</p>
        </div>
      </div>
      <VisionButton @click="fetchRoms" variant="secondary" size="sm" :disabled="loading">
        <span class="material-symbols-rounded text-lg animate-spin" v-if="loading">refresh</span>
        <span class="material-symbols-rounded text-lg" v-else>refresh</span>
      </VisionButton>
    </div>

    <div v-if="error" class="p-4 rounded-lg bg-error/10 border border-error/20 text-error text-sm mb-4">
      {{ error }}
    </div>

    <div v-if="loading && roms.length === 0" class="flex-1 flex items-center justify-center">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
    </div>

    <div v-else class="grid grid-cols-1 gap-4 overflow-y-auto custom-scrollbar pr-2 max-h-[400px]">
      <div 
        v-for="rom in roms" 
        :key="rom.id"
        class="p-4 rounded-xl bg-surface/40 border border-white/5 hover:bg-surface/60 transition-colors group"
      >
        <div class="flex justify-between items-start">
          <div>
            <h4 class="font-medium text-white group-hover:text-primary transition-colors">{{ rom.name }}</h4>
            <div class="flex items-center space-x-2 text-xs text-text-secondary mt-1">
              <span class="px-1.5 py-0.5 rounded bg-white/5">{{ rom.device }}</span>
              <span class="px-1.5 py-0.5 rounded bg-white/5">v{{ rom.version }}</span>
              <span>by {{ rom.author }}</span>
            </div>
          </div>
          <div class="flex items-center space-x-1 text-xs text-success bg-success/10 px-2 py-1 rounded-full">
            <span class="material-symbols-rounded text-sm">thumb_up</span>
            <span>{{ rom.likes }}</span>
          </div>
        </div>
        
        <p class="text-sm text-text-secondary mt-3 line-clamp-2">{{ rom.description }}</p>
        
        <div class="mt-4 flex justify-end">
          <button 
            @click="downloadRom(rom.download_url)"
            class="text-xs font-medium text-primary hover:text-primary-hover flex items-center space-x-1"
          >
            <span class="material-symbols-rounded text-sm">download</span>
            <span>Download</span>
          </button>
        </div>
      </div>
    </div>
  </GlassCard>
</template>
