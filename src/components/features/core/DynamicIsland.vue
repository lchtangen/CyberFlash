<script setup lang="ts">
import { computed } from 'vue';
import { useDynamicIslandStore } from '../../../stores/dynamicIsland';

const store = useDynamicIslandStore();
const activity = computed(() => store.activeActivity);

const isExpanded = computed(() => !!activity.value);

const getIconColor = (type: string) => {
  switch (type) {
    case 'success': return 'text-success';
    case 'error': return 'text-error';
    case 'warning': return 'text-warning';
    case 'process': return 'text-primary';
    case 'ai': return 'text-purple-400';
    default: return 'text-white';
  }
};

const getBgGlow = (type: string) => {
  switch (type) {
    case 'success': return 'shadow-[0_0_30px_rgba(48,209,88,0.3)]';
    case 'error': return 'shadow-[0_0_30px_rgba(255,69,58,0.3)]';
    case 'warning': return 'shadow-[0_0_30px_rgba(255,214,10,0.3)]';
    case 'process': return 'shadow-[0_0_30px_rgba(10,132,255,0.3)]';
    case 'ai': return 'shadow-[0_0_30px_rgba(192,132,252,0.3)]';
    default: return 'shadow-none';
  }
};
</script>

<template>
  <div 
    class="fixed top-6 left-1/2 -translate-x-1/2 z-[100] transition-all duration-500 ease-[cubic-bezier(0.175,0.885,0.32,1.275)]"
    :class="[
      isExpanded ? 'w-[400px] h-auto' : 'w-[120px] h-[36px]',
    ]"
  >
    <div 
      class="relative w-full h-full bg-black/80 backdrop-blur-2xl rounded-[24px] border border-white/10 overflow-hidden flex items-center justify-center transition-all duration-500"
      :class="[
        isExpanded ? 'p-4' : 'p-1',
        activity ? getBgGlow(activity.type) : ''
      ]"
    >
      <!-- Idle State -->
      <div 
        v-if="!isExpanded" 
        class="w-full h-full flex items-center justify-center gap-2 opacity-50"
      >
        <div class="w-1 h-1 rounded-full bg-white/50"></div>
        <div class="w-16 h-1 rounded-full bg-white/20"></div>
        <div class="w-1 h-1 rounded-full bg-white/50"></div>
      </div>

      <!-- Active State -->
      <div v-else class="w-full flex items-center gap-4">
        <!-- Icon / Spinner -->
        <div class="relative flex-shrink-0 w-10 h-10 flex items-center justify-center">
          <div v-if="activity?.type === 'process' || activity?.type === 'download'" class="absolute inset-0 border-2 border-white/10 border-t-primary rounded-full animate-spin"></div>
          <span 
            class="material-symbols-rounded text-2xl transition-colors duration-300"
            :class="getIconColor(activity?.type || 'info')"
          >
            {{ activity?.icon }}
          </span>
        </div>

        <!-- Content -->
        <div class="flex-1 min-w-0">
          <h3 class="text-sm font-bold text-white truncate">{{ activity?.title }}</h3>
          <p v-if="activity?.subtitle" class="text-xs text-white/60 truncate">{{ activity?.subtitle }}</p>
          
          <!-- Progress Bar -->
          <div v-if="activity?.progress !== undefined" class="mt-2 h-1.5 bg-white/10 rounded-full overflow-hidden">
            <div 
              class="h-full bg-primary transition-all duration-300 ease-out"
              :style="{ width: `${activity.progress}%` }"
              :class="{ 
                'bg-success': activity.type === 'success',
                'bg-error': activity.type === 'error',
                'bg-warning': activity.type === 'warning',
                'bg-purple-500': activity.type === 'ai'
              }"
            ></div>
          </div>
        </div>

        <!-- Action / Status -->
        <div v-if="activity?.progress !== undefined" class="text-xs font-mono font-bold text-white/40">
          {{ Math.round(activity.progress) }}%
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.ease-spring {
  transition-timing-function: cubic-bezier(0.175, 0.885, 0.32, 1.275);
}
</style>
