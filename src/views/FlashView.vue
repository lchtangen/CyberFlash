<script setup lang="ts">
import { ref } from 'vue';
import DeviceStatusPanel from '../components/features/DeviceStatusPanel.vue';

const currentStep = ref(1);
const steps = ['Select ROM', 'Verify Device', 'Flash', 'Complete'];
</script>

<template>
  <div class="p-6 h-full flex flex-col">
    <header class="mb-8">
      <h2 class="text-2xl font-bold text-white">Flash Firmware</h2>
      <p class="text-text-secondary">Install new ROMs safely</p>
    </header>

    <div class="flex-1 flex gap-6">
      <div class="flex-1 flex flex-col gap-6">
        <!-- Progress -->
        <div class="bg-white/5 border border-white/10 rounded-xl p-6 backdrop-blur-md">
           <div class="flex justify-between relative">
             <div class="absolute top-1/2 left-0 w-full h-0.5 bg-white/10 -z-10"></div>
             <div 
               v-for="(step, index) in steps" 
               :key="step"
               class="flex flex-col items-center gap-2 bg-[#121212] px-2"
             >
               <div 
                 class="w-8 h-8 rounded-full flex items-center justify-center text-sm font-bold transition-colors"
                 :class="currentStep > index + 1 ? 'bg-success text-black' : currentStep === index + 1 ? 'bg-primary text-black shadow-[0_0_10px_rgba(10,132,255,0.5)]' : 'bg-white/10 text-gray-500'"
               >
                 {{ currentStep > index + 1 ? '✓' : index + 1 }}
               </div>
               <span 
                 class="text-xs font-medium"
                 :class="currentStep === index + 1 ? 'text-white' : 'text-gray-500'"
               >
                 {{ step }}
               </span>
             </div>
           </div>
        </div>

        <!-- Main Content Area -->
        <div class="flex-1 bg-white/5 border border-white/10 rounded-xl p-6 backdrop-blur-md relative overflow-hidden">
          <div v-if="currentStep === 1" class="h-full flex flex-col items-center justify-center text-center space-y-4">
            <div class="w-20 h-20 rounded-full bg-white/5 flex items-center justify-center mb-4">
              <span class="material-icons text-4xl text-gray-400">upload_file</span>
            </div>
            <h3 class="text-xl font-semibold text-white">Select Firmware</h3>
            <p class="text-gray-400 max-w-md">Drag and drop your ROM zip file here, or browse to select. Supports .zip, .img, and payload.bin</p>
            <button class="px-6 py-3 bg-primary text-white rounded-lg hover:bg-primary/90 transition-colors font-medium shadow-lg shadow-primary/20">
              Browse Files
            </button>
          </div>
        </div>
      </div>

      <!-- Sidebar -->
      <div class="w-80 space-y-6">
        <DeviceStatusPanel />
        
        <div class="bg-white/5 border border-white/10 rounded-xl p-4 backdrop-blur-md">
          <h3 class="text-sm font-bold text-gray-400 uppercase tracking-wider mb-3">Flash Options</h3>
          <div class="space-y-3">
            <label class="flex items-center gap-3 p-3 rounded-lg hover:bg-white/5 cursor-pointer transition-colors">
              <input type="checkbox" class="w-4 h-4 rounded border-gray-600 text-primary focus:ring-primary bg-transparent">
              <span class="text-sm text-gray-300">Wipe Data / Factory Reset</span>
            </label>
            <label class="flex items-center gap-3 p-3 rounded-lg hover:bg-white/5 cursor-pointer transition-colors">
              <input type="checkbox" class="w-4 h-4 rounded border-gray-600 text-primary focus:ring-primary bg-transparent">
              <span class="text-sm text-gray-300">Disable Verity / V-B Meta</span>
            </label>
            <label class="flex items-center gap-3 p-3 rounded-lg hover:bg-white/5 cursor-pointer transition-colors">
              <input type="checkbox" class="w-4 h-4 rounded border-gray-600 text-primary focus:ring-primary bg-transparent">
              <span class="text-sm text-gray-300">Inject Magisk (Root)</span>
            </label>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
