<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';

const powerStats = ref({
  voltage_v: 0,
  current_ma: 0,
  power_w: 0,
  level_percent: 0,
  status: 'Unknown',
  technology: 'Unknown',
  temperature_c: 0,
});

const thermalStats = ref({
  cpu_temp: 0,
  battery_temp: 0,
  skin_temp: 0,
  throttling: false,
});

const loading = ref(true);
let interval: number | null = null;

const fetchData = async () => {
  try {
    const [power, thermal] = await Promise.all([
      invoke('get_power_stats'),
      invoke('get_thermal_stats'),
    ]);
    powerStats.value = power as any;
    thermalStats.value = thermal as any;
  } catch (e) {
    console.error('Failed to fetch telemetry:', e);
  } finally {
    loading.value = false;
  }
};

onMounted(() => {
  fetchData();
  interval = window.setInterval(fetchData, 3000);
});

onUnmounted(() => {
  if (interval) clearInterval(interval);
});

const getTempColor = (temp: number) => {
  if (temp < 35) return 'text-success';
  if (temp < 45) return 'text-warning';
  return 'text-error';
};
</script>

<template>
  <GlassCard class="h-full flex flex-col">
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-sm font-bold text-white flex items-center gap-2">
        <span class="material-symbols-rounded text-primary">monitoring</span>
        Live Telemetry
      </h3>
      <div v-if="loading" class="w-4 h-4 border-2 border-white/20 border-t-primary rounded-full animate-spin"></div>
    </div>

    <div class="grid grid-cols-2 gap-4">
      <!-- Battery -->
      <div class="bg-white/5 rounded-xl p-3 border border-white/5">
        <div class="text-[10px] text-white/50 uppercase tracking-wider mb-1">Battery</div>
        <div class="flex items-end gap-1">
          <span class="text-2xl font-bold text-white">{{ powerStats.level_percent }}</span>
          <span class="text-xs text-white/50 mb-1">%</span>
        </div>
        <div class="mt-2 space-y-1">
          <div class="flex justify-between text-xs">
            <span class="text-white/50">Voltage</span>
            <span class="text-white font-mono">{{ powerStats.voltage_v.toFixed(2) }}V</span>
          </div>
          <div class="flex justify-between text-xs">
            <span class="text-white/50">Current</span>
            <span class="text-white font-mono">{{ powerStats.current_ma.toFixed(0) }}mA</span>
          </div>
          <div class="flex justify-between text-xs">
            <span class="text-white/50">Temp</span>
            <span class="font-mono" :class="getTempColor(powerStats.temperature_c)">{{ powerStats.temperature_c.toFixed(1) }}°C</span>
          </div>
        </div>
      </div>

      <!-- Thermal -->
      <div class="bg-white/5 rounded-xl p-3 border border-white/5">
        <div class="text-[10px] text-white/50 uppercase tracking-wider mb-1">Thermal</div>
        <div class="flex items-end gap-1">
          <span class="text-2xl font-bold" :class="getTempColor(thermalStats.cpu_temp)">{{ thermalStats.cpu_temp.toFixed(1) }}</span>
          <span class="text-xs text-white/50 mb-1">°C</span>
        </div>
        <div class="mt-2 space-y-1">
          <div class="flex justify-between text-xs">
            <span class="text-white/50">CPU</span>
            <span class="font-mono" :class="getTempColor(thermalStats.cpu_temp)">{{ thermalStats.cpu_temp.toFixed(1) }}°C</span>
          </div>
          <div class="flex justify-between text-xs">
            <span class="text-white/50">Skin</span>
            <span class="font-mono" :class="getTempColor(thermalStats.skin_temp)">{{ thermalStats.skin_temp.toFixed(1) }}°C</span>
          </div>
          <div class="flex justify-between text-xs">
            <span class="text-white/50">Status</span>
            <span class="font-mono" :class="thermalStats.throttling ? 'text-error' : 'text-success'">
              {{ thermalStats.throttling ? 'Throttling' : 'Normal' }}
            </span>
          </div>
        </div>
      </div>
    </div>
  </GlassCard>
</template>
