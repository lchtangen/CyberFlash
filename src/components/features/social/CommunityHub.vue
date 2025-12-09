<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <h2 class="text-2xl font-bold text-white">Community Repository</h2>
      <div class="flex gap-2">
        <VisionButton variant="secondary" @click="refreshRepo" icon="refresh" :loading="loading">
          Refresh
        </VisionButton>
      </div>
    </div>

    <div v-if="loading" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <GlassCard v-for="i in 3" :key="i" class="h-48 animate-pulse">
        <div class="h-full flex flex-col justify-between">
          <div class="space-y-3">
            <div class="h-6 w-3/4 bg-white/10 rounded"></div>
            <div class="h-4 w-1/2 bg-white/10 rounded"></div>
          </div>
        </div>
      </GlassCard>
    </div>

    <div v-else-if="error" class="p-6 rounded-xl bg-error/10 border border-error/20 text-error">
      <div class="flex items-center gap-3">
        <span class="material-symbols-rounded text-xl">warning</span>
        <span>{{ error }}</span>
      </div>
      <VisionButton class="mt-4" variant="secondary" @click="refreshRepo">Retry</VisionButton>
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <GlassCard 
        v-for="rom in roms" 
        :key="rom.id"
        class="group hover:border-primary/50 transition-colors"
      >
        <div class="flex flex-col h-full">
          <div class="flex justify-between items-start mb-4">
            <div>
              <h3 class="text-lg font-bold text-white group-hover:text-primary transition-colors">
                {{ rom.name }}
              </h3>
              <p class="text-sm text-white/60">{{ rom.version }} • {{ rom.device }}</p>
            </div>
            <div class="flex items-center gap-1 text-xs text-success bg-success/10 px-2 py-1 rounded-full">
              <span class="material-symbols-rounded text-sm">thumb_up</span>
              <span>{{ rom.likes }}</span>
            </div>
          </div>

          <p class="text-sm text-white/70 mb-6 flex-grow line-clamp-3">
            {{ rom.description }}
          </p>

          <div class="flex items-center justify-between mt-auto pt-4 border-t border-white/5">
            <div class="flex items-center gap-2 text-xs text-white/50">
              <span class="material-symbols-rounded text-sm">person</span>
              <span>{{ rom.author }}</span>
            </div>
            <VisionButton size="sm" @click="openUrl(rom.download_url)" icon="open_in_new">
              Download
            </VisionButton>
          </div>
        </div>
      </GlassCard>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-shell';
import GlassCard from '@/components/ui/GlassCard.vue';
import VisionButton from '@/components/ui/VisionButton.vue';

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
const loading = ref(true);
const error = ref<string | null>(null);

const refreshRepo = async () => {
  loading.value = true;
  error.value = null;
  try {
    roms.value = await invoke('fetch_community_repos');
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
};

const openUrl = async (url: string) => {
  await open(url);
};

onMounted(refreshRepo);
</script>
