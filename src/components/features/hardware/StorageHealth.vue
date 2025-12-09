<template>
  <GlassCard>
    <div class="flex justify-between items-center mb-6">
      <h3 class="text-lg font-bold text-white flex items-center gap-2">
        <i class="fas fa-hdd text-secondary"></i>
        Storage Health
      </h3>
      <div class="text-xs text-white/50">UFS / eMMC</div>
    </div>

    <div v-if="loading" class="animate-pulse space-y-4">
      <div class="h-12 bg-white/10 rounded"></div>
      <div class="h-12 bg-white/10 rounded"></div>
    </div>

    <div v-else-if="health" class="space-y-4">
      <div class="flex items-center gap-4 p-4 rounded-xl bg-white/5 border border-white/10">
        <div class="w-12 h-12 rounded-full bg-success/20 flex items-center justify-center text-success text-xl">
          <i class="fas fa-check-circle"></i>
        </div>
        <div>
          <div class="font-bold text-white">{{ health.health_desc }}</div>
          <div class="text-xs text-white/50">Overall Status</div>
        </div>
      </div>

      <div class="space-y-3">
        <div class="flex justify-between items-center text-sm">
          <span class="text-white/70">Life Used (SLC)</span>
          <span class="font-mono text-white">{{ health.life_used_a }}</span>
        </div>
        <div class="w-full bg-white/10 h-1.5 rounded-full overflow-hidden">
          <div class="bg-success h-full" style="width: 10%"></div>
        </div>

        <div class="flex justify-between items-center text-sm pt-2">
          <span class="text-white/70">Life Used (MLC)</span>
          <span class="font-mono text-white">{{ health.life_used_b }}</span>
        </div>
        <div class="w-full bg-white/10 h-1.5 rounded-full overflow-hidden">
          <div class="bg-success h-full" style="width: 10%"></div>
        </div>

        <div class="flex justify-between items-center text-sm pt-2 border-t border-white/5 mt-2">
          <span class="text-white/70">Pre-EOL Info</span>
          <span class="font-mono text-success">{{ health.pre_eol_info }}</span>
        </div>
      </div>
    </div>
  </GlassCard>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '@/components/ui/GlassCard.vue';

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
