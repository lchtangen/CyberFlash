<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';

interface DebloatList {
  id: string;
  name: string;
  description: string;
  packages: string[];
}

interface BatchOpResult {
  item: string;
  success: boolean;
  message: string;
}

const lists = ref<DebloatList[]>([]);
const selectedListId = ref<string>('');
const customPackages = ref<string>('');
const isProcessing = ref(false);
const results = ref<BatchOpResult[]>([]);

onMounted(async () => {
  try {
    lists.value = await invoke('get_debloat_lists');
    if (lists.value.length > 0) {
      selectedListId.value = lists.value[0].id;
    }
  } catch (e) {
    console.error('Failed to load debloat lists', e);
  }
});

const getSelectedPackages = () => {
  const list = lists.value.find(l => l.id === selectedListId.value);
  const listPkgs = list ? list.packages : [];
  const customPkgs = customPackages.value
    .split('\n')
    .map(p => p.trim())
    .filter(p => p.length > 0);
  
  return [...new Set([...listPkgs, ...customPkgs])];
};

const runDebloat = async () => {
  const pkgs = getSelectedPackages();
  if (pkgs.length === 0) return;

  if (!confirm(`Are you sure you want to uninstall ${pkgs.length} packages? This may break system features.`)) return;

  isProcessing.value = true;
  results.value = [];

  try {
    results.value = await invoke('batch_uninstall', { packages: pkgs });
  } catch (e) {
    console.error(e);
  } finally {
    isProcessing.value = false;
  }
};
</script>

<template>
  <GlassCard>
    <div class="flex items-center gap-3 mb-6">
      <div class="p-3 rounded-xl bg-error/20 text-error">
        <span class="material-symbols-rounded text-2xl">delete_forever</span>
      </div>
      <div>
        <h2 class="text-xl font-bold text-white">Bloatware Remover</h2>
        <p class="text-sm text-white/60">Clean up system apps safely.</p>
      </div>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <div class="space-y-4">
        <div>
          <label class="text-xs text-white/60 mb-2 block">Select Preset</label>
          <select v-model="selectedListId" class="w-full bg-black/20 border border-white/10 rounded-xl p-3 text-white outline-none focus:border-primary/50">
            <option v-for="list in lists" :key="list.id" :value="list.id">
              {{ list.name }}
            </option>
          </select>
          <p class="text-xs text-white/40 mt-2" v-if="selectedListId">
            {{ lists.find(l => l.id === selectedListId)?.description }}
          </p>
        </div>

        <div>
          <label class="text-xs text-white/60 mb-2 block">Custom Packages (One per line)</label>
          <textarea 
            v-model="customPackages"
            class="w-full h-32 bg-black/20 border border-white/10 rounded-xl p-3 text-white text-sm font-mono outline-none focus:border-primary/50 resize-none"
            placeholder="com.example.bloatware"
          ></textarea>
        </div>

        <VisionButton 
          @click="runDebloat" 
          :loading="isProcessing" 
          variant="danger" 
          class="w-full"
          icon="cleaning_services"
        >
          Debloat Selected
        </VisionButton>
      </div>

      <div class="bg-black/20 rounded-xl border border-white/5 p-4 h-[300px] overflow-y-auto custom-scrollbar">
        <h3 class="text-sm font-bold text-white mb-3 sticky top-0 bg-transparent">Results</h3>
        <div v-if="results.length === 0" class="text-white/20 text-sm text-center mt-10">
          Results will appear here...
        </div>
        <div v-else class="space-y-2">
          <div v-for="(res, i) in results" :key="i" class="flex items-center justify-between text-xs p-2 rounded bg-white/5">
            <span class="font-mono text-white/80 truncate max-w-[180px]">{{ res.item }}</span>
            <span :class="res.success ? 'text-success' : 'text-error'">{{ res.message }}</span>
          </div>
        </div>
      </div>
    </div>
  </GlassCard>
</template>
