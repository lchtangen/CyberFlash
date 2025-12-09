<script setup lang="ts">
import GlassCard from '../../ui/GlassCard.vue';

const news = [
  { id: 1, title: 'LineageOS 21 Released', source: 'XDA Developers', time: '4h ago', image: 'https://images.unsplash.com/photo-1511707171634-5f897ff02aa9?q=80&w=1000&auto=format&fit=crop' },
  { id: 2, title: 'Magisk v27.0 Update', source: 'GitHub', time: '1d ago', image: 'https://images.unsplash.com/photo-1555949963-ff9fe0c870eb?q=80&w=1000&auto=format&fit=crop' },
];
</script>

<template>
  <GlassCard class="h-full flex flex-col">
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-sm font-bold text-white flex items-center gap-2">
        <span class="material-symbols-rounded text-secondary">feed</span>
        Community Feed
      </h3>
    </div>

    <div class="space-y-3">
      <div 
        v-for="item in news" 
        :key="item.id"
        class="relative h-24 rounded-xl overflow-hidden group cursor-pointer border border-white/10"
      >
        <!-- Background Image -->
        <div class="absolute inset-0 bg-cover bg-center transition-transform duration-500 group-hover:scale-110" :style="{ backgroundImage: `url(${item.image})` }"></div>
        <div class="absolute inset-0 bg-gradient-to-t from-black/90 via-black/50 to-transparent"></div>

        <!-- Content -->
        <div class="absolute bottom-0 left-0 right-0 p-3">
          <div class="text-xs font-bold text-white leading-tight mb-1 group-hover:text-primary transition-colors">{{ item.title }}</div>
          <div class="flex justify-between items-center text-[10px] text-white/60">
            <span>{{ item.source }}</span>
            <span>{{ item.time }}</span>
          </div>
        </div>
      </div>
    </div>
  </GlassCard>
</template>
