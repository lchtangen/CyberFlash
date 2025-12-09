<script setup lang="ts">
defineProps<{
  progress: number; // 0-100
  label?: string;
  eta?: string;
}>();
</script>

<template>
  <div class="w-full">
    <div class="flex justify-between text-xs mb-1 font-mono">
      <span class="text-white/70">{{ label || 'PROCESSING' }}</span>
      <span class="text-primary">{{ progress }}%</span>
    </div>
    
    <div class="relative h-2 bg-white/5 rounded-full overflow-hidden">
      <!-- Background Pattern -->
      <div class="absolute inset-0 bg-[url('data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNCIgaGVpZ2h0PSI0IiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPjxwYXRoIGQ9Ik0wIDRMNCAwIiBzdHJva2U9InJnYmEoMjU1LDI1NSwyNTUsMC4wNSkiLz48L3N2Zz4')] opacity-50"></div>
      
      <!-- Bar -->
      <div 
        class="absolute top-0 left-0 h-full bg-gradient-to-r from-primary/50 via-primary to-white transition-all duration-300 ease-out"
        :style="{ width: `${progress}%` }"
      >
        <!-- Leading Edge Glow -->
        <div class="absolute right-0 top-1/2 -translate-y-1/2 w-2 h-4 bg-white blur-sm"></div>
      </div>
    </div>

    <div v-if="eta" class="text-[10px] text-right text-white/30 mt-1 font-mono">
      ETA: {{ eta }}
    </div>
  </div>
</template>
