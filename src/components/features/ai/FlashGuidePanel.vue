<script setup lang="ts">
import { ref } from 'vue';
import GlassCard from '../../ui/GlassCard.vue';

interface Guide {
  id: string;
  title: string;
  description: string;
  icon: string;
  steps: string[];
}

const guides: Guide[] = [
  {
    id: 'unlock',
    title: 'Unlock Bootloader',
    description: 'Essential first step for custom ROMs',
    icon: 'lock_open',
    steps: [
      'Enable Developer Options in Android Settings.',
      'Enable "OEM Unlocking" and "USB Debugging".',
      'Reboot device into Fastboot Mode.',
      'Run command: fastboot flashing unlock',
      'Confirm on device screen using volume keys.'
    ]
  },
  {
    id: 'flash',
    title: 'Flash Custom ROM',
    description: 'Install a new OS on your device',
    icon: 'system_update',
    steps: [
      'Download the ROM zip file.',
      'Reboot into Custom Recovery (TWRP/OrangeFox).',
      'Wipe Data, Cache, and Dalvik Cache.',
      'Select "Install" and choose the ROM zip.',
      'Swipe to confirm flash.',
      'Reboot System.'
    ]
  },
  {
    id: 'root',
    title: 'Root with Magisk',
    description: 'Gain administrative privileges',
    icon: 'security',
    steps: [
      'Download the latest Magisk APK.',
      'Rename the .apk extension to .zip.',
      'Reboot into Custom Recovery.',
      'Flash the Magisk zip file.',
      'Reboot System and install the Magisk app.'
    ]
  }
];

const selectedGuide = ref<Guide | null>(null);
</script>

<template>
  <GlassCard noPadding class="w-full h-full flex flex-col overflow-hidden border-white/10 shadow-2xl shadow-black/50">
    <!-- Header -->
    <div class="px-5 py-4 border-b border-white/5 flex items-center justify-between bg-white/5 backdrop-blur-md z-10">
      <div class="flex items-center gap-4">
        <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-yellow-500/20 to-yellow-500/5 flex items-center justify-center border border-white/10 shadow-inner">
          <span class="material-symbols-rounded text-yellow-400 text-xl">menu_book</span>
        </div>
        <div>
          <h3 class="font-bold text-white text-base tracking-wide">Flash Guide</h3>
          <p class="text-xs text-text-secondary font-medium">Interactive Tutorials</p>
        </div>
      </div>
      
      <button 
        @click="$emit('close')"
        class="p-2 rounded-lg hover:bg-white/10 text-text-secondary hover:text-white transition-all duration-200 group"
      >
        <span class="material-symbols-rounded text-lg group-hover:scale-110 transition-transform">close</span>
      </button>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto p-5 custom-scrollbar">
      
      <!-- Guide List -->
      <div v-if="!selectedGuide" class="space-y-3">
        <button 
          v-for="guide in guides" 
          :key="guide.id"
          @click="selectedGuide = guide"
          class="w-full text-left p-4 rounded-xl bg-surface/50 border border-white/5 hover:bg-white/10 hover:border-white/20 transition-all group flex items-center gap-4"
        >
          <div class="w-10 h-10 rounded-full bg-white/5 flex items-center justify-center group-hover:scale-110 transition-transform">
            <span class="material-symbols-rounded text-white/80">{{ guide.icon }}</span>
          </div>
          <div class="flex-1">
            <h4 class="font-bold text-white text-sm">{{ guide.title }}</h4>
            <p class="text-xs text-text-secondary">{{ guide.description }}</p>
          </div>
          <span class="material-symbols-rounded text-white/30 group-hover:translate-x-1 transition-transform">chevron_right</span>
        </button>
      </div>

      <!-- Guide Detail -->
      <div v-else class="animate-fade-in">
        <button 
          @click="selectedGuide = null"
          class="mb-4 flex items-center gap-2 text-xs text-text-secondary hover:text-white transition-colors"
        >
          <span class="material-symbols-rounded text-sm">arrow_back</span>
          Back to Guides
        </button>

        <div class="bg-surface/50 border border-white/10 rounded-xl p-6">
          <div class="flex items-center gap-3 mb-6">
            <span class="material-symbols-rounded text-2xl text-primary">{{ selectedGuide.icon }}</span>
            <h2 class="text-xl font-bold text-white">{{ selectedGuide.title }}</h2>
          </div>

          <div class="space-y-6">
            <div v-for="(step, index) in selectedGuide.steps" :key="index" class="flex gap-4">
              <div class="flex flex-col items-center">
                <div class="w-6 h-6 rounded-full bg-primary/20 border border-primary/50 flex items-center justify-center text-xs font-bold text-primary">
                  {{ index + 1 }}
                </div>
                <div v-if="index !== selectedGuide.steps.length - 1" class="w-px h-full bg-white/10 my-2"></div>
              </div>
              <p class="text-sm text-white/90 pt-0.5">{{ step }}</p>
            </div>
          </div>
        </div>
      </div>

    </div>
  </GlassCard>
</template>
