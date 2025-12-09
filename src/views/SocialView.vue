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
  <div class="flex h-full gap-6 overflow-hidden p-6">
    <!-- Sidebar Navigation -->
    <div class="w-64 flex-shrink-0 bg-surface/30 border border-white/10 rounded-2xl backdrop-blur-xl flex flex-col overflow-hidden">
      <div class="p-4 border-b border-white/10">
        <h2 class="text-lg font-bold text-white tracking-tight">Social Hub</h2>
        <p class="text-xs text-text-secondary">Connect & Share</p>
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
          <p class="text-sm text-text-secondary">Community interactions and resources</p>
        </div>
      </div>

      <!-- Scrollable Content -->
      <div class="flex-1 overflow-y-auto p-6 custom-scrollbar">
        
        <!-- Community Tab -->
        <div v-if="activeCategory === 'community'" class="space-y-6">
          <CommunityHub />
        </div>

        <!-- Share Tab -->
        <div v-if="activeCategory === 'share'" class="space-y-6">
          <ConfigShare />
        </div>

        <!-- Profile Tab -->
        <div v-if="activeCategory === 'profile'" class="space-y-6">
          <div class="p-6 rounded-xl border border-white/10 bg-surface/30 backdrop-blur-xl max-w-2xl mx-auto">
            <h3 class="text-lg font-bold text-white mb-4">Dev Spotlight</h3>
            <div class="flex items-center gap-4 mb-4">
              <div class="w-16 h-16 rounded-full bg-primary/20 flex items-center justify-center text-primary border border-primary/30">
                <span class="material-symbols-rounded text-3xl">code</span>
              </div>
              <div>
                <div class="font-bold text-white text-lg">LineageOS Team</div>
                <div class="text-xs text-white/50 uppercase tracking-wider font-bold">Official Maintainers</div>
              </div>
            </div>
            <p class="text-sm text-white/70 mb-6 leading-relaxed">
              Keeping legacy devices alive since 2016. Support the project!
            </p>
            <button class="w-full py-3 rounded-xl bg-white/5 hover:bg-white/10 text-white text-sm transition-colors border border-white/5 font-medium">
              View Full Profile
            </button>
          </div>
        </div>

      </div>
    </div>
  </div>
</template>
