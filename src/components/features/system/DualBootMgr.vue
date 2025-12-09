<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';
import GlassInput from '../../ui/GlassInput.vue';

const activeSlot = ref<'a' | 'b'>('a');
const loading = ref(false);
const message = ref<string | null>(null);

// DSU State
const dsuFile = ref('');
const dsuSize = ref(8); // GB

const switchSlot = async (slot: 'a' | 'b') => {
  loading.value = true;
  message.value = null;
  
  try {
    const res = await invoke<string>('switch_dual_boot_slot', { slot });
    message.value = res;
    activeSlot.value = slot;
  } catch (err: any) {
    message.value = `Error: ${err}`;
  } finally {
    loading.value = false;
  }
};

const selectDsuFile = async () => {
  const file = await open({
    filters: [{ name: 'System Image', extensions: ['img', 'gz'] }]
  });
  if (file) {
    dsuFile.value = Array.isArray(file) ? file[0] : file;
  }
};

const installDsu = async () => {
  if (!dsuFile.value) return;
  loading.value = true;
  message.value = "Installing DSU... This may take a while.";
  
  try {
    await invoke<string>('install_dsu', { 
      systemImagePath: dsuFile.value,
      userdataSizeGb: dsuSize.value 
    });
    message.value = "DSU Installation Started! Check device notification.";
  } catch (err: any) {
    message.value = `DSU Error: ${err}`;
  } finally {
    loading.value = false;
  }
};
</script>

<template>
  <GlassCard class="h-full flex flex-col overflow-y-auto custom-scrollbar">
    <div class="flex items-center space-x-3 mb-6">
      <div class="p-2 rounded-lg bg-teal-500/20 text-teal-400">
        <span class="material-symbols-rounded text-xl">splitscreen</span>
      </div>
      <div>
        <h3 class="text-lg font-semibold text-white">Boot Manager</h3>
        <p class="text-xs text-text-secondary">A/B Slots & DSU Loader</p>
      </div>
    </div>

    <div class="space-y-8">
      <!-- A/B Slot Manager -->
      <div class="flex flex-col items-center justify-center space-y-6 p-4 rounded-xl bg-white/5 border border-white/5">
        <h4 class="text-sm font-medium text-white/80">Active Slot</h4>
        <div class="flex items-center space-x-8">
          <div 
            class="flex flex-col items-center space-y-2 cursor-pointer transition-all duration-300"
            :class="activeSlot === 'a' ? 'scale-110 opacity-100' : 'opacity-50 hover:opacity-80'"
            @click="switchSlot('a')"
          >
            <div class="w-16 h-16 rounded-2xl flex items-center justify-center text-2xl font-bold border-2"
              :class="activeSlot === 'a' ? 'bg-primary/20 border-primary text-primary' : 'bg-surface/30 border-white/10 text-white'"
            >
              A
            </div>
            <span class="text-sm font-medium">Slot A</span>
          </div>

          <div class="h-px w-12 bg-white/10"></div>

          <div 
            class="flex flex-col items-center space-y-2 cursor-pointer transition-all duration-300"
            :class="activeSlot === 'b' ? 'scale-110 opacity-100' : 'opacity-50 hover:opacity-80'"
            @click="switchSlot('b')"
          >
            <div class="w-16 h-16 rounded-2xl flex items-center justify-center text-2xl font-bold border-2"
              :class="activeSlot === 'b' ? 'bg-primary/20 border-primary text-primary' : 'bg-surface/30 border-white/10 text-white'"
            >
              B
            </div>
            <span class="text-sm font-medium">Slot B</span>
          </div>
        </div>
      </div>

      <!-- DSU Loader -->
      <div class="p-4 rounded-xl bg-white/5 border border-white/5 space-y-4">
        <div class="flex items-center justify-between">
          <h4 class="text-sm font-medium text-white/80">Dynamic System Update (GSI)</h4>
          <span class="text-xs text-primary bg-primary/10 px-2 py-0.5 rounded">Android 10+</span>
        </div>
        
        <div class="space-y-3">
          <div class="flex gap-2">
            <GlassInput v-model="dsuFile" placeholder="Select system.img..." class="flex-1" readonly />
            <VisionButton icon="folder_open" variant="secondary" @click="selectDsuFile">Browse</VisionButton>
          </div>
          
          <div class="flex items-center gap-4">
            <div class="flex-1">
              <label class="text-xs text-white/60 mb-1 block">Userdata Size (GB)</label>
              <input type="range" v-model.number="dsuSize" min="2" max="32" class="w-full accent-primary" />
            </div>
            <div class="text-sm font-mono w-12 text-right">{{ dsuSize }}G</div>
          </div>

          <VisionButton 
            class="w-full" 
            variant="primary" 
            icon="rocket_launch" 
            :loading="loading"
            @click="installDsu"
            :disabled="!dsuFile"
          >
            Boot GSI via DSU
          </VisionButton>
        </div>
      </div>

      <div v-if="message" class="text-sm text-white bg-white/5 px-3 py-2 rounded-lg border border-white/10">
        {{ message }}
      </div>
    </div>
  </GlassCard>
</template>
