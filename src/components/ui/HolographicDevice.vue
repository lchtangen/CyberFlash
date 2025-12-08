<script setup lang="ts">
import { computed } from 'vue';
import { useDeviceStore } from '../../stores/device';

const deviceStore = useDeviceStore();

const isConnected = computed(() => deviceStore.isConnected);
const model = computed(() => deviceStore.deviceModel || 'Unknown Device');
const status = computed(() => deviceStore.connectionType || 'Disconnected');

</script>

<template>
  <div class="perspective-1000 w-full h-64 flex items-center justify-center relative group">
    <!-- Floating Container with 3D Tilt -->
    <div class="relative w-32 h-64 transition-transform duration-500 ease-out transform-style-3d group-hover:rotate-y-12 group-hover:rotate-x-6">
      
      <!-- Layer 1: Backplate (Hardware) -->
      <div class="absolute inset-0 bg-black/80 border border-white/10 rounded-[2rem] shadow-2xl transform translate-z-[-20px] flex flex-col items-center justify-center">
        <div class="w-full h-full bg-grid-white/[0.05] rounded-[2rem]"></div>
      </div>

      <!-- Layer 2: Midframe (System) -->
      <div class="absolute inset-0 bg-surface/90 border border-white/20 rounded-[2rem] transform translate-z-[0px] shadow-xl backdrop-blur-sm flex flex-col items-center pt-4">
        <!-- Camera Notch -->
        <div class="w-12 h-4 bg-black rounded-full mb-2"></div>
        <!-- Screen Content -->
        <div class="w-full h-full px-3 pb-3 flex flex-col">
          <div class="flex-1 bg-black/50 rounded-xl border border-white/5 p-3 flex flex-col items-center justify-center gap-2 overflow-hidden relative">
            <!-- Scanline Effect -->
            <div class="absolute inset-0 bg-gradient-to-b from-transparent via-primary/10 to-transparent animate-scan pointer-events-none"></div>
            
            <span class="material-symbols-rounded text-4xl" :class="isConnected ? 'text-primary' : 'text-text-secondary'">smartphone</span>
            <div class="text-center">
              <div class="text-xs text-text-secondary font-mono uppercase tracking-wider">Status</div>
              <div class="text-sm font-bold" :class="isConnected ? 'text-success' : 'text-error'">{{ status }}</div>
            </div>
          </div>
        </div>
      </div>

      <!-- Layer 3: Glass (UI/Overlay) -->
      <div class="absolute inset-0 bg-gradient-to-tr from-white/5 to-transparent border border-white/10 rounded-[2rem] transform translate-z-[20px] pointer-events-none shadow-inner">
        <!-- Reflection -->
        <div class="absolute inset-0 bg-gradient-to-tr from-white/10 via-transparent to-transparent rounded-[2rem]"></div>
      </div>

      <!-- Connection Particles (if connected) -->
      <div v-if="isConnected" class="absolute -inset-4 border border-primary/30 rounded-[2.5rem] animate-pulse-slow transform translate-z-[-10px]"></div>
    </div>

    <!-- Floor Reflection/Shadow -->
    <div class="absolute bottom-0 w-32 h-4 bg-black/50 blur-xl rounded-[100%] transform rotate-x-90 translate-y-12 scale-110 group-hover:scale-125 transition-transform duration-500"></div>
  </div>
</template>

<style scoped>
.perspective-1000 {
  perspective: 1000px;
}
.transform-style-3d {
  transform-style: preserve-3d;
}
.rotate-y-12 {
  transform: rotateY(12deg);
}
.rotate-x-6 {
  transform: rotateX(6deg);
}
.translate-z-\[-20px\] {
  transform: translateZ(-20px);
}
.translate-z-\[0px\] {
  transform: translateZ(0px);
}
.translate-z-\[20px\] {
  transform: translateZ(20px);
}
.translate-z-\[-10px\] {
  transform: translateZ(-10px);
}

@keyframes scan {
  0% { transform: translateY(-100%); }
  100% { transform: translateY(100%); }
}
.animate-scan {
  animation: scan 3s linear infinite;
}
</style>
