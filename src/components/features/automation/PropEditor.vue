<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';
import GlassInput from '../../ui/GlassInput.vue';

const props = ref<Record<string, string>>({});
const searchQuery = ref('');
const editedProps = ref<Record<string, string>>({});
const loading = ref(false);

const fetchProps = async () => {
  loading.value = true;
  try {
    props.value = await invoke('get_all_props');
  } catch (e) {
    console.error(e);
  } finally {
    loading.value = false;
  }
};

const filteredProps = computed(() => {
  const query = searchQuery.value.toLowerCase();
  return Object.entries(props.value)
    .filter(([k, v]) => k.toLowerCase().includes(query) || v.toLowerCase().includes(query))
    .sort(([a], [b]) => a.localeCompare(b));
});

const updateProp = (key: string, value: string) => {
  editedProps.value[key] = value;
};

const exportModule = async () => {
  if (Object.keys(editedProps.value).length === 0) return;
  
  try {
    const path = await save({
      filters: [{ name: 'Magisk Module', extensions: ['zip'] }],
      defaultPath: 'cyberflash_props.zip'
    });
    
    if (path) {
      await invoke('generate_prop_module', { 
        props: editedProps.value, 
        savePath: path 
      });
      alert('Module saved! Flash it in Magisk to apply changes.');
    }
  } catch (e) {
    console.error(e);
  }
};

onMounted(fetchProps);
</script>

<template>
  <GlassCard>
    <div class="flex items-center justify-between mb-6">
      <div class="flex items-center gap-3">
        <div class="p-3 rounded-xl bg-warning/20 text-warning">
          <span class="material-symbols-rounded text-2xl">settings_system_daydream</span>
        </div>
        <div>
          <h2 class="text-xl font-bold text-white">Prop Editor</h2>
          <p class="text-sm text-white/60">Systemless build.prop editor.</p>
        </div>
      </div>
      <div class="flex gap-2">
        <VisionButton @click="fetchProps" icon="refresh" size="sm" variant="secondary" :loading="loading">Refresh</VisionButton>
        <VisionButton 
          @click="exportModule" 
          icon="save" 
          size="sm" 
          :disabled="Object.keys(editedProps).length === 0"
        >
          Export Module ({{ Object.keys(editedProps).length }})
        </VisionButton>
      </div>
    </div>

    <div class="mb-4">
      <GlassInput v-model="searchQuery" placeholder="Search properties..." icon="search" />
    </div>

    <div class="bg-black/20 rounded-xl border border-white/5 h-[400px] overflow-y-auto custom-scrollbar">
      <table class="w-full text-left border-collapse">
        <thead class="sticky top-0 bg-surface/90 backdrop-blur-md z-10">
          <tr>
            <th class="p-3 text-xs font-bold text-white/60 border-b border-white/10">Property</th>
            <th class="p-3 text-xs font-bold text-white/60 border-b border-white/10">Value</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="[key, val] in filteredProps" :key="key" class="hover:bg-white/5 group">
            <td class="p-2 text-xs font-mono text-primary border-b border-white/5 w-1/2 break-all">
              {{ key }}
            </td>
            <td class="p-2 border-b border-white/5">
              <input 
                type="text" 
                :value="editedProps[key] !== undefined ? editedProps[key] : val"
                @input="(e) => updateProp(key, (e.target as HTMLInputElement).value)"
                class="w-full bg-transparent text-xs text-white font-mono outline-none focus:text-warning"
                :class="{'text-warning font-bold': editedProps[key] !== undefined}"
              />
            </td>
          </tr>
        </tbody>
      </table>
      
      <div v-if="filteredProps.length === 0" class="text-center py-12 text-white/20">
        No properties found.
      </div>
    </div>
  </GlassCard>
</template>
