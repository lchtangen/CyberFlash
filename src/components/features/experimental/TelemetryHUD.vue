<script setup lang="ts">
import { ref, onMounted } from 'vue';

const cpu = ref(0);
const ram = ref(0);
const temp = ref(0);

onMounted(() => {
  setInterval(() => {
    cpu.value = Math.floor(Math.random() * 30) + 10;
    ram.value = Math.floor(Math.random() * 20) + 40;
    temp.value = Math.floor(Math.random() * 10) + 35;
  }, 2000);
});

const getCircleDash = (percent: number, r: number) => {
  const c = 2 * Math.PI * r;
  return `${(percent / 100) * c} ${c}`;
};
</script>

<template>
  <div class="grid grid-cols-3 gap-4 p-4 bg-black/40 rounded-xl border border-white/10 backdrop-blur-md">
    
    <!-- CPU -->
    <div class="flex flex-col items-center">
      <div class="relative w-20 h-20">
        <svg class="w-full h-full -rotate-90">
          <circle cx="40" cy="40" r="36" class="stroke-white/10 fill-none stroke-[4]" />
          <circle 
            cx="40" cy="40" r="36" 
            class="stroke-primary fill-none stroke-[4] transition-all duration-1000 ease-out"
            :stroke-dasharray="getCircleDash(cpu, 36)"
          />
        </svg>
        <div class="absolute inset-0 flex items-center justify-center flex-col">
          <span class="text-lg font-bold text-white">{{ cpu }}%</span>
          <span class="text-[10px] text-white/50">CPU</span>
        </div>
      </div>
    </div>

    <!-- RAM -->
    <div class="flex flex-col items-center">
      <div class="relative w-20 h-20">
        <svg class="w-full h-full -rotate-90">
          <circle cx="40" cy="40" r="36" class="stroke-white/10 fill-none stroke-[4]" />
          <circle 
            cx="40" cy="40" r="36" 
            class="stroke-purple-500 fill-none stroke-[4] transition-all duration-1000 ease-out"
            :stroke-dasharray="getCircleDash(ram, 36)"
          />
        </svg>
        <div class="absolute inset-0 flex items-center justify-center flex-col">
          <span class="text-lg font-bold text-white">{{ ram }}%</span>
          <span class="text-[10px] text-white/50">RAM</span>
        </div>
      </div>
    </div>

    <!-- TEMP -->
    <div class="flex flex-col items-center">
      <div class="relative w-20 h-20">
        <svg class="w-full h-full -rotate-90">
          <circle cx="40" cy="40" r="36" class="stroke-white/10 fill-none stroke-[4]" />
          <circle 
            cx="40" cy="40" r="36" 
            class="stroke-orange-500 fill-none stroke-[4] transition-all duration-1000 ease-out"
            :stroke-dasharray="getCircleDash(temp, 36)"
          />
        </svg>
        <div class="absolute inset-0 flex items-center justify-center flex-col">
          <span class="text-lg font-bold text-white">{{ temp }}°</span>
          <span class="text-[10px] text-white/50">TEMP</span>
        </div>
      </div>
    </div>

  </div>
</template>
