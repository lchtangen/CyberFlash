<template>
  <div class="p-6 space-y-6 h-full overflow-y-auto relative custom-scrollbar">
    <header class="flex justify-between items-center">
      <div>
        <h2 class="text-2xl font-bold text-white tracking-tight">Hardware Diagnostics</h2>
        <p class="text-white/60 text-sm">Physical layer analysis and health monitoring.</p>
      </div>
    </header>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <!-- Power Monitor -->
      <PowerMonitor />

      <!-- Cable Tester -->
      <CableTester />

      <!-- Storage Health -->
      <StorageHealth />
    </div>

    <!-- Thermal Monitor (Placeholder for now, could be integrated into PowerMonitor or separate) -->
    <GlassCard class="col-span-full">
      <h3 class="text-lg font-bold text-white mb-4">Thermal Vision</h3>
      <div class="h-48 bg-black/20 rounded-xl flex items-center justify-center border border-white/5">
        <p class="text-white/30">Real-time thermal graph coming in v2.1</p>
      </div>
    </GlassCard>
  </div>
</template>

<script setup lang="ts">
import PowerMonitor from '../components/features/hardware/PowerMonitor.vue';
import CableTester from '../components/features/hardware/CableTester.vue';
import StorageHealth from '../components/features/hardware/StorageHealth.vue';
import GlassCard from '../components/ui/GlassCard.vue';
</script>
