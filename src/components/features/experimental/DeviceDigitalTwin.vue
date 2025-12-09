<script setup lang="ts">
import { ref } from 'vue';

const rotation = ref({ x: 0, y: 0 });
const container = ref<HTMLElement | null>(null);

const handleMouseMove = (e: MouseEvent) => {
  if (!container.value) return;
  const rect = container.value.getBoundingClientRect();
  const x = e.clientX - rect.left;
  const y = e.clientY - rect.top;
  
  const centerX = rect.width / 2;
  const centerY = rect.height / 2;
  
  // Calculate rotation (max 15 degrees)
  rotation.value.y = ((x - centerX) / centerX) * 15;
  rotation.value.x = -((y - centerY) / centerY) * 15;
};

const handleMouseLeave = () => {
  rotation.value = { x: 0, y: 0 };
};
</script>

<template>
  <div 
    ref="container"
    class="relative w-full h-full flex items-center justify-center perspective-1000"
    @mousemove="handleMouseMove"
    @mouseleave="handleMouseLeave"
  >
    <!-- 3D Phone Container -->
    <div 
      class="relative w-[180px] h-[360px] transition-transform duration-200 ease-out transform-style-3d"
      :style="{ transform: `rotateX(${rotation.x}deg) rotateY(${rotation.y}deg)` }"
    >
      <!-- Front Glass -->
      <div class="absolute inset-0 bg-black rounded-[2.5rem] border-[6px] border-gray-800 shadow-2xl overflow-hidden z-20 flex flex-col">
        <!-- Notch/Island -->
        <div class="absolute top-2 left-1/2 -translate-x-1/2 w-24 h-7 bg-black rounded-full z-30 flex items-center justify-center gap-2">
          <div class="w-2 h-2 rounded-full bg-gray-900 border border-gray-800"></div>
          <div class="w-1 h-1 rounded-full bg-blue-900/50"></div>
        </div>

        <!-- Screen Content -->
        <div class="flex-1 bg-surface relative overflow-hidden">
          <!-- Wallpaper -->
          <div class="absolute inset-0 bg-gradient-to-br from-primary/20 via-purple-500/10 to-black"></div>
          
          <!-- UI Elements -->
          <div class="absolute top-12 left-4 right-4">
            <div class="text-4xl font-thin text-white/80">12:45</div>
            <div class="text-xs text-white/50 mt-1">Tuesday, Dec 9</div>
          </div>

          <div class="absolute bottom-8 left-4 right-4 flex justify-between px-4">
            <div class="w-10 h-10 rounded-full bg-white/10 backdrop-blur"></div>
            <div class="w-10 h-10 rounded-full bg-white/10 backdrop-blur"></div>
            <div class="w-10 h-10 rounded-full bg-white/10 backdrop-blur"></div>
            <div class="w-10 h-10 rounded-full bg-white/10 backdrop-blur"></div>
          </div>
          
          <!-- Bootloader Mode Overlay (Hidden by default) -->
          <div class="absolute inset-0 bg-black/90 flex flex-col items-center justify-center text-center p-4 hidden">
            <span class="text-red-500 font-bold text-xl mb-2">FASTBOOT MODE</span>
            <span class="text-white/50 text-xs">PRODUCT_NAME - sdm845</span>
            <span class="text-white/50 text-xs">SECURE BOOT - disabled</span>
            <span class="text-green-500 text-xs mt-4">DEVICE STATE - UNLOCKED</span>
          </div>
        </div>
      </div>

      <!-- Side Frame (Simulated Thickness) -->
      <div class="absolute inset-0 rounded-[2.5rem] border-[6px] border-gray-700 translate-z-[-10px] bg-gray-800"></div>
      
      <!-- Glow Reflection -->
      <div class="absolute inset-0 rounded-[2.5rem] bg-gradient-to-tr from-white/0 via-white/10 to-white/0 pointer-events-none z-30 mix-blend-overlay"></div>
    </div>
    
    <!-- Floor Shadow -->
    <div class="absolute bottom-10 w-32 h-4 bg-black/50 blur-xl rounded-[100%] transform rotateX(60deg) translate-y-20"></div>
  </div>
</template>

<style scoped>
.perspective-1000 {
  perspective: 1000px;
}
.transform-style-3d {
  transform-style: preserve-3d;
}
.translate-z-\[-10px\] {
  transform: translateZ(-10px);
}
</style>
