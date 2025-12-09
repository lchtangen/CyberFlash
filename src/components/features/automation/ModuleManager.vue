<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';
import ToggleSwitch from '../../ui/ToggleSwitch.vue';

interface MagiskModule {
  id: string;
  name: string;
  version: string;
  author: string;
  description: string;
  enabled: boolean;
}

const modules = ref<MagiskModule[]>([]);
const loading = ref(false);
const error = ref<string | null>(null);

const fetchModules = async () => {
  loading.value = true;
  error.value = null;
  try {
    modules.value = await invoke('list_magisk_modules');
  } catch (e) {
    error.value = String(e);
  } finally {
    loading.value = false;
  }
};

const toggle = async (mod: MagiskModule) => {
  // Optimistic update
  const oldState = mod.enabled;
  mod.enabled = !mod.enabled;
  
  try {
    await invoke('toggle_module', { id: mod.id, enable: mod.enabled });
  } catch (e) {
    mod.enabled = oldState; // Revert
    console.error(e);
  }
};

const remove = async (mod: MagiskModule) => {
  if (!confirm(`Remove module "${mod.name}"? It will be deleted on next reboot.`)) return;
  try {
    await invoke('remove_module', { id: mod.id });
    alert('Module marked for removal. Please reboot.');
  } catch (e) {
    console.error(e);
  }
};

const install = async () => {
  try {
    const file = await open({
      filters: [{ name: 'Magisk Module', extensions: ['zip'] }]
    });
    
    if (file) {
      const path = Array.isArray(file) ? file[0] : file;
      loading.value = true;
      await invoke('install_module_zip', { zipPath: path });
      alert('Module installed! Please reboot.');
      fetchModules();
    }
  } catch (e) {
    console.error(e);
    alert('Install failed: ' + e);
  } finally {
    loading.value = false;
  }
};

onMounted(fetchModules);
</script>

<template>
  <GlassCard>
    <div class="flex items-center justify-between mb-6">
      <div class="flex items-center gap-3">
        <div class="p-3 rounded-xl bg-white/10 text-white">
          <span class="material-symbols-rounded text-2xl">extension</span>
        </div>
        <div>
          <h2 class="text-xl font-bold text-white">Module Manager</h2>
          <p class="text-sm text-white/60">Manage Magisk/KernelSU modules.</p>
        </div>
      </div>
      <div class="flex gap-2">
        <VisionButton @click="fetchModules" icon="refresh" size="sm" variant="secondary" :loading="loading">Refresh</VisionButton>
        <VisionButton @click="install" icon="add" size="sm">Install</VisionButton>
      </div>
    </div>

    <div v-if="error" class="p-4 rounded-xl bg-error/10 border border-error/20 text-error text-sm mb-4">
      {{ error }}
    </div>

    <div v-if="loading && modules.length === 0" class="text-center py-12 text-white/40">
      <span class="material-symbols-rounded animate-spin text-3xl mb-2">refresh</span>
      <p>Loading modules...</p>
    </div>

    <div v-else-if="modules.length === 0 && !error" class="text-center py-12 text-white/40">
      <span class="material-symbols-rounded text-4xl mb-2">extension_off</span>
      <p>No modules found or root access denied.</p>
    </div>

    <div v-else class="space-y-3 max-h-[400px] overflow-y-auto custom-scrollbar pr-2">
      <div 
        v-for="mod in modules" 
        :key="mod.id"
        class="p-4 rounded-xl bg-white/5 border border-white/10 hover:bg-white/10 transition-colors"
      >
        <div class="flex justify-between items-start mb-2">
          <div>
            <h3 class="font-bold text-white">{{ mod.name }} <span class="text-xs font-normal text-white/40 ml-2">v{{ mod.version }}</span></h3>
            <p class="text-xs text-primary">{{ mod.author }}</p>
          </div>
          <ToggleSwitch :modelValue="mod.enabled" @update:modelValue="toggle(mod)" />
        </div>
        
        <p class="text-sm text-white/70 mb-3">{{ mod.description }}</p>
        
        <div class="flex justify-end">
          <button @click="remove(mod)" class="text-xs text-error hover:text-error-hover flex items-center gap-1">
            <span class="material-symbols-rounded text-sm">delete</span> Remove
          </button>
        </div>
      </div>
    </div>
  </GlassCard>
</template>
