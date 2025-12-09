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
  <GlassCard class="p-4 flex items-center justify-between gap-4">
    <!-- CPU -->
    <div class="flex items-center gap-3">
      <div class="relative w-12 h-12">
        <svg class="w-full h-full -rotate-90">
          <circle cx="24" cy="24" r="20" class="stroke-white/10 fill-none stroke-[3]" />
          <circle 
            cx="24" cy="24" r="20" 
            class="stroke-primary fill-none stroke-[3] transition-all duration-1000 ease-out"
            :stroke-dasharray="getCircleDash(cpu, 20)"
          />
        </svg>
        <div class="absolute inset-0 flex items-center justify-center">
          <span class="text-[10px] font-bold text-white">{{ cpu }}%</span>
        </div>
      </div>
      <div>
        <div class="text-xs font-bold text-white">CPU Load</div>
        <div class="text-[10px] text-text-secondary">Snapdragon 855</div>
      </div>
    </div>

    <div class="w-px h-8 bg-white/10"></div>

    <!-- RAM -->
    <div class="flex items-center gap-3">
      <div class="relative w-12 h-12">
        <svg class="w-full h-full -rotate-90">
          <circle cx="24" cy="24" r="20" class="stroke-white/10 fill-none stroke-[3]" />
          <circle 
            cx="24" cy="24" r="20" 
            class="stroke-purple-500 fill-none stroke-[3] transition-all duration-1000 ease-out"
            :stroke-dasharray="getCircleDash(ram, 20)"
          />
        </svg>
        <div class="absolute inset-0 flex items-center justify-center">
          <span class="text-[10px] font-bold text-white">{{ ram }}%</span>
        </div>
      </div>
      <div>
        <div class="text-xs font-bold text-white">Memory</div>
        <div class="text-[10px] text-text-secondary">3.4 / 8 GB</div>
      </div>
    </div>

    <div class="w-px h-8 bg-white/10"></div>

    <!-- TEMP -->
    <div class="flex items-center gap-3">
      <div class="relative w-12 h-12">
        <svg class="w-full h-full -rotate-90">
          <circle cx="24" cy="24" r="20" class="stroke-white/10 fill-none stroke-[3]" />
          <circle 
            cx="24" cy="24" r="20" 
            class="stroke-orange-500 fill-none stroke-[3] transition-all duration-1000 ease-out"
            :stroke-dasharray="getCircleDash(temp, 20)"
          />
        </svg>
        <div class="absolute inset-0 flex items-center justify-center">
          <span class="text-[10px] font-bold text-white">{{ temp }}°</span>
        </div>
      </div>
      <div>
        <div class="text-xs font-bold text-white">Thermal</div>
        <div class="text-[10px] text-text-secondary">Battery Temp</div>
      </div>
    </div>
    
    <div class="w-px h-8 bg-white/10"></div>

    <!-- Network -->
    <div class="flex items-center gap-3">
      <div class="relative w-12 h-12 flex items-center justify-center bg-white/5 rounded-full">
        <span class="material-symbols-rounded text-cyan-400 text-xl">network_check</span>
      </div>
      <div>
        <div class="text-xs font-bold text-white">ADB Link</div>
        <div class="text-[10px] text-text-secondary">{{ network }} MB/s</div>
      </div>
    </div>

  </GlassCard>
</template>
