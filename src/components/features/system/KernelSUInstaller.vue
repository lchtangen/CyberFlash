<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';
import GlassInput from '../../ui/GlassInput.vue';

const method = ref('direct');
const loading = ref(false);
const result = ref<string | null>(null);
const bootImagePath = ref('');

const selectBootImage = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'Boot Images',
        extensions: ['img']
      }]
    });
    if (selected && typeof selected === 'string') {
      bootImagePath.value = selected;
    }
  } catch (e) {
    console.error('File selection failed', e);
  }
};

const install = async () => {
  if (method.value === 'patch' && !bootImagePath.value) {
    result.value = "Please select a boot image to patch.";
    return;
  }

  loading.value = true;
  result.value = null;
  
  try {
    const res = await invoke<string>('install_kernelsu', { 
      method: method.value,
      filePath: method.value === 'patch' ? bootImagePath.value : undefined
    });
    result.value = res;
  } catch (err: any) {
    result.value = `Error: ${err}`;
  } finally {
    loading.value = false;
  }
};
</script>

<template>
  <GlassCard class="h-full flex flex-col">
    <div class="flex items-center space-x-3 mb-6">
      <div class="p-2 rounded-lg bg-purple-500/20 text-purple-400">
        <span class="material-symbols-rounded text-xl">security</span>
      </div>
      <div>
        <h3 class="text-lg font-semibold text-white">KernelSU Installer</h3>
        <p class="text-xs text-text-secondary">Root via Kernel (KSU)</p>
      </div>
    </div>

    <div class="space-y-4 flex-1">
      <div class="space-y-2">
        <label class="text-xs font-medium text-text-secondary ml-1">Installation Method</label>
        <div class="grid grid-cols-2 gap-2">
          <button 
            @click="method = 'direct'"
            class="p-3 rounded-xl border text-sm transition-all"
            :class="method === 'direct' ? 'bg-primary/20 border-primary text-white' : 'bg-surface/30 border-white/10 text-text-secondary hover:bg-surface/50'"
          >
            Direct Install
          </button>
          <button 
            @click="method = 'patch'"
            class="p-3 rounded-xl border text-sm transition-all"
            :class="method === 'patch' ? 'bg-primary/20 border-primary text-white' : 'bg-surface/30 border-white/10 text-text-secondary hover:bg-surface/50'"
          >
            Patch Boot Image
          </button>
        </div>
      </div>

      <div class="text-xs text-text-secondary p-2">
        <p v-if="method === 'direct'">
          Directly flashes KernelSU to the current boot partition. Requires unlocked bootloader.
        </p>
        <div v-else class="space-y-3">
          <p>Patches a selected boot.img file which you can then flash manually.</p>
          <div class="flex gap-2">
            <GlassInput 
              v-model="bootImagePath" 
              placeholder="Select boot.img..." 
              readonly 
              class="flex-1"
            />
            <VisionButton @click="selectBootImage" variant="secondary">
              Browse
            </VisionButton>
          </div>
        </div>
      </div>

      <div v-if="result" class="p-3 rounded-lg bg-white/5 text-sm text-white break-all">
        {{ result }}
      </div>
    </div>

    <div class="mt-6">
      <VisionButton 
        @click="install" 
        :loading="loading"
        class="w-full"
      >
        {{ method === 'direct' ? 'Install KernelSU' : 'Patch Boot Image' }}
      </VisionButton>
    </div>
  </GlassCard>
</template>
