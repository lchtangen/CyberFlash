<script setup lang="ts">
import { ref, computed } from 'vue';
import { useDeviceStore } from '../../stores/device';

const props = defineProps<{
  currentView: string;
}>();

const emit = defineEmits<{
  (e: 'navigate', view: string): void;
  (e: 'action', action: string): void;
}>();

const deviceStore = useDeviceStore();
const isHovered = ref(false);
const hoveredIndex = ref<number | null>(null);

const menuItems = [
  { id: 'dashboard', icon: 'dashboard', label: 'Home', color: 'text-blue-400' },
  { id: 'flash', icon: 'flash_on', label: 'Flash', color: 'text-yellow-400' },
  { id: 'terminal', icon: 'terminal', label: 'Terminal', color: 'text-green-400' },
  { id: 'ai', icon: 'smart_toy', label: 'AI Assistant', color: 'text-purple-400' },
  { id: 'tools', icon: 'construction', label: 'Tools', color: 'text-orange-400' },
  { id: 'settings', icon: 'settings', label: 'Settings', color: 'text-gray-400' },
];

// Smart Context Actions based on device state
const smartActions = computed(() => {
  const actions = [];
  
  if (deviceStore.isConnected) {
    actions.push({ 
      id: 'reboot', 
      icon: 'restart_alt', 
      label: 'Reboot', 
      action: 'reboot_device',
      color: 'text-cyan-400'
    });
  }
  
  if (props.currentView === 'flash') {
    actions.push({
      id: 'logs',
      icon: 'description',
      label: 'View Logs',
      action: 'toggle_logs',
      color: 'text-white'
    });
  }

  return actions;
});

const handleItemClick = (item: any) => {
  if (item.action) {
    emit('action', item.action);
  } else {
    emit('navigate', item.id);
  }
};

const getScale = (index: number) => {
  if (hoveredIndex.value === null) return 1;
  const distance = Math.abs(hoveredIndex.value - index);
  if (distance === 0) return 1.5;
  if (distance === 1) return 1.25;
  return 1;
};
</script>

<template>
  <div 
    class="fixed bottom-8 left-1/2 -translate-x-1/2 z-50 flex flex-col items-center gap-4 pointer-events-none"
  >
    <!-- Smart Suggestions (Floating Bubbles) -->
    <div class="flex gap-2 pointer-events-auto">
      <transition-group name="pop">
        <button
          v-for="action in smartActions"
          :key="action.id"
          @click="handleItemClick(action)"
          class="bg-surface/80 backdrop-blur-xl border border-white/10 rounded-full px-4 py-2 flex items-center gap-2 shadow-lg hover:bg-white/10 transition-all active:scale-95 group"
        >
          <span class="material-symbols-rounded text-lg" :class="action.color">{{ action.icon }}</span>
          <span class="text-xs font-medium text-white">{{ action.label }}</span>
        </button>
      </transition-group>
    </div>

    <!-- Main Dock -->
    <div 
      class="bg-surface/40 backdrop-blur-2xl border border-white/10 rounded-2xl p-2 flex items-end gap-2 shadow-2xl shadow-black/50 pointer-events-auto transition-all duration-300 hover:bg-surface/60 hover:border-white/20 hover:scale-105"
      @mouseleave="hoveredIndex = null"
    >
      <button
        v-for="(item, index) in menuItems"
        :key="item.id"
        @click="handleItemClick(item)"
        @mouseenter="hoveredIndex = index"
        class="relative group flex flex-col items-center justify-center transition-all duration-200 ease-out"
        :class="[
          currentView === item.id ? 'bg-white/10' : 'hover:bg-white/5',
          'w-12 h-12 rounded-xl'
        ]"
        :style="{ transform: `scale(${getScale(index)}) translateY(${hoveredIndex === index ? '-10px' : '0'})` }"
      >
        <span 
          class="material-symbols-rounded text-2xl transition-colors duration-300"
          :class="[
            currentView === item.id ? item.color : 'text-white/70 group-hover:text-white'
          ]"
        >
          {{ item.icon }}
        </span>
        
        <!-- Active Indicator -->
        <div 
          v-if="currentView === item.id"
          class="absolute -bottom-1 w-1 h-1 rounded-full bg-white shadow-[0_0_8px_rgba(255,255,255,0.8)]"
        ></div>

        <!-- Tooltip -->
        <div 
          class="absolute -top-10 bg-black/80 backdrop-blur-md text-white text-[10px] font-medium px-2 py-1 rounded-lg opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none whitespace-nowrap border border-white/10"
          :style="{ transform: 'scale(0.8)' }"
        >
          {{ item.label }}
        </div>
      </button>
    </div>
  </div>
</template>

<style scoped>
.pop-enter-active,
.pop-leave-active {
  transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.pop-enter-from,
.pop-leave-to {
  opacity: 0;
  transform: translateY(10px) scale(0.8);
}
</style>