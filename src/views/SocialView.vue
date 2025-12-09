<script setup lang="ts">
import { ref, computed } from 'vue';
import SidebarItem from '../components/ui/SidebarItem.vue';

// Components
import CommunityHub from '../components/features/social/CommunityHub.vue';
import ConfigShare from '../components/features/social/ConfigShare.vue';
import GlassCard from '../components/ui/GlassCard.vue';

const activeCategory = ref('community');

const categories = [
  { id: 'community', label: 'Community Repo', icon: 'hub' },
  { id: 'share', label: 'Share Config', icon: 'share' },
  { id: 'profile', label: 'Dev Profile', icon: 'person' },
];

const activeCategoryLabel = computed(() => categories.find(c => c.id === activeCategory.value)?.label);
</script>

<template>
  <div class="flex h-full gap-6 overflow-hidden">
    <!-- Sidebar Navigation -->
    <GlassCard noPadding class="w-64 flex-shrink-0 flex flex-col overflow-hidden">
      <div class="p-5 border-b border-white/10 bg-white/5 backdrop-blur-md">
        <h2 class="text-lg font-bold text-white tracking-tight flex items-center gap-2">
          <span class="material-symbols-rounded text-secondary">hub</span>
          Social Hub
        </h2>
        <p class="text-xs text-text-secondary mt-1">Connect & Share</p>
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
    <GlassCard noPadding class="flex-1 flex flex-col min-w-0 overflow-hidden relative">
      <!-- Header -->
      <div class="p-6 border-b border-white/10 flex justify-between items-center bg-white/5 backdrop-blur-md z-10">
        <div>
          <h2 class="text-xl font-bold text-white tracking-tight flex items-center gap-2">
            {{ activeCategoryLabel }}
            <span class="px-2 py-0.5 rounded-full bg-secondary/20 text-secondary text-[10px] font-bold border border-secondary/20 uppercase tracking-wider">Global</span>
          </h2>
          <p class="text-sm text-text-secondary mt-1">Community interactions and resources</p>
        </div>
      </div>

      <!-- Scrollable Content -->
      <div class="flex-1 overflow-y-auto p-8 custom-scrollbar bg-gradient-to-b from-transparent to-black/20">
        
        <!-- Community Tab -->
        <div v-if="activeCategory === 'community'" class="space-y-8 animate-slide-up">
          <div class="hover-tilt-subtle">
            <CommunityHub />
          </div>
        </div>

        <!-- Share Tab -->
        <div v-if="activeCategory === 'share'" class="space-y-8 animate-slide-up">
          <div class="hover-tilt-subtle">
            <ConfigShare />
          </div>
        </div>

        <!-- Profile Tab -->
        <div v-if="activeCategory === 'profile'" class="space-y-8 animate-slide-up">
          <GlassCard class="p-8 max-w-2xl mx-auto hover-tilt">
            <h3 class="text-lg font-bold text-white mb-6 flex items-center gap-2">
              <span class="material-symbols-rounded text-warning">star</span>
              Dev Spotlight
            </h3>
            <div class="flex items-center gap-6 mb-6">
              <div class="w-20 h-20 rounded-2xl bg-primary/20 flex items-center justify-center text-primary border border-primary/30 shadow-lg shadow-primary/10">
                <span class="material-symbols-rounded text-4xl">code</span>
              </div>
              <div>
                <div class="font-bold text-white text-xl tracking-tight">LineageOS Team</div>
                <div class="text-xs text-primary uppercase tracking-wider font-bold mt-1 bg-primary/10 px-2 py-1 rounded-md inline-block border border-primary/20">Official Maintainers</div>
              </div>
            </div>
            <p class="text-sm text-white/70 mb-8 leading-relaxed">
              Keeping legacy devices alive since 2016. Support the project!
            </p>
            <button class="w-full py-3 rounded-xl bg-white/5 hover:bg-white/10 text-white text-sm transition-all duration-300 border border-white/5 font-medium hover:scale-[1.02] active:scale-95">
              View Full Profile
            </button>
          </GlassCard>
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
