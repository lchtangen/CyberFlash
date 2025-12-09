<template>
  <GlassCard class="max-w-2xl mx-auto">
    <div class="space-y-6">
      <div class="text-center">
        <h2 class="text-2xl font-bold text-white mb-2">Share Your Setup</h2>
        <p class="text-white/60">Generate a unique link to share your current ROM, Kernel, and Magisk configuration.</p>
      </div>

      <div class="bg-surface/50 rounded-xl p-4 border border-white/5 space-y-3">
        <div class="flex justify-between items-center pb-3 border-b border-white/5">
          <span class="text-white/70">Device</span>
          <span class="font-mono text-primary">{{ config.device }}</span>
        </div>
        <div class="flex justify-between items-center pb-3 border-b border-white/5">
          <span class="text-white/70">ROM</span>
          <span class="font-mono text-white">{{ config.rom }}</span>
        </div>
        <div class="flex justify-between items-center pb-3 border-b border-white/5">
          <span class="text-white/70">Magisk</span>
          <span :class="config.magisk ? 'text-success' : 'text-white/50'">
            {{ config.magisk ? 'Enabled' : 'Disabled' }}
          </span>
        </div>
        <div class="flex justify-between items-start">
          <span class="text-white/70">Modules</span>
          <div class="text-right">
            <div v-for="mod in config.modules" :key="mod" class="text-xs text-white/80 bg-white/10 px-2 py-1 rounded mb-1 inline-block ml-1">
              {{ mod }}
            </div>
            <span v-if="config.modules.length === 0" class="text-white/30 italic">None</span>
          </div>
        </div>
      </div>

      <div v-if="generatedLink" class="space-y-2 animate-fade-in">
        <label class="text-xs font-bold text-white/50 uppercase tracking-wider">Share Link</label>
        <div class="flex gap-2">
          <div class="flex-grow bg-black/30 rounded-lg border border-white/10 p-3 font-mono text-xs text-primary truncate select-all">
            {{ generatedLink }}
          </div>
          <VisionButton @click="copyLink" variant="secondary">
            <i class="fas fa-copy"></i>
          </VisionButton>
        </div>
      </div>

      <div class="flex justify-center pt-4">
        <VisionButton size="lg" @click="generateLink" :loading="loading">
          <i class="fas fa-share-alt mr-2"></i>
          Generate Link
        </VisionButton>
      </div>
    </div>
  </GlassCard>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '@/components/ui/GlassCard.vue';
import VisionButton from '@/components/ui/VisionButton.vue';

// Mock data - in real app, this would come from a store
const config = reactive({
  device: 'OnePlus 7 Pro',
  rom: 'LineageOS 21',
  gapps: 'MindTheGapps',
  magisk: true,
  modules: ['SafetyNet Fix', 'AOSP Mods']
});

const generatedLink = ref<string | null>(null);
const loading = ref(false);

const generateLink = async () => {
  loading.value = true;
  try {
    const link = await invoke<string>('generate_share_link', { config });
    generatedLink.value = link;
  } catch (e) {
    console.error(e);
    // toast.error("Failed to generate link");
  } finally {
    loading.value = false;
  }
};

const copyLink = async () => {
  if (generatedLink.value) {
    await navigator.clipboard.writeText(generatedLink.value);
    // toast.success("Link copied to clipboard");
  }
};
</script>
