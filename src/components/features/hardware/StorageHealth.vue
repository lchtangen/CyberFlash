<template>
  <GlassCard class="relative overflow-hidden">
    <div class="flex justify-between items-center mb-6 relative z-10">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-secondary/10 flex items-center justify-center border border-secondary/20 shadow-[0_0_15px_rgba(168,85,247,0.1)]">
          <span class="material-symbols-rounded text-secondary text-xl">hard_drive</span>
        </div>
        <div>
          <h3 class="text-lg font-bold text-white tracking-wide">Storage Health</h3>
          <p class="text-xs text-text-secondary font-medium">UFS / eMMC Diagnostics</p>
        </div>
      </div>
    </div>

    <div v-if="loading" class="animate-pulse space-y-4 relative z-10">
      <div class="h-24 bg-white/5 rounded-2xl border border-white/5"></div>
      <div class="h-12 bg-white/5 rounded-xl border border-white/5"></div>
      <div class="h-12 bg-white/5 rounded-xl border border-white/5"></div>
    </div>

    <div v-else-if="health" class="space-y-6 relative z-10">
      <!-- Overall Status -->
      <div class="flex items-center gap-5 p-5 rounded-2xl bg-white/5 border border-white/10 backdrop-blur-md">
        <div class="relative">
          <div class="w-16 h-16 rounded-full bg-success/20 flex items-center justify-center text-success border border-success/30 shadow-[0_0_20px_rgba(48,209,88,0.2)]">
            <span class="material-symbols-rounded text-3xl">check_circle</span>
          </div>
          <div class="absolute inset-0 rounded-full border border-success/50 animate-ping opacity-20"></div>
        </div>
        <div>
          <div class="text-2xl font-bold text-white tracking-tight">{{ health.health_desc }}</div>
          <div class="text-sm text-text-secondary font-medium">Overall Storage Status</div>
        </div>
      </div>

      <!-- Life Used Meters -->
      <div class="space-y-4">
        <div class="space-y-2">
          <div class="flex justify-between items-center text-xs font-bold uppercase tracking-wider text-text-secondary">
            <span>Life Used (SLC)</span>
            <span class="font-mono text-white">{{ health.life_used_a }}</span>
          </div>
          <div class="h-2 bg-white/10 rounded-full overflow-hidden">
            <div class="h-full bg-gradient-to-r from-success to-primary w-[10%] shadow-[0_0_10px_rgba(48,209,88,0.5)]"></div>
          </div>
        </div>

        <div class="space-y-2">
          <div class="flex justify-between items-center text-xs font-bold uppercase tracking-wider text-text-secondary">
            <span>Life Used (MLC)</span>
            <span class="font-mono text-white">{{ health.life_used_b }}</span>
          </div>
          <div class="h-2 bg-white/10 rounded-full overflow-hidden">
            <div class="h-full bg-gradient-to-r from-success to-primary w-[10%] shadow-[0_0_10px_rgba(48,209,88,0.5)]"></div>
          </div>
        </div>
      </div>

      <!-- Pre-EOL Info -->
      <div class="flex justify-between items-center p-4 rounded-xl bg-white/5 border border-white/5">
        <span class="text-sm text-text-secondary font-medium">Pre-EOL Information</span>
        <span class="font-mono text-sm font-bold text-success bg-success/10 px-2 py-1 rounded border border-success/20">{{ health.pre_eol_info }}</span>
      </div>
    </div>
    
    <!-- Background Mesh -->
    <div class="absolute inset-0 mesh-gradient-bg opacity-10 pointer-events-none"></div>
  </GlassCard>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';

interface StorageHealth {
  life_used_a: string;
  life_used_b: string;
  pre_eol_info: string;
  health_desc: string;
}

const health = ref<StorageHealth | null>(null);
const loading = ref(true);

onMounted(async () => {
  try {
    health.value = await invoke('get_storage_health');
  } catch (e) {
    console.error(e);
  } finally {
    loading.value = false;
  }
});
</script>
