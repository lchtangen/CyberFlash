<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';
import GlassInput from '../../ui/GlassInput.vue';

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
const searchQuery = ref('');

const fetchRoms = async () => {
  loading.value = true;
  error.value = null;
  try {
    if (searchQuery.value) {
      const allRoms = await invoke<CommunityRom[]>('fetch_community_repos');
      roms.value = allRoms.filter(r => 
        r.name.toLowerCase().includes(searchQuery.value.toLowerCase()) || 
        r.description.toLowerCase().includes(searchQuery.value.toLowerCase())
      );
    } else {
      roms.value = await invoke('fetch_community_repos');
    }
  } catch (err: any) {
    error.value = err.toString();
  } finally {
    loading.value = false;
  }
};

const downloadRom = async (rom: CommunityRom) => {
  try {
    const path = await invoke('download_feed_item', { 
      url: rom.download_url, 
      title: rom.name 
    });
    alert(`Download started/saved to: ${path}`);
  } catch (e) {
    alert('Download failed: ' + e);
  }
};

watch(searchQuery, () => {
  fetchRoms();
});

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
          <h2 class="text-lg font-bold text-white">Community Repos</h2>
          <p class="text-xs text-white/60">Verified ROMs & Kernels</p>
        </div>
      </div>
      <div class="w-64">
        <GlassInput v-model="searchQuery" placeholder="Search ROMs..." icon="search" />
      </div>
    </div>

    <div v-if="loading" class="flex-1 flex items-center justify-center">
      <span class="material-symbols-rounded animate-spin text-3xl text-primary">sync</span>
    </div>

    <div v-else-if="error" class="p-4 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm">
      {{ error }}
    </div>

    <div v-else class="flex-1 overflow-y-auto space-y-3 custom-scrollbar pr-2">
      <div 
        v-for="rom in roms" 
        :key="rom.id"
        class="group p-4 rounded-xl bg-white/5 border border-white/5 hover:bg-white/10 transition-all duration-300"
      >
        <div class="flex justify-between items-start">
          <div>
            <div class="flex items-center space-x-2">
              <h3 class="font-bold text-white group-hover:text-primary transition-colors">{{ rom.name }}</h3>
              <span class="px-2 py-0.5 rounded text-[10px] font-bold bg-white/10 text-white/60">{{ rom.version }}</span>
            </div>
            <p class="text-xs text-white/40 mt-1">for {{ rom.device }} • by {{ rom.author }}</p>
            <p class="text-sm text-white/70 mt-2 line-clamp-2">{{ rom.description }}</p>
          </div>
          <VisionButton @click="downloadRom(rom)" size="sm" variant="secondary" class="opacity-0 group-hover:opacity-100 transition-opacity">
            Download
          </VisionButton>
        </div>
      </div>
    </div>
  </GlassCard>
</template>
