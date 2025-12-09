<template>
  <div class="h-full flex flex-col space-y-6 p-6">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-2xl font-bold text-white">Module Manager</h2>
        <p class="text-white/60">Manage Magisk/KernelSU modules and DenyList</p>
      </div>
      <div class="flex space-x-2">
        <VisionButton 
          @click="activeTab = 'modules'" 
          :variant="activeTab === 'modules' ? 'primary' : 'secondary'"
          size="sm"
        >
          Modules
        </VisionButton>
        <VisionButton 
          @click="activeTab = 'denylist'" 
          :variant="activeTab === 'denylist' ? 'primary' : 'secondary'"
          size="sm"
        >
          DenyList
        </VisionButton>
      </div>
    </div>

    <!-- Modules Tab -->
    <div v-if="activeTab === 'modules'" class="flex-1 flex flex-col min-h-0">
      <div class="flex justify-end mb-4">
        <VisionButton @click="installModule" variant="primary">
          <Icon icon="mdi:plus" class="mr-2" />
          Install from Zip
        </VisionButton>
      </div>

      <div v-if="loading" class="flex-1 flex items-center justify-center">
        <Icon icon="mdi:loading" class="animate-spin text-4xl text-primary" />
      </div>

      <div v-else-if="error" class="p-4 rounded-xl bg-error/10 border border-error/20 text-error">
        {{ error }}
        <VisionButton @click="fetchModules" variant="ghost" class="ml-2 underline">Retry</VisionButton>
      </div>

      <div v-else class="flex-1 overflow-y-auto space-y-4 custom-scrollbar pr-2">
        <GlassCard 
          v-for="mod in modules" 
          :key="mod.id"
          class="p-4 transition-all duration-300 hover:bg-surface/40"
        >
          <div class="flex items-start justify-between">
            <div>
              <h3 class="text-lg font-bold text-white">{{ mod.name }} <span class="text-xs text-white/40 ml-2">{{ mod.version }}</span></h3>
              <p class="text-sm text-primary mb-1">by {{ mod.author }}</p>
              <p class="text-sm text-white/70">{{ mod.description }}</p>
            </div>
            <div class="flex items-center space-x-4">
              <ToggleSwitch 
                v-model="mod.enabled" 
                @update:modelValue="toggle(mod)"
              />
              <button @click="remove(mod)" class="text-white/40 hover:text-error transition-colors">
                <Icon icon="mdi:delete" />
              </button>
            </div>
          </div>
        </GlassCard>
      </div>
    </div>

    <!-- DenyList Tab -->
    <div v-else class="flex-1 flex flex-col min-h-0">
      <div class="mb-4">
        <GlassInput v-model="denySearch" placeholder="Search packages..." icon="search" />
      </div>
      
      <div class="flex-1 overflow-y-auto space-y-2 custom-scrollbar pr-2">
        <div 
          v-for="pkg in filteredPackages" 
          :key="pkg"
          class="flex items-center justify-between p-3 rounded-xl border border-white/5 bg-surface/30"
        >
          <span class="text-white font-mono text-sm">{{ pkg }}</span>
          <ToggleSwitch 
            :modelValue="denyList.includes(pkg)"
            @update:modelValue="(val) => toggleDeny(pkg, val)"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { Icon } from '@iconify/vue';
import GlassCard from '../../ui/GlassCard.vue';
import GlassInput from '../../ui/GlassInput.vue';
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

const activeTab = ref('modules');
const modules = ref<MagiskModule[]>([]);
const denyList = ref<string[]>([]);
const allPackages = ref<string[]>([]);
const denySearch = ref('');
const loading = ref(false);
const error = ref<string | null>(null);

const filteredPackages = computed(() => {
  if (!denySearch.value) return allPackages.value;
  return allPackages.value.filter(p => p.toLowerCase().includes(denySearch.value.toLowerCase()));
});

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

const fetchDenyList = async () => {
  try {
    const [denied, packages] = await Promise.all([
      invoke<string[]>('get_denylist'),
      invoke<string[]>('list_packages')
    ]);
    denyList.value = denied;
    allPackages.value = packages.sort();
  } catch (e) {
    console.error(e);
  }
};

const toggle = async (mod: MagiskModule) => {
  const oldState = mod.enabled;
  // mod.enabled is already updated by v-model
  try {
    await invoke('toggle_module', { id: mod.id, enable: mod.enabled });
  } catch (e) {
    mod.enabled = oldState;
    console.error(e);
  }
};

const remove = async (mod: MagiskModule) => {
  const confirmed = window.confirm(`Remove module "${mod.name}"? It will be deleted on next reboot.`);
  if (!confirmed) return;
  try {
    await invoke('remove_module', { id: mod.id });
    modules.value = modules.value.filter(m => m.id !== mod.id);
  } catch (e) {
    error.value = `Failed to remove module: ${String(e)}`;
  }
};

const installModule = async () => {
  const selected = await open({
    multiple: false,
    filters: [{ name: 'Zip', extensions: ['zip'] }]
  });

  if (selected && typeof selected === 'string') {
    loading.value = true;
    try {
      await invoke('install_module_zip', { zipPath: selected });
      error.value = null;
      await fetchModules();
    } catch (e) {
      error.value = `Failed to install module: ${String(e)}`;
    } finally {
      loading.value = false;
    }
  }
};

const toggleDeny = async (pkg: string, enable: boolean) => {
  try {
    await invoke('toggle_denylist', { package: pkg, enable });
    if (enable) {
      denyList.value.push(pkg);
    } else {
      denyList.value = denyList.value.filter(p => p !== pkg);
    }
  } catch (e) {
    console.error(e);
  }
};

onMounted(() => {
  fetchModules();
  fetchDenyList();
});
</script>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.05);
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.2);
  border-radius: 2px;
}
</style>
