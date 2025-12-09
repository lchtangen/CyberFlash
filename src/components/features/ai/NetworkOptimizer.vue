<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';

interface MirrorResult {
  url: string;
  latency: number;
  status: string;
}

interface MirrorReport {
  fastest_mirror: string;
  latency_ms: number;
  all_results: MirrorResult[];
}

const report = ref<MirrorReport | null>(null);
const isLoading = ref(false);

async function optimize() {
  isLoading.value = true;
  try {
    report.value = await invoke<MirrorReport>('optimize_mirrors');
  } catch (e) {
    console.error(e);
  } finally {
    isLoading.value = false;
  }
}

onMounted(optimize);
</script>

<template>
  <GlassCard>
    <div class="flex items-center gap-3 mb-6">
      <div class="p-3 rounded-xl bg-primary/20 text-primary">
        <span class="material-symbols-rounded text-2xl">network_check</span>
      </div>
      <div>
        <h2 class="text-xl font-bold text-white">Network Optimizer</h2>
        <p class="text-sm text-white/60">Find fastest download mirrors.</p>
      </div>
    </div>

    <div v-if="isLoading" class="flex justify-center py-8">
      <div class="w-8 h-8 border-2 border-primary border-t-transparent rounded-full animate-spin"></div>
    </div>

    <div v-else-if="report" class="space-y-4">
      <div class="p-4 rounded-xl bg-success/10 border border-success/20">
        <div class="text-sm text-success mb-1">Fastest Mirror Selected</div>
        <div class="font-mono text-white truncate">{{ report.fastest_mirror }}</div>
        <div class="text-xs text-white/60 mt-1">{{ report.latency_ms }}ms latency</div>
      </div>

      <div class="space-y-2">
        <div v-for="(mirror, i) in report.all_results" :key="i" class="flex justify-between items-center text-sm p-2 rounded hover:bg-white/5">
          <span class="text-white/80 truncate flex-1 mr-4">{{ mirror.url }}</span>
          <span class="font-mono" :class="mirror.latency < 100 ? 'text-success' : 'text-warning'">{{ mirror.latency }}ms</span>
        </div>
      </div>
      
      <VisionButton size="sm" class="w-full" @click="optimize">Re-Scan</VisionButton>
    </div>
  </GlassCard>
</template>
