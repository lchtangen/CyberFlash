<script setup lang="ts">
import { ref, computed } from 'vue';
import SidebarItem from '../components/ui/SidebarItem.vue';
import DevicePanel from '../components/features/core/DevicePanel.vue';
import AIChatInterface from '../components/features/ai/AIChatInterface.vue';
import Terminal from '../components/features/system/Terminal.vue';
import GlassCard from '../components/ui/GlassCard.vue';
import VisionButton from '../components/ui/VisionButton.vue';
import { useDeviceStore } from '../stores/device';

const emit = defineEmits<{
  (e: 'navigate', view: string): void
}>();

const deviceStore = useDeviceStore();
const activeCategory = ref('overview');

const categories = [
  { id: 'overview', label: 'Overview', icon: 'dashboard' },
  { id: 'terminal', label: 'Live Terminal', icon: 'terminal' },
  { id: 'ai', label: 'AI Assistant', icon: 'smart_toy' },
];

const activeCategoryLabel = computed(() => categories.find(c => c.id === activeCategory.value)?.label);

const features = [
  { id: 'flash', title: 'Flash Firmware', icon: 'flash_on', desc: 'Install ROMs, Recovery, and Kernels', color: 'text-primary' },
  { id: 'tools', title: 'System Tools', icon: 'construction', desc: 'ADB/Fastboot utilities & scripts', color: 'text-warning' },
  { id: 'hardware', title: 'Hardware', icon: 'memory', desc: 'Diagnostics & Health Check', color: 'text-success' },
  { id: 'social', title: 'Community', icon: 'groups', desc: 'Browse ROMs & Share Configs', color: 'text-secondary' },
];

const refreshSystem = async () => {
  await deviceStore.scanDevices();
};
</script>

<template>
  <div class="flex h-full gap-6 overflow-hidden p-6">
    <!-- Sidebar Navigation -->
    <div class="w-64 flex-shrink-0 bg-surface/30 border border-white/10 rounded-2xl backdrop-blur-xl flex flex-col overflow-hidden">
      <div class="p-4 border-b border-white/10">
        <h2 class="text-lg font-bold text-white tracking-tight">Mission Control</h2>
        <p class="text-xs text-text-secondary">System Overview</p>
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
      
      <div class="p-4 border-t border-white/10">
         <VisionButton variant="secondary" icon="refresh" size="sm" class="w-full justify-center" @click="refreshSystem">
          Refresh System
        </VisionButton>
      </div>
    </div>

    <!-- Main Content Area -->
    <div class="flex-1 flex flex-col min-w-0 bg-surface/30 border border-white/10 rounded-2xl backdrop-blur-xl overflow-hidden">
      <!-- Header -->
      <div class="p-6 border-b border-white/10 flex justify-between items-center bg-white/5">
        <div>
          <h2 class="text-xl font-bold text-white tracking-tight">{{ activeCategoryLabel }}</h2>
          <p class="text-sm text-text-secondary">
            {{ activeCategory === 'overview' ? 'System Status & Quick Actions' : 'Advanced Operations' }}
          </p>
        </div>
      </div>

      <!-- Scrollable Content -->
      <div class="flex-1 overflow-y-auto p-6 custom-scrollbar">
        
        <!-- Overview Tab -->
        <div v-if="activeCategory === 'overview'" class="space-y-6">
           <!-- Device Status (Hero) -->
            <DevicePanel />

            <!-- Feature Grid -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <GlassCard 
                v-for="feat in features" 
                :key="feat.id"
                class="group cursor-pointer hover:bg-white/5 transition-all duration-300 border-white/5 hover:border-primary/30 active:scale-95"
                @click="emit('navigate', feat.id)"
            >
                <div class="flex flex-col h-full justify-between p-2">
                <div class="mb-4">
                    <div class="w-10 h-10 rounded-lg bg-white/5 flex items-center justify-center mb-3 group-hover:scale-110 transition-transform duration-300">
                    <span class="material-symbols-rounded text-2xl" :class="feat.color">{{ feat.icon }}</span>
                    </div>
                    <h3 class="text-lg font-bold text-white group-hover:text-primary transition-colors">{{ feat.title }}</h3>
                    <p class="text-sm text-white/50 mt-1">{{ feat.desc }}</p>
                </div>
                <div class="flex items-center text-xs font-medium text-white/30 group-hover:text-white/70 transition-colors">
                    <span>Launch</span>
                    <span class="material-symbols-rounded text-sm ml-1 group-hover:translate-x-1 transition-transform">arrow_forward</span>
                </div>
                </div>
            </GlassCard>
            </div>
        </div>

        <!-- Terminal Tab -->
        <div v-if="activeCategory === 'terminal'" class="h-full">
          <Terminal class="h-full" />
        </div>

        <!-- AI Tab -->
        <div v-if="activeCategory === 'ai'" class="h-full">
          <AIChatInterface heightClass="h-full" />
        </div>

      </div>
    </div>
  </div>
</template>
