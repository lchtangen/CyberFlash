<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useDeviceStore } from '../../../stores/device';
import GlassCard from '../../ui/GlassCard.vue';

const deviceStore = useDeviceStore();
const cpu = ref(0);
const ram = ref(0);
const temp = ref(0);
const network = ref(0);
let interval: any = null;

const fetchTelemetry = async () => {
  if (!deviceStore.isConnected) {
    cpu.value = 0;
    ram.value = 0;
    temp.value = 0;
    network.value = 0;
    return;
  }

  try {
    const data = await invoke<{ cpu: number, ram: number, temp: number, network: number }>('get_telemetry');
    cpu.value = data.cpu;
    ram.value = data.ram;
    temp.value = data.temp;
    network.value = data.network;
  } catch (e) {
    console.error('Telemetry error:', e);
  }
};

onMounted(() => {
  fetchTelemetry();
  interval = setInterval(fetchTelemetry, 3000);
});

onUnmounted(() => {
  if (interval) clearInterval(interval);
});

const getCircleDash = (percent: number, r: number) => {
  const c = 2 * Math.PI * r;
  return `${(percent / 100) * c} ${c}`;
};
</script>

<template>
  <GlassCard class="h-full flex flex-col relative overflow-hidden group">
    <!-- Header -->
    <div class="flex items-center justify-between mb-5 relative z-10">
      <div class="flex items-center gap-2">
        <div class="p-1.5 rounded-lg bg-white/5 border border-white/10">
          <span class="material-symbols-rounded text-base text-primary">monitoring</span>
        </div>
        <h3 class="text-sm font-bold text-white tracking-tight">Telemetry</h3>
      </div>
      <div class="flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-white/5 border border-white/5">
         <span class="w-1 h-1 rounded-full bg-success animate-pulse"></span>
         <span class="text-[9px] font-mono text-success font-bold">LIVE</span>
      </div>
    </div>

    <!-- Metrics Grid -->
    <div class="flex-1 grid grid-cols-1 gap-3 relative z-10">
      
      <!-- CPU -->
      <div class="flex items-center gap-3 p-2.5 rounded-xl bg-white/5 border border-white/5 hover:bg-white/10 transition-colors group/item">
        <div class="relative w-10 h-10 shrink-0">
          <svg class="w-full h-full -rotate-90">
            <circle cx="20" cy="20" r="16" class="stroke-white/10 fill-none stroke-[3]" />
            <circle 
              cx="20" cy="20" r="16" 
              class="stroke-primary fill-none stroke-[3] transition-all duration-1000 ease-out"
              :stroke-dasharray="getCircleDash(cpu, 16)"
              stroke-linecap="round"
            />
          </svg>
          <div class="absolute inset-0 flex items-center justify-center">
            <span class="text-[9px] font-bold text-white">{{ cpu }}%</span>
          </div>
        </div>
        <div class="flex-1 min-w-0">
          <div class="flex justify-between items-center mb-0.5">
            <span class="text-xs font-bold text-white">CPU Load</span>
            <span class="text-[9px] font-mono text-primary opacity-0 group-hover/item:opacity-100 transition-opacity">SD855</span>
          </div>
          <div class="w-full h-1 bg-white/10 rounded-full overflow-hidden">
            <div class="h-full bg-primary transition-all duration-1000 ease-out" :style="{ width: `${cpu}%` }"></div>
          </div>
        </div>
      </div>

      <!-- RAM -->
      <div class="flex items-center gap-3 p-2.5 rounded-xl bg-white/5 border border-white/5 hover:bg-white/10 transition-colors group/item">
        <div class="relative w-10 h-10 shrink-0">
          <svg class="w-full h-full -rotate-90">
            <circle cx="20" cy="20" r="16" class="stroke-white/10 fill-none stroke-[3]" />
            <circle 
              cx="20" cy="20" r="16" 
              class="stroke-purple-500 fill-none stroke-[3] transition-all duration-1000 ease-out"
              :stroke-dasharray="getCircleDash(ram, 16)"
              stroke-linecap="round"
            />
          </svg>
          <div class="absolute inset-0 flex items-center justify-center">
            <span class="text-[9px] font-bold text-white">{{ ram }}%</span>
          </div>
        </div>
        <div class="flex-1 min-w-0">
          <div class="flex justify-between items-center mb-0.5">
            <span class="text-xs font-bold text-white">Memory</span>
            <span class="text-[9px] font-mono text-purple-400 opacity-0 group-hover/item:opacity-100 transition-opacity">8GB</span>
          </div>
          <div class="w-full h-1 bg-white/10 rounded-full overflow-hidden">
            <div class="h-full bg-purple-500 transition-all duration-1000 ease-out" :style="{ width: `${ram}%` }"></div>
          </div>
        </div>
      </div>

      <!-- Temp -->
      <div class="flex items-center gap-3 p-2.5 rounded-xl bg-white/5 border border-white/5 hover:bg-white/10 transition-colors group/item">
        <div class="relative w-10 h-10 shrink-0 flex items-center justify-center bg-white/5 rounded-full">
           <span class="material-symbols-rounded text-lg" :class="temp > 40 ? 'text-error' : 'text-success'">thermostat</span>
        </div>
        <div class="flex-1 min-w-0">
          <div class="flex justify-between items-center mb-0.5">
            <span class="text-xs font-bold text-white">Battery Temp</span>
            <span class="text-[10px] font-mono font-bold" :class="temp > 40 ? 'text-error' : 'text-success'">{{ temp }}°C</span>
          </div>
          <div class="w-full h-1 bg-white/10 rounded-full overflow-hidden">
            <div class="h-full transition-all duration-1000 ease-out" 
                 :class="temp > 40 ? 'bg-error' : 'bg-success'"
                 :style="{ width: `${(temp / 60) * 100}%` }"></div>
          </div>
        </div>
      </div>

    </div>
    
    <!-- Background Graph Decoration -->
    <div class="absolute bottom-0 left-0 right-0 h-24 bg-gradient-to-t from-primary/5 to-transparent pointer-events-none"></div>
    <div class="absolute -bottom-6 -right-6 w-32 h-32 bg-primary/10 rounded-full blur-3xl pointer-events-none"></div>
  </GlassCard>
</template>
