<template>
  <GlassCard>
    <div class="flex justify-between items-center mb-6">
      <h3 class="text-lg font-bold text-white flex items-center gap-2">
        <i class="fas fa-bolt text-warning"></i>
        Power Monitor
      </h3>
      <div class="text-xs text-white/50 font-mono">
        {{ stats?.status || 'Unknown' }}
      </div>
    </div>

    <div v-if="loading && !stats" class="animate-pulse space-y-4">
      <div class="h-20 bg-white/10 rounded-xl"></div>
      <div class="h-20 bg-white/10 rounded-xl"></div>
    </div>

    <div v-else class="space-y-6">
      <!-- Main Power Gauge -->
      <div class="relative h-32 flex items-center justify-center">
        <div class="text-center">
          <div class="text-4xl font-bold text-white font-mono tracking-tighter">
            {{ stats?.power_w.toFixed(2) }}<span class="text-lg text-white/50 ml-1">W</span>
          </div>
          <div class="text-xs text-white/50 mt-1 uppercase tracking-wider">Charging Speed</div>
        </div>
        
        <!-- Circular Progress (CSS only for simplicity) -->
        <svg class="absolute inset-0 w-full h-full -rotate-90 pointer-events-none">
          <circle cx="50%" cy="50%" r="45%" fill="none" stroke="rgba(255,255,255,0.1)" stroke-width="4" />
          <circle cx="50%" cy="50%" r="45%" fill="none" stroke="var(--color-warning)" stroke-width="4" 
            stroke-dasharray="283" :stroke-dashoffset="283 - (283 * (Math.min(stats?.power_w || 0, 65) / 65))" 
            class="transition-all duration-1000 ease-out" />
        </svg>
      </div>

      <!-- Details Grid -->
      <div class="grid grid-cols-2 gap-4">
        <div class="bg-surface/50 p-3 rounded-lg border border-white/5">
          <div class="text-xs text-white/50 mb-1">Voltage</div>
          <div class="text-xl font-mono text-white">{{ stats?.voltage_v.toFixed(2) }}V</div>
        </div>
        <div class="bg-surface/50 p-3 rounded-lg border border-white/5">
          <div class="text-xs text-white/50 mb-1">Current</div>
          <div class="text-xl font-mono text-white">{{ stats?.current_ma.toFixed(0) }}mA</div>
        </div>
        <div class="bg-surface/50 p-3 rounded-lg border border-white/5">
          <div class="text-xs text-white/50 mb-1">Temp</div>
          <div class="text-xl font-mono text-white">{{ stats?.temperature_c.toFixed(1) }}°C</div>
        </div>
        <div class="bg-surface/50 p-3 rounded-lg border border-white/5">
          <div class="text-xs text-white/50 mb-1">Level</div>
          <div class="text-xl font-mono text-white">{{ stats?.level_percent }}%</div>
        </div>
      </div>
    </div>
  </GlassCard>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '@/components/ui/GlassCard.vue';

interface PowerStats {
  voltage_v: number;
  current_ma: number;
  power_w: number;
  level_percent: number;
  status: string;
  technology: string;
  temperature_c: number;
}

const stats = ref<PowerStats | null>(null);
const loading = ref(true);
let interval: number | null = null;

const fetchStats = async () => {
  try {
    stats.value = await invoke('get_power_stats');
  } catch (e) {
    console.error(e);
  } finally {
    loading.value = false;
  }
};

onMounted(() => {
  fetchStats();
  interval = window.setInterval(fetchStats, 2000);
});

onUnmounted(() => {
  if (interval) clearInterval(interval);
});
</script>
