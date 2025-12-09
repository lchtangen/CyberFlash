<script setup lang="ts">
import { ref, computed } from 'vue';
import SidebarItem from '../components/ui/SidebarItem.vue';

// Components
import PowerMonitor from '../components/features/hardware/PowerMonitor.vue';
import CableTester from '../components/features/hardware/CableTester.vue';
import StorageHealth from '../components/features/hardware/StorageHealth.vue';
import GlassCard from '../components/ui/GlassCard.vue';

const activeCategory = ref('power');

const categories = [
  { id: 'power', label: 'Power Monitor', icon: 'battery_charging_full' },
  { id: 'cable', label: 'Cable Tester', icon: 'usb' },
  { id: 'storage', label: 'Storage Health', icon: 'hard_drive' },
  { id: 'thermal', label: 'Thermal Vision', icon: 'thermostat' },
];

const activeCategoryLabel = computed(() => categories.find(c => c.id === activeCategory.value)?.label);
</script>

<template>
  <div class="flex h-full gap-6 overflow-hidden">
    <!-- Sidebar Navigation -->
    <GlassCard noPadding class="w-64 flex-shrink-0 flex flex-col overflow-hidden ring-1 ring-white/5 shadow-2xl shadow-black/20">
      <div class="p-5 border-b border-white/10 bg-surface/30 backdrop-blur-xl">
        <h2 class="text-lg font-bold text-white tracking-tight flex items-center gap-2">
          <span class="material-symbols-rounded text-success drop-shadow-[0_0_8px_rgba(48,209,88,0.5)]">memory</span>
          Hardware
        </h2>
        <p class="text-xs text-text-secondary mt-1 font-medium tracking-wide">Diagnostics & Health</p>
      </div>
      
      <div class="flex-1 overflow-y-auto p-3 space-y-1 custom-scrollbar">
        <SidebarItem 
          v-for="cat in categories" 
          :key="cat.id"
          :icon="cat.icon"
          :label="cat.label"
          :active="activeCategory === cat.id"
          @click="activeCategory = cat.id"
        />
      </div>
    </GlassCard>

    <!-- Main Content Area -->
    <GlassCard noPadding class="flex-1 flex flex-col min-w-0 overflow-hidden relative ring-1 ring-white/5 shadow-2xl shadow-black/20">
      <!-- Header -->
      <div class="p-6 border-b border-white/10 flex justify-between items-center bg-surface/30 backdrop-blur-xl z-10">
        <div>
          <h2 class="text-xl font-bold text-white tracking-tight flex items-center gap-2">
            {{ activeCategoryLabel }}
            <span class="px-2 py-0.5 rounded-full bg-success/20 text-success text-[10px] font-bold border border-success/20 uppercase tracking-wider shadow-[0_0_10px_rgba(48,209,88,0.3)]">Real-time</span>
          </h2>
          <p class="text-sm text-text-secondary mt-1 font-medium">Real-time hardware analysis</p>
        </div>
      </div>

      <!-- Scrollable Content -->
      <div class="flex-1 overflow-y-auto p-8 custom-scrollbar relative">
        <!-- Mesh Background -->
        <div class="absolute inset-0 mesh-gradient-bg opacity-20 pointer-events-none"></div>
        
        <div class="relative z-10">
          <!-- Power Tab -->
        <div v-if="activeCategory === 'power'" class="space-y-8 animate-slide-up">
          <div class="hover-tilt-subtle">
            <PowerMonitor />
          </div>
        </div>

        <!-- Cable Tab -->
        <div v-if="activeCategory === 'cable'" class="space-y-8 animate-slide-up">
          <div class="hover-tilt-subtle">
            <CableTester />
          </div>
        </div>

        <!-- Storage Tab -->
        <div v-if="activeCategory === 'storage'" class="space-y-8 animate-slide-up">
          <div class="hover-tilt-subtle">
            <StorageHealth />
          </div>
        </div>

        <!-- Thermal Tab -->
        <div v-if="activeCategory === 'thermal'" class="space-y-8 animate-slide-up">
          <GlassCard class="h-64 flex flex-col items-center justify-center text-center p-8 hover-tilt bg-surface/30 backdrop-blur-xl border-white/10">
            <div class="w-16 h-16 rounded-full bg-white/5 flex items-center justify-center mb-4 shadow-lg shadow-black/20 border border-white/5">
              <span class="material-symbols-rounded text-4xl text-white/20">thermostat</span>
            </div>
            <h3 class="text-lg font-bold text-white mb-2">Thermal Vision</h3>
            <p class="text-white/50 max-w-md leading-relaxed">
              Real-time thermal graph and heatmap visualization is coming in v2.1. This feature will allow you to monitor CPU and battery temperatures during intensive flash operations.
            </p>
          </GlassCard>
        </div>

        </div>
      </div>
    </GlassCard>
  </div>
</template>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 10px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.2);
}

@keyframes slide-up {
  from { opacity: 0; transform: translateY(20px); }
  to { opacity: 1; transform: translateY(0); }
}

.animate-slide-up {
  animation: slide-up 0.5s cubic-bezier(0.16, 1, 0.3, 1) forwards;
}

.hover-tilt-subtle {
  transition: transform 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275), box-shadow 0.4s ease;
}

.hover-tilt-subtle:hover {
  transform: translateY(-4px) scale(1.005);
  box-shadow: 0 20px 40px -10px rgba(0, 0, 0, 0.4);
  z-index: 10;
}

.hover-tilt {
  transition: transform 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275), box-shadow 0.4s ease;
}

.hover-tilt:hover {
  transform: translateY(-8px) scale(1.02) rotateX(2deg);
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
  z-index: 10;
}
</style>
