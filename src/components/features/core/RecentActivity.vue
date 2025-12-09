<script setup lang="ts">
import { ref } from 'vue';
import GlassCard from '../../ui/GlassCard.vue';

const activities = ref([
  { id: 1, action: 'Flashed boot.img', time: '2 mins ago', status: 'success', icon: 'flash_on' },
  { id: 2, action: 'ADB Backup', time: '1 hour ago', status: 'success', icon: 'backup' },
  { id: 3, action: 'Unlock Bootloader', time: 'Yesterday', status: 'warning', icon: 'lock_open' },
  { id: 4, action: 'Install Magisk', time: '2 days ago', status: 'error', icon: 'security' },
]);
</script>

<template>
  <GlassCard class="h-full flex flex-col">
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-sm font-bold text-white flex items-center gap-2">
        <span class="material-symbols-rounded text-primary">history</span>
        Recent Activity
      </h3>
      <button class="text-[10px] text-primary hover:text-white transition-colors">View All</button>
    </div>

    <div class="space-y-3 flex-1 overflow-y-auto custom-scrollbar pr-2">
      <div 
        v-for="item in activities" 
        :key="item.id"
        class="flex items-center gap-3 p-3 rounded-xl bg-white/5 border border-white/5 hover:bg-white/10 transition-colors group"
      >
        <div 
          class="w-8 h-8 rounded-full flex items-center justify-center"
          :class="{
            'bg-success/20 text-success': item.status === 'success',
            'bg-warning/20 text-warning': item.status === 'warning',
            'bg-error/20 text-error': item.status === 'error',
          }"
        >
          <span class="material-symbols-rounded text-sm">{{ item.icon }}</span>
        </div>
        
        <div class="flex-1">
          <div class="text-xs font-bold text-white group-hover:text-primary transition-colors">{{ item.action }}</div>
          <div class="text-[10px] text-text-secondary">{{ item.time }}</div>
        </div>

        <div class="w-1.5 h-1.5 rounded-full" :class="{
            'bg-success shadow-[0_0_5px_rgba(48,209,88,0.5)]': item.status === 'success',
            'bg-warning shadow-[0_0_5px_rgba(255,214,10,0.5)]': item.status === 'warning',
            'bg-error shadow-[0_0_5px_rgba(255,69,58,0.5)]': item.status === 'error',
        }"></div>
      </div>
    </div>
  </GlassCard>
</template>
