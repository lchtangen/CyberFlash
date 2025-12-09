<script setup lang="ts">
const steps = [
  { id: 1, label: 'Check ADB', status: 'completed' },
  { id: 2, label: 'Reboot Bootloader', status: 'completed' },
  { id: 3, label: 'Unlock OEM', status: 'active' },
  { id: 4, label: 'Flash Recovery', status: 'pending' },
  { id: 5, label: 'Wipe Data', status: 'pending' },
];
</script>

<template>
  <div class="w-full overflow-x-auto custom-scrollbar pb-4">
    <div class="flex items-center min-w-max px-4">
      <div v-for="(step, index) in steps" :key="step.id" class="flex items-center">
        
        <!-- Node -->
        <div class="relative flex flex-col items-center group">
          <div 
            class="w-8 h-8 rounded-full flex items-center justify-center border-2 transition-all duration-300 z-10 bg-surface"
            :class="{
              'border-success text-success': step.status === 'completed',
              'border-primary text-primary shadow-[0_0_15px_rgba(10,132,255,0.5)] scale-110': step.status === 'active',
              'border-white/20 text-white/30': step.status === 'pending'
            }"
          >
            <span class="material-symbols-rounded text-sm font-bold">
              {{ step.status === 'completed' ? 'check' : step.id }}
            </span>
          </div>
          
          <div class="absolute top-10 w-32 text-center">
            <div class="text-xs font-bold text-white">{{ step.label }}</div>
            <div class="text-[10px] text-white/50 uppercase">{{ step.status }}</div>
          </div>
        </div>

        <!-- Connector -->
        <div v-if="index < steps.length - 1" class="w-16 h-0.5 mx-2 relative bg-white/10">
          <div 
            class="absolute inset-0 bg-success transition-all duration-500"
            :style="{ width: step.status === 'completed' ? '100%' : '0%' }"
          ></div>
        </div>

      </div>
    </div>
  </div>
</template>
