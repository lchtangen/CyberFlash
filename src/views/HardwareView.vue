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
  <div class="flex h-full gap-6 overflow-hidden p-6">
    <!-- Sidebar Navigation -->
    <div class="w-64 flex-shrink-0 bg-surface/30 border border-white/10 rounded-2xl backdrop-blur-xl flex flex-col overflow-hidden">
      <div class="p-4 border-b border-white/10">
        <h2 class="text-lg font-bold text-white tracking-tight">Hardware</h2>
        <p class="text-xs text-text-secondary">Diagnostics & Health</p>
      </div>
      
      <div class="flex-1 overflow-y-auto p-2 space-y-1 custom-scrollbar">
        <SidebarItem 
          v-for="cat in categories" 
          :key="cat.id"
          :icon="cat.icon"
          :label="cat.label"
          :active="activeCategory === cat.id"
          @click="activeCategory = cat.id"
        />
      </div>
    </div>

    <!-- Main Content Area -->
    <div class="flex-1 flex flex-col min-w-0 bg-surface/30 border border-white/10 rounded-2xl backdrop-blur-xl overflow-hidden">
      <!-- Header -->
      <div class="p-6 border-b border-white/10 flex justify-between items-center bg-white/5">
        <div>
          <h2 class="text-xl font-bold text-white tracking-tight">{{ activeCategoryLabel }}</h2>
          <p class="text-sm text-text-secondary">Real-time hardware analysis</p>
        </div>
      </div>

      <!-- Scrollable Content -->
      <div class="flex-1 overflow-y-auto p-6 custom-scrollbar">
        
        <!-- Power Tab -->
        <div v-if="activeCategory === 'power'" class="space-y-6">
          <PowerMonitor />
        </div>

        <!-- Cable Tab -->
        <div v-if="activeCategory === 'cable'" class="space-y-6">
          <CableTester />
        </div>

        <!-- Storage Tab -->
        <div v-if="activeCategory === 'storage'" class="space-y-6">
          <StorageHealth />
        </div>

        <!-- Thermal Tab -->
        <div v-if="activeCategory === 'thermal'" class="space-y-6">
          <GlassCard class="h-64 flex flex-col items-center justify-center text-center p-8">
            <div class="w-16 h-16 rounded-full bg-white/5 flex items-center justify-center mb-4">
              <span class="material-symbols-rounded text-4xl text-white/20">thermostat</span>
            </div>
            <h3 class="text-lg font-bold text-white mb-2">Thermal Vision</h3>
            <p class="text-white/50 max-w-md">
              Real-time thermal graph and heatmap visualization is coming in v2.1. This feature will allow you to monitor CPU and battery temperatures during intensive flash operations.
            </p>
          </GlassCard>
        </div>

      </div>
    </div>
  </div>
</template>
