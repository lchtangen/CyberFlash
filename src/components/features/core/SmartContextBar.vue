<script setup lang="ts">
import { computed } from 'vue';
import { useDeviceStore } from '../../../stores/device';
import { useFlashStore } from '../../../stores/flash';
import { useAIStore } from '../../../stores/ai';
import { useDynamicIslandStore } from '../../../stores/dynamicIsland';

const deviceStore = useDeviceStore();
const flashStore = useFlashStore();
const aiStore = useAIStore();
const islandStore = useDynamicIslandStore();

const status = computed(() => {
  // 1. Explicit Dynamic Island Activity (Highest Priority)
  if (islandStore.activeActivity) {
    return {
      ...islandStore.activeActivity,
      bg: islandStore.activeActivity.bg || 'bg-surface/50',
      border: islandStore.activeActivity.border || 'border-white/10',
      color: islandStore.activeActivity.color || 'text-white'
    };
  }

  // 2. Flashing State
  if (flashStore.isFlashing) {
    return {
      type: 'process',
      icon: 'sync',
      title: 'Flashing Firmware',
      subtitle: `Phase ${flashStore.currentPhase + 1} - ${Math.round(flashStore.progress)}%`,
      color: 'text-primary',
      bg: 'bg-primary/20',
      border: 'border-primary/30',
      progress: flashStore.progress
    };
  }

  // 3. AI Thinking State
  if (aiStore.isThinking) {
    return {
      type: 'ai',
      icon: 'smart_toy',
      title: 'CyberFlash AI',
      subtitle: 'Processing request...',
      color: 'text-purple-400',
      bg: 'bg-purple-500/20',
      border: 'border-purple-500/30'
    };
  }
  
  // 4. Device Connected State
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

  // 5. Idle State
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
      class="relative flex items-center gap-4 px-4 py-2 rounded-full border backdrop-blur-xl transition-all duration-500 shadow-lg overflow-hidden"
      :class="[
        status.bg, 
        status.border, 
        (status.type === 'process' || status.type === 'ai' || status.type === 'download') ? 'w-96' : 'w-auto hover:scale-105'
      ]"
    >
      <!-- Icon Circle -->
      <div 
        class="w-8 h-8 rounded-full flex items-center justify-center transition-colors duration-300 shrink-0"
        :class="[
          status.type === 'process' ? 'bg-primary text-white animate-spin' : '',
          status.type === 'ai' ? 'bg-purple-500 text-white animate-pulse' : '',
          (status.type !== 'process' && status.type !== 'ai') ? 'bg-black/20 ' + status.color : ''
        ]"
      >
        <span class="material-symbols-rounded text-lg">{{ status.icon }}</span>
      </div>

      <!-- Text Content -->
      <div class="flex flex-col min-w-[120px] transition-all duration-300">
        <span class="text-xs font-bold text-white tracking-wide">{{ status.title }}</span>
        <span class="text-[10px] font-medium opacity-70 truncate max-w-[200px]" :class="status.color">{{ status.subtitle }}</span>
      </div>

      <!-- Progress Bar (Flashing / Download / Custom) -->
      <div v-if="status.progress !== undefined" class="absolute bottom-0 left-0 right-0 h-0.5 bg-black/20 overflow-hidden rounded-b-full">
        <div 
          class="h-full transition-all duration-300"
          :class="status.type === 'ai' ? 'bg-purple-500' : 'bg-primary'"
          :style="{ width: `${status.progress}%` }"
        ></div>
      </div>
      
      <!-- AI Waveform Animation -->
      <div v-if="status.type === 'ai'" class="absolute right-4 flex gap-0.5 items-center h-4">
        <div class="w-0.5 bg-purple-400 animate-[music-bar_0.5s_ease-in-out_infinite]" style="animation-delay: 0s"></div>
        <div class="w-0.5 bg-purple-400 animate-[music-bar_0.5s_ease-in-out_infinite]" style="animation-delay: 0.1s"></div>
        <div class="w-0.5 bg-purple-400 animate-[music-bar_0.5s_ease-in-out_infinite]" style="animation-delay: 0.2s"></div>
        <div class="w-0.5 bg-purple-400 animate-[music-bar_0.5s_ease-in-out_infinite]" style="animation-delay: 0.3s"></div>
      </div>

      <!-- Glow Effect -->
      <div class="absolute inset-0 rounded-full blur-xl opacity-20 pointer-events-none" :class="status.bg"></div>
    </div>
  </div>
</template>

<style scoped>
@keyframes music-bar {
  0%, 100% { height: 4px; }
  50% { height: 12px; }
}
</style>
