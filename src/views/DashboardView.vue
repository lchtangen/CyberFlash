<script setup lang="ts">
import { ref, computed } from 'vue';
import SidebarItem from '../components/ui/SidebarItem.vue';
import DevicePanel from '../components/features/core/DevicePanel.vue';
import TelemetryWidget from '../components/features/core/TelemetryWidget.vue';
import RecentActivity from '../components/features/core/RecentActivity.vue';
import CommunityFeed from '../components/features/core/CommunityFeed.vue';
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
    <GlassCard noPadding class="w-64 flex-shrink-0 flex flex-col overflow-hidden ring-1 ring-white/5 shadow-2xl shadow-black/20">
      <div class="p-5 border-b border-white/10 bg-surface/30 backdrop-blur-xl">
        <h2 class="text-lg font-bold text-white tracking-tight flex items-center gap-2">
          <span class="material-symbols-rounded text-primary drop-shadow-[0_0_8px_rgba(0,240,255,0.5)]">dashboard</span>
          Mission Control
        </h2>
        <p class="text-xs text-text-secondary mt-1 font-medium tracking-wide">System Overview</p>
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
      
      <div class="p-4 border-t border-white/10 bg-surface/30 backdrop-blur-xl">
         <VisionButton variant="secondary" icon="refresh" size="sm" class="w-full justify-center shadow-lg shadow-black/20" @click="refreshSystem">
          Refresh System
        </VisionButton>
      </div>
    </GlassCard>

    <!-- Main Content Area -->
    <GlassCard noPadding class="flex-1 flex flex-col min-w-0 overflow-hidden relative ring-1 ring-white/5 shadow-2xl shadow-black/20">
      <!-- Header -->
      <div class="p-6 border-b border-white/10 flex justify-between items-center bg-surface/30 backdrop-blur-xl z-10">
        <div>
          <h2 class="text-xl font-bold text-white tracking-tight flex items-center gap-2">
            {{ activeCategoryLabel }}
            <span v-if="activeCategory === 'overview'" class="px-2 py-0.5 rounded-full bg-primary/20 text-primary text-[10px] font-bold border border-primary/20 uppercase tracking-wider shadow-[0_0_10px_rgba(0,240,255,0.3)]">Live</span>
          </h2>
          <p class="text-sm text-text-secondary mt-1 font-medium">
            {{ activeCategory === 'overview' ? 'System Status & Quick Actions' : 'Advanced Operations' }}
          </p>
        </div>
      </div>

      <!-- Scrollable Content -->
      <div class="flex-1 overflow-y-auto p-8 custom-scrollbar bg-gradient-to-b from-transparent to-black/40">
        
        <!-- Overview Tab -->
        <div v-if="activeCategory === 'overview'" class="space-y-6 animate-slide-up">
           
           <!-- Top Row: Device Status & Telemetry -->
           <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
              <!-- Device Status (Hero) -->
              <div class="lg:col-span-2 hover-tilt-subtle transition-transform duration-500">
                <DevicePanel />
              </div>
              <!-- Live Telemetry -->
              <div class="lg:col-span-1">
                <TelemetryWidget />
              </div>
           </div>

            <!-- Middle Row: Quick Actions / Features -->
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
              <GlassCard 
                  v-for="feat in features" 
                  :key="feat.id"
                  class="group cursor-pointer hover:bg-surface/40 transition-all duration-500 border-white/5 hover:border-primary/30 hover-tilt ring-1 ring-white/5 hover:ring-primary/20"
                  @click="emit('navigate', feat.id)"
                  noPadding
              >
                  <div class="flex flex-col h-full justify-between p-5 relative overflow-hidden">
                    <!-- Glow Effect -->
                    <div class="absolute -right-10 -top-10 w-24 h-24 bg-primary/20 blur-[40px] rounded-full opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>

                    <div class="relative z-10">
                        <div class="w-10 h-10 rounded-lg bg-white/5 flex items-center justify-center mb-3 group-hover:scale-110 group-hover:bg-primary/20 transition-all duration-300 shadow-lg shadow-black/20">
                          <span class="material-symbols-rounded text-2xl" :class="feat.color">{{ feat.icon }}</span>
                        </div>
                        <h3 class="text-lg font-bold text-white group-hover:text-primary transition-colors tracking-tight">{{ feat.title }}</h3>
                        <p class="text-xs text-white/50 mt-1 leading-relaxed line-clamp-2">{{ feat.desc }}</p>
                    </div>
                  </div>
              </GlassCard>
            </div>

            <!-- Bottom Row: Activity & Community -->
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
              <RecentActivity />
              <CommunityFeed />
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
