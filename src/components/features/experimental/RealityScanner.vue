<script setup lang="ts">
</script>

<template>
  <div class="relative w-full h-full bg-black overflow-hidden rounded-xl border border-white/10 group">
    <!-- Camera Feed Placeholder -->
    <div class="absolute inset-0 bg-[url('https://images.unsplash.com/photo-1550751827-4bd374c3f58b?q=80&w=2070&auto=format&fit=crop')] bg-cover bg-center opacity-50 grayscale group-hover:grayscale-0 transition-all duration-500"></div>
    
    <!-- Overlay UI -->
    <div class="absolute inset-0 p-4 flex flex-col justify-between">
      <!-- Top HUD -->
      <div class="flex justify-between items-start">
        <div class="text-xs font-mono text-primary">
          REC ● 00:12:45<br>
          ISO 800
        </div>
        <div class="flex gap-1">
          <div class="w-1 h-1 bg-white/50"></div>
          <div class="w-1 h-1 bg-white/50"></div>
          <div class="w-1 h-1 bg-white/50"></div>
        </div>
      </div>

      <!-- Reticle -->
      <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-48 h-32 border border-white/30 rounded-lg flex items-center justify-center">
        <div class="w-full h-px bg-red-500/50 absolute top-1/2"></div>
        <div class="h-full w-px bg-red-500/50 absolute left-1/2"></div>
        
        <!-- Corner Markers -->
        <div class="absolute top-0 left-0 w-4 h-4 border-t-2 border-l-2 border-primary"></div>
        <div class="absolute top-0 right-0 w-4 h-4 border-t-2 border-r-2 border-primary"></div>
        <div class="absolute bottom-0 left-0 w-4 h-4 border-b-2 border-l-2 border-primary"></div>
        <div class="absolute bottom-0 right-0 w-4 h-4 border-b-2 border-r-2 border-primary"></div>

        <!-- Detected Object Label -->
        <div class="absolute -top-6 left-0 bg-primary/80 text-white text-[10px] px-2 py-0.5 rounded font-mono">
          ONEPLUS_7_PRO (98%)
        </div>
      </div>

      <!-- Bottom HUD -->
      <div class="text-center">
        <div class="inline-block px-4 py-1 bg-black/50 backdrop-blur rounded-full border border-white/10 text-xs text-white">
          Align device with frame
        </div>
      </div>
    </div>

    <!-- Scanlines -->
    <div class="absolute inset-0 bg-[linear-gradient(transparent_50%,rgba(0,0,0,0.5)_50%)] bg-[length:100%_4px] pointer-events-none opacity-20"></div>
  </div>
</template>
