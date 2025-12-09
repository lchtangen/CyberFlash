<script setup lang="ts">
import { computed } from 'vue';
import { useDeviceStore } from '../../../stores/device';
import { useFlashStore } from '../../../stores/flash';

const deviceStore = useDeviceStore();
const flashStore = useFlashStore();

const status = computed(() => {
  if (flashStore.isFlashing) {
    return {
      type: 'process',
      icon: 'sync',
      title: 'Flashing Firmware',
      subtitle: `Phase ${flashStore.currentPhase + 1} - ${Math.round(flashStore.progress)}%`,
      color: 'text-primary',
      bg: 'bg-primary/20',
      border: 'border-primary/30'
    };
  }
  
  if (deviceStore.isConnected) {
    return {
      type: 'success',
      icon: 'smartphone',
      title: deviceStore.deviceModel || 'Device Connected',
      subtitle: 'Ready for operations',
      color: 'text-success',
      bg: 'bg-success/20',
      border: 'border-success/30'
    };
  }

  return {
    type: 'idle',
    icon: 'usb_off',
    title: 'No Device',
    subtitle: 'Connect via USB',
    color: 'text-white/50',
    bg: 'bg-white/5',
    border: 'border-white/10'
  };
});
</script>

<template>
  <div class="flex justify-center w-full py-2">
    <div 
      class="relative flex items-center gap-4 px-4 py-2 rounded-full border backdrop-blur-xl transition-all duration-500 shadow-lg"
      :class="[status.bg, status.border, flashStore.isFlashing ? 'w-96' : 'w-auto hover:scale-105']"
    >
      <!-- Icon Circle -->
      <div 
        class="w-8 h-8 rounded-full flex items-center justify-center transition-colors duration-300"
        :class="[status.type === 'process' ? 'bg-primary text-white animate-spin' : 'bg-black/20 ' + status.color]"
      >
        <span class="material-symbols-rounded text-lg">{{ status.icon }}</span>
      </div>

      <!-- Text Content -->
      <div class="flex flex-col min-w-[120px]">
        <span class="text-xs font-bold text-white tracking-wide">{{ status.title }}</span>
        <span class="text-[10px] font-medium opacity-70 truncate max-w-[200px]" :class="status.color">{{ status.subtitle }}</span>
      </div>

      <!-- Progress Bar (Only when flashing) -->
      <div v-if="status.type === 'process'" class="absolute bottom-0 left-0 right-0 h-0.5 bg-black/20 overflow-hidden rounded-b-full">
        <div 
          class="h-full bg-primary transition-all duration-300"
          :style="{ width: `${flashStore.progress}%` }"
        ></div>
      </div>
      
      <!-- Glow Effect -->
      <div class="absolute inset-0 rounded-full blur-xl opacity-20 pointer-events-none" :class="status.bg"></div>
    </div>
  </div>
</template>
