<template>
  <div class="p-6 space-y-6 h-full overflow-y-auto relative custom-scrollbar">
    <header class="flex justify-between items-center">
      <div>
        <h2 class="text-2xl font-bold text-white tracking-tight">Social Hub</h2>
        <p class="text-white/60 text-sm">Connect with the community, share configs, and find ROMs.</p>
      </div>
    </header>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- Left Column: Community Repo -->
      <div class="lg:col-span-2 space-y-6">
        <CommunityHub />
      </div>

      <!-- Right Column: Share & Profile -->
      <div class="space-y-6">
        <ConfigShare />
        
        <!-- Placeholder for Dev Profile or other social widgets -->
        <div class="p-6 rounded-xl border border-white/10 bg-surface/30 backdrop-blur-xl">
          <h3 class="text-lg font-bold text-white mb-4">Dev Spotlight</h3>
          <div class="flex items-center gap-4 mb-4">
            <div class="w-12 h-12 rounded-full bg-primary/20 flex items-center justify-center text-primary">
              <i class="fas fa-code text-xl"></i>
            </div>
            <div>
              <div class="font-bold text-white">LineageOS Team</div>
              <div class="text-xs text-white/50">Official Maintainers</div>
            </div>
          </div>
          <p class="text-sm text-white/70 mb-4">
            Keeping legacy devices alive since 2016. Support the project!
          </p>
          <button class="w-full py-2 rounded-lg bg-white/5 hover:bg-white/10 text-white text-sm transition-colors border border-white/5">
            View Profile
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import CommunityHub from '../components/features/social/CommunityHub.vue';
import ConfigShare from '../components/features/social/ConfigShare.vue';
</script>
