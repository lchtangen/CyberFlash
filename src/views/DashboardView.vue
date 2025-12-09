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
    <GlassCard noPadding class="w-64 flex-shrink-0 flex flex-col overflow-hidden">
      <div class="p-5 border-b border-white/10 bg-white/5 backdrop-blur-md">
        <h2 class="text-lg font-bold text-white tracking-tight flex items-center gap-2">
          <span class="material-symbols-rounded text-primary">dashboard</span>
          Mission Control
        </h2>
        <p class="text-xs text-text-secondary mt-1">System Overview</p>
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
      
      <div class="p-4 border-t border-white/10 bg-white/5">
         <VisionButton variant="secondary" icon="refresh" size="sm" class="w-full justify-center" @click="refreshSystem">
          Refresh System
        </VisionButton>
      </div>
    </GlassCard>

    <!-- Main Content Area -->
    <GlassCard noPadding class="flex-1 flex flex-col min-w-0 overflow-hidden relative">
      <!-- Header -->
      <div class="p-6 border-b border-white/10 flex justify-between items-center bg-white/5 backdrop-blur-md z-10">
        <div>
          <h2 class="text-xl font-bold text-white tracking-tight flex items-center gap-2">
            {{ activeCategoryLabel }}
            <span v-if="activeCategory === 'overview'" class="px-2 py-0.5 rounded-full bg-primary/20 text-primary text-[10px] font-bold border border-primary/20 uppercase tracking-wider">Live</span>
          </h2>
          <p class="text-sm text-text-secondary mt-1">
            {{ activeCategory === 'overview' ? 'System Status & Quick Actions' : 'Advanced Operations' }}
          </p>
        </div>
      </div>

      <!-- Scrollable Content -->
      <div class="flex-1 overflow-y-auto p-8 custom-scrollbar bg-gradient-to-b from-transparent to-black/20">
        
        <!-- Overview Tab -->
        <div v-if="activeCategory === 'overview'" class="space-y-8 animate-slide-up">
           <!-- Device Status (Hero) -->
            <div class="hover-tilt-subtle">
              <DevicePanel />
            </div>

            <!-- Feature Grid -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <GlassCard 
                v-for="feat in features" 
                :key="feat.id"
                class="group cursor-pointer hover:bg-white/10 transition-all duration-500 border-white/5 hover:border-primary/50 hover-tilt"
                @click="emit('navigate', feat.id)"
            >
                <div class="flex flex-col h-full justify-between p-4 relative overflow-hidden">
                  <!-- Glow Effect -->
                  <div class="absolute -right-10 -top-10 w-32 h-32 bg-primary/20 blur-[50px] rounded-full opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>

                  <div class="mb-4 relative z-10">
                      <div class="w-12 h-12 rounded-xl bg-white/5 flex items-center justify-center mb-4 group-hover:scale-110 group-hover:bg-primary/20 transition-all duration-300 shadow-lg shadow-black/20">
                        <span class="material-symbols-rounded text-3xl" :class="feat.color">{{ feat.icon }}</span>
                      </div>
                      <h3 class="text-xl font-bold text-white group-hover:text-primary transition-colors tracking-tight">{{ feat.title }}</h3>
                      <p class="text-sm text-white/60 mt-2 leading-relaxed">{{ feat.desc }}</p>
                  </div>
                  <div class="flex items-center text-xs font-bold text-white/30 group-hover:text-primary transition-colors uppercase tracking-wider relative z-10">
                      <span>Launch Module</span>
                      <span class="material-symbols-rounded text-sm ml-2 group-hover:translate-x-1 transition-transform">arrow_forward</span>
                  </div>
                </div>
            </GlassCard>
            </div>
        </div>

        <!-- Terminal Tab -->
        <div v-if="activeCategory === 'terminal'" class="h-full animate-slide-up">
          <Terminal class="h-full shadow-2xl shadow-black/50 rounded-xl overflow-hidden border border-white/10" />
        </div>

        <!-- AI Tab -->
        <div v-if="activeCategory === 'ai'" class="h-full animate-slide-up">
          <AIChatInterface heightClass="h-full" />
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

.hover-tilt {
  transition: transform 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275), box-shadow 0.4s ease, border-color 0.4s ease, background-color 0.4s ease;
}

.hover-tilt:hover {
  transform: translateY(-8px) scale(1.02) rotateX(2deg);
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5), 0 0 20px rgba(var(--primary-rgb), 0.2);
  z-index: 10;
}

.hover-tilt-subtle {
  transition: transform 0.4s ease;
}
.hover-tilt-subtle:hover {
  transform: translateY(-2px);
}
</style>
