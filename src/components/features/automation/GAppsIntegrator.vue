<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-shell';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';

const androidVersion = ref('14');
const arch = ref('arm64');
const variant = ref('nano');

const openDownload = async () => {
  try {
    const url = await invoke<string>('get_gapps_url', {
      androidVersion: androidVersion.value,
      arch: arch.value,
      variant: variant.value
    });
    await open(url);
  } catch (e) {
    console.error(e);
  }
};
</script>

<template>
  <GlassCard>
    <div class="flex items-center gap-3 mb-6">
      <div class="p-3 rounded-xl bg-primary/20 text-primary">
        <span class="material-symbols-rounded text-2xl">extension</span>
      </div>
      <div>
        <h2 class="text-xl font-bold text-white">GApps Integrator</h2>
        <p class="text-sm text-white/60">Find the right Google Apps package.</p>
      </div>
    </div>

    <div class="space-y-4">
      <div class="grid grid-cols-3 gap-4">
        <div>
          <label class="text-xs text-white/60 mb-2 block">Android Version</label>
          <select v-model="androidVersion" class="w-full bg-black/20 border border-white/10 rounded-lg p-2 text-white text-sm">
            <option value="14">Android 14</option>
            <option value="13">Android 13</option>
            <option value="12">Android 12</option>
            <option value="11">Android 11</option>
            <option value="10">Android 10</option>
          </select>
        </div>
        
        <div>
          <label class="text-xs text-white/60 mb-2 block">Architecture</label>
          <select v-model="arch" class="w-full bg-black/20 border border-white/10 rounded-lg p-2 text-white text-sm">
            <option value="arm64">ARM64</option>
            <option value="arm">ARM</option>
            <option value="x86_64">x86_64</option>
          </select>
        </div>

        <div>
          <label class="text-xs text-white/60 mb-2 block">Variant</label>
          <select v-model="variant" class="w-full bg-black/20 border border-white/10 rounded-lg p-2 text-white text-sm">
            <option value="pico">Pico (Min)</option>
            <option value="nano">Nano (Rec)</option>
            <option value="stock">Stock</option>
            <option value="full">Full</option>
          </select>
        </div>
      </div>

      <VisionButton @click="openDownload" icon="open_in_new" class="w-full">
        Open Download Page
      </VisionButton>
    </div>
  </GlassCard>
</template>
