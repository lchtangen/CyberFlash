<template>
  <GlassCard class="relative overflow-hidden group">
    <!-- Header -->
    <div class="flex justify-between items-center mb-6 relative z-10">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-warning/10 flex items-center justify-center border border-warning/20 shadow-[0_0_15px_rgba(255,214,10,0.1)]">
          <span class="material-symbols-rounded text-warning text-xl">bolt</span>
        </div>
        <div>
          <h3 class="text-lg font-bold text-white tracking-wide">Power Monitor</h3>
          <p class="text-xs text-text-secondary font-medium">Real-time battery diagnostics</p>
        </div>
      </div>
      <div class="px-2 py-1 rounded-lg bg-white/5 border border-white/10 text-xs font-mono text-text-secondary">
        {{ stats?.status || 'Scanning...' }}
      </div>
    </div>

    <div v-if="loading && !stats" class="animate-pulse space-y-4">
      <div class="h-32 bg-white/5 rounded-2xl border border-white/5"></div>
      <div class="grid grid-cols-2 gap-3">
        <div class="h-20 bg-white/5 rounded-xl border border-white/5"></div>
        <div class="h-20 bg-white/5 rounded-xl border border-white/5"></div>
      </div>
    </div>

    <div v-else class="space-y-6 relative z-10">
      <!-- Main Power Gauge -->
      <div class="relative h-40 flex items-center justify-center bg-white/5 rounded-2xl border border-white/5 backdrop-blur-sm overflow-hidden">
        <!-- Background Grid -->
        <div class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-[size:20px_20px]"></div>
        
        <div class="text-center relative z-10">
          <div class="flex items-baseline justify-center gap-1">
            <span class="text-5xl font-bold text-white font-mono tracking-tighter drop-shadow-glow">{{ stats?.power_w.toFixed(1) }}</span>
            <span class="text-xl text-white/50 font-medium">W</span>
          </div>
          <div class="text-xs text-warning font-bold uppercase tracking-widest mt-1 flex items-center justify-center gap-1.5">
            <span class="w-1.5 h-1.5 rounded-full bg-warning animate-pulse"></span>
            Charging Speed
          </div>
        </div>
        
        <!-- Circular Progress -->
        <svg class="absolute inset-0 w-full h-full -rotate-90 pointer-events-none opacity-30">
          <circle cx="50%" cy="50%" r="35%" fill="none" stroke="rgba(255,255,255,0.05)" stroke-width="8" />
          <circle cx="50%" cy="50%" r="35%" fill="none" stroke="url(#gradient-power)" stroke-width="8" 
            stroke-dasharray="220" :stroke-dashoffset="220 - (220 * (Math.min(stats?.power_w || 0, 65) / 65))" 
            stroke-linecap="round"
            class="transition-all duration-1000 ease-out" />
          <defs>
            <linearGradient id="gradient-power" x1="0%" y1="0%" x2="100%" y2="0%">
              <stop offset="0%" stop-color="#FFD60A" />
              <stop offset="100%" stop-color="#FF9F0A" />
            </linearGradient>
          </defs>
        </svg>
      </div>

      <!-- Details Grid -->
      <div class="grid grid-cols-2 gap-3">
        <div class="bg-white/5 p-3 rounded-xl border border-white/5 hover:bg-white/10 transition-colors group/item">
          <div class="flex items-center gap-2 mb-1">
            <span class="material-symbols-rounded text-xs text-text-muted">electrical_services</span>
            <span class="text-xs text-text-muted uppercase tracking-wider font-bold">Voltage</span>
          </div>
          <div class="text-xl font-mono text-white group-hover/item:text-primary transition-colors">{{ stats?.voltage_v.toFixed(2) }}V</div>
        </div>
        
        <div class="bg-white/5 p-3 rounded-xl border border-white/5 hover:bg-white/10 transition-colors group/item">
          <div class="flex items-center gap-2 mb-1">
            <span class="material-symbols-rounded text-xs text-text-muted">electric_bolt</span>
            <span class="text-xs text-text-muted uppercase tracking-wider font-bold">Current</span>
          </div>
          <div class="text-xl font-mono text-white group-hover/item:text-warning transition-colors">{{ stats?.current_ma.toFixed(0) }}mA</div>
        </div>
        
        <div class="bg-white/5 p-3 rounded-xl border border-white/5 hover:bg-white/10 transition-colors group/item">
          <div class="flex items-center gap-2 mb-1">
            <span class="material-symbols-rounded text-xs text-text-muted">thermostat</span>
            <span class="text-xs text-text-muted uppercase tracking-wider font-bold">Temp</span>
          </div>
          <div class="text-xl font-mono text-white group-hover/item:text-error transition-colors">{{ stats?.temperature_c.toFixed(1) }}°C</div>
        </div>
        
        <div class="bg-white/5 p-3 rounded-xl border border-white/5 hover:bg-white/10 transition-colors group/item">
          <div class="flex items-center gap-2 mb-1">
            <span class="material-symbols-rounded text-xs text-text-muted">battery_full</span>
            <span class="text-xs text-text-muted uppercase tracking-wider font-bold">Level</span>
          </div>
          <div class="text-xl font-mono text-white group-hover/item:text-success transition-colors">{{ stats?.level_percent }}%</div>
        </div>
      </div>
    </div>
    
    <!-- Background Glow -->
    <div class="absolute -top-20 -right-20 w-64 h-64 bg-warning/5 rounded-full blur-3xl pointer-events-none"></div>
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
    // Mock data for development if backend fails or is not connected
    if (!stats.value) {
       stats.value = {
         voltage_v: 9.0,
         current_ma: 2000,
         power_w: 18.0,
         level_percent: 85,
         status: 'Charging',
         technology: 'Li-poly',
         temperature_c: 32.5
       };
    }
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

<style scoped>
.drop-shadow-glow {
  filter: drop-shadow(0 0 10px rgba(255, 214, 10, 0.3));
}
</style>
