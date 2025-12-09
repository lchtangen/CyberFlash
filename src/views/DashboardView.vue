<script setup lang="ts">
import DevicePanel from '../components/features/core/DevicePanel.vue';
import AIChatInterface from '../components/features/ai/AIChatInterface.vue';
import Terminal from '../components/features/system/Terminal.vue';
import CyberDeck from '../components/features/core/CyberDeck.vue';
import GlassCard from '../components/ui/GlassCard.vue';
import VisionButton from '../components/ui/VisionButton.vue';

</script>

<template>
  <div class="p-6 space-y-6 h-full overflow-y-auto relative custom-scrollbar">
    <header class="flex justify-between items-center">
      <div>
        <h2 class="text-2xl font-bold text-white tracking-tight">Dashboard</h2>
        <p class="text-text-secondary text-sm">Device Overview & Status</p>
      </div>
      <div class="flex gap-3">
        <VisionButton variant="secondary" icon="refresh" size="sm">
          Refresh
        </VisionButton>
      </div>
    </header>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- Left Column -->
      <div class="lg:col-span-2 space-y-6">
        <DevicePanel />
        
        <GlassCard>
          <h3 class="text-lg font-semibold text-white mb-4 flex items-center gap-2 tracking-tight">
            <span class="material-symbols-rounded text-primary">terminal</span>
            System Logs
          </h3>
          <Terminal />
        </GlassCard>
      </div>

      <!-- Right Column -->
      <div class="space-y-6">
        <AIChatInterface heightClass="h-[400px]" />
        
        <CyberDeck />
      </div>
    </div>
  </div>
</template>
