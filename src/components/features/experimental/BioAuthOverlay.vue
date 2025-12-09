<script setup lang="ts">
import { ref, onMounted } from 'vue';

const isScanning = ref(true);
const isAuthenticated = ref(false);

onMounted(() => {
  setTimeout(() => {
    isScanning.value = false;
    isAuthenticated.value = true;
  }, 3000);
});
</script>

<template>
  <div class="relative w-full h-full flex flex-col items-center justify-center overflow-hidden bg-black/80 backdrop-blur-sm rounded-2xl border border-white/10">
    
    <!-- Face ID Icon Container -->
    <div class="relative w-32 h-32 mb-8">
      <!-- Static Frame -->
      <svg viewBox="0 0 100 100" class="w-full h-full text-white/20 fill-none stroke-current stroke-[2]">
        <path d="M 30 20 Q 20 20 20 30" />
        <path d="M 70 20 Q 80 20 80 30" />
        <path d="M 20 70 Q 20 80 30 80" />
        <path d="M 80 70 Q 80 80 70 80" />
      </svg>

      <!-- Animated Face Mesh -->
      <div v-if="isScanning" class="absolute inset-0 flex items-center justify-center">
        <div class="w-20 h-24 border border-primary/30 rounded-[2rem] animate-pulse relative overflow-hidden">
          <!-- Grid -->
          <div class="absolute inset-0 bg-[url('data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTAiIGhlaWdodD0iMTAiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+PHBhdGggZD0iTTEgMEwwIDBMMCAxIiBzdHJva2U9InJnYmEoMTAsIDEzMiwgMjU1LCAwLjIpIiBmaWxsPSJub25lIi8+PC9zdmc+')] opacity-50"></div>
        </div>
      </div>

      <!-- Success Icon -->
      <div v-else class="absolute inset-0 flex items-center justify-center animate-scale-in">
        <span class="material-symbols-rounded text-5xl text-success drop-shadow-[0_0_15px_rgba(48,209,88,0.6)]">lock_open</span>
      </div>

      <!-- Scanning Beam -->
      <div 
        v-if="isScanning"
        class="absolute left-0 right-0 h-1 bg-primary shadow-[0_0_15px_rgba(10,132,255,0.8)] animate-scan-vertical"
      ></div>
    </div>

    <!-- Status Text -->
    <div class="text-center space-y-2">
      <h3 class="text-xl font-bold tracking-wide" :class="isAuthenticated ? 'text-success' : 'text-white'">
        {{ isAuthenticated ? 'IDENTITY VERIFIED' : 'AUTHENTICATING...' }}
      </h3>
      <p class="text-xs text-white/50 font-mono uppercase tracking-widest">
        {{ isAuthenticated ? 'Access Granted' : 'Biometric Scan in Progress' }}
      </p>
    </div>

    <!-- Particles (Decor) -->
    <div v-if="isScanning" class="absolute inset-0 pointer-events-none">
      <div class="absolute top-1/4 left-1/4 w-1 h-1 bg-primary rounded-full animate-ping" style="animation-delay: 0.2s"></div>
      <div class="absolute top-3/4 right-1/3 w-1 h-1 bg-primary rounded-full animate-ping" style="animation-delay: 0.5s"></div>
      <div class="absolute bottom-1/4 left-2/3 w-1 h-1 bg-primary rounded-full animate-ping" style="animation-delay: 0.8s"></div>
    </div>

  </div>
</template>

<style scoped>
@keyframes scan-vertical {
  0% { top: 10%; opacity: 0; }
  10% { opacity: 1; }
  90% { opacity: 1; }
  100% { top: 90%; opacity: 0; }
}
.animate-scan-vertical {
  animation: scan-vertical 1.5s cubic-bezier(0.4, 0, 0.2, 1) infinite;
}
.animate-scale-in {
  animation: scale-in 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}
@keyframes scale-in {
  from { transform: scale(0); opacity: 0; }
  to { transform: scale(1); opacity: 1; }
}
</style>
