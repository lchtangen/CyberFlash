<script setup lang="ts">
import { ref } from 'vue';

const device = ref({
  model: 'OnePlus 7 Pro',
  codename: 'guacamole',
  status: 'connected',
  battery: 85,
  serial: '72939284'
});
</script>

<template>
  <div class="bg-white/5 border border-white/10 rounded-xl p-4 backdrop-blur-md relative overflow-hidden group">
    <div class="absolute top-0 right-0 p-4 opacity-10 group-hover:opacity-20 transition-opacity">
      <span class="material-icons text-6xl text-white">smartphone</span>
    </div>
    
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-semibold text-white">Device Status</h3>
      <div class="flex items-center gap-2 px-2 py-1 rounded-full bg-green-500/20 border border-green-500/30">
        <div class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></div>
        <span class="text-xs font-medium text-green-400 uppercase">Online</span>
      </div>
    </div>

    <div class="space-y-4 relative z-10">
      <div>
        <p class="text-xs text-gray-400 uppercase tracking-wider">Model</p>
        <p class="text-xl font-bold text-white tracking-wide">{{ device.model }}</p>
        <p class="text-xs text-gray-500 font-mono">{{ device.codename }}</p>
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div>
          <p class="text-xs text-gray-400 uppercase tracking-wider">Battery</p>
          <div class="flex items-center gap-2 mt-1">
            <span class="material-icons text-green-400 text-sm">battery_std</span>
            <span class="text-white font-mono">{{ device.battery }}%</span>
          </div>
        </div>
        <div>
          <p class="text-xs text-gray-400 uppercase tracking-wider">Serial</p>
          <p class="text-white font-mono text-sm mt-1 truncate">{{ device.serial }}</p>
        </div>
      </div>
    </div>
  </div>
</template>
