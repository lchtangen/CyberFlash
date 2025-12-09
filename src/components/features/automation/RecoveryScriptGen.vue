<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';
import GlassInput from '../../ui/GlassInput.vue';

interface OrsAction {
  type: 'Install' | 'Wipe' | 'Backup' | 'Restore' | 'Mount' | 'Unmount' | 'Shell' | 'Print';
  payload: any;
  id: string; // for UI key
}

const actions = ref<OrsAction[]>([]);
const generatedScript = ref('');

const addAction = (type: OrsAction['type']) => {
  const id = Math.random().toString(36).substring(7);
  let payload = {};
  
  switch (type) {
    case 'Install': payload = { path: '/sdcard/' }; break;
    case 'Wipe': payload = { partition: 'data' }; break;
    case 'Backup': payload = { partitions: ['S', 'D', 'B'], name: 'Backup_001' }; break;
    case 'Restore': payload = { path: '/sdcard/TWRP/BACKUPS/...' }; break;
    case 'Mount': payload = { partition: 'system' }; break;
    case 'Unmount': payload = { partition: 'system' }; break;
    case 'Shell': payload = { command: 'ls -la' }; break;
    case 'Print': payload = { message: 'Operation Complete' }; break;
  }
  
  actions.value.push({ type, payload, id });
  updatePreview();
};

const removeAction = (index: number) => {
  actions.value.splice(index, 1);
  updatePreview();
};

const updatePreview = async () => {
  // Strip IDs before sending to backend
  const cleanActions = actions.value.map(({ id, ...rest }) => rest);
  try {
    generatedScript.value = await invoke('generate_ors_script', { actions: cleanActions });
  } catch (e) {
    console.error(e);
  }
};

const saveScript = async () => {
  try {
    const path = await save({
      filters: [{
        name: 'OpenRecoveryScript',
        extensions: ['ors', 'txt']
      }]
    });
    
    if (path) {
      await invoke('save_ors_script', { path, content: generatedScript.value });
      // Show success notification (mock)
      alert('Script saved successfully!');
    }
  } catch (e) {
    console.error(e);
  }
};

// Helper for Backup partitions
const togglePartition = (action: OrsAction, part: string) => {
  const list = action.payload.partitions as string[];
  if (list.includes(part)) {
    action.payload.partitions = list.filter(p => p !== part);
  } else {
    list.push(part);
  }
  updatePreview();
};
</script>

<template>
  <GlassCard>
    <div class="flex items-center gap-3 mb-6">
      <div class="p-3 rounded-xl bg-primary/20 text-primary">
        <span class="material-symbols-rounded text-2xl">description</span>
      </div>
      <div>
        <h2 class="text-xl font-bold text-white">Recovery Script Gen</h2>
        <p class="text-sm text-white/60">Automate TWRP with OpenRecoveryScript.</p>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- Builder Column -->
      <div class="space-y-4">
        <div class="flex flex-wrap gap-2 mb-4">
          <VisionButton size="sm" icon="add" @click="addAction('Install')">Install</VisionButton>
          <VisionButton size="sm" icon="delete_sweep" @click="addAction('Wipe')">Wipe</VisionButton>
          <VisionButton size="sm" icon="save" @click="addAction('Backup')">Backup</VisionButton>
          <VisionButton size="sm" icon="terminal" @click="addAction('Shell')">Shell</VisionButton>
          <VisionButton size="sm" icon="campaign" @click="addAction('Print')">Print</VisionButton>
        </div>

        <div class="space-y-2 max-h-[400px] overflow-y-auto custom-scrollbar pr-2">
          <div 
            v-for="(action, index) in actions" 
            :key="action.id"
            class="p-3 rounded-lg bg-white/5 border border-white/10 flex flex-col gap-2"
          >
            <div class="flex justify-between items-center">
              <span class="font-bold text-sm text-primary">{{ action.type }}</span>
              <button @click="removeAction(index)" class="text-white/40 hover:text-error">
                <span class="material-symbols-rounded text-sm">close</span>
              </button>
            </div>

            <!-- Dynamic Inputs based on Type -->
            <div v-if="action.type === 'Install'">
              <GlassInput v-model="action.payload.path" placeholder="/sdcard/rom.zip" @input="updatePreview" />
            </div>

            <div v-if="action.type === 'Wipe'">
              <select v-model="action.payload.partition" @change="updatePreview" class="w-full bg-black/20 border border-white/10 rounded px-2 py-1 text-white text-sm">
                <option value="data">Data</option>
                <option value="cache">Cache</option>
                <option value="dalvik">Dalvik</option>
                <option value="system">System</option>
              </select>
            </div>

            <div v-if="action.type === 'Backup'">
              <div class="flex gap-2 mb-2">
                <button 
                  v-for="p in ['S', 'D', 'B', 'R']" 
                  :key="p"
                  @click="togglePartition(action, p)"
                  class="w-8 h-8 rounded flex items-center justify-center text-xs font-bold border transition-colors"
                  :class="action.payload.partitions.includes(p) ? 'bg-primary border-primary text-white' : 'bg-white/5 border-white/10 text-white/40'"
                >
                  {{ p }}
                </button>
              </div>
              <GlassInput v-model="action.payload.name" placeholder="Backup Name" @input="updatePreview" />
            </div>

            <div v-if="action.type === 'Shell'">
              <GlassInput v-model="action.payload.command" placeholder="rm -rf /sdcard/..." @input="updatePreview" />
            </div>
            
            <div v-if="action.type === 'Print'">
              <GlassInput v-model="action.payload.message" placeholder="Message..." @input="updatePreview" />
            </div>
          </div>
          
          <div v-if="actions.length === 0" class="text-center py-8 text-white/20 text-sm italic">
            No actions added. Click buttons above to start.
          </div>
        </div>
      </div>

      <!-- Preview Column -->
      <div class="flex flex-col h-full">
        <div class="flex-1 bg-black/40 rounded-xl border border-white/10 p-4 font-mono text-xs text-white/80 overflow-auto whitespace-pre mb-4">
          {{ generatedScript || '# Generated script will appear here' }}
        </div>
        <VisionButton icon="save_alt" variant="primary" @click="saveScript" :disabled="!generatedScript">
          Save Script (.ors)
        </VisionButton>
      </div>
    </div>
  </GlassCard>
</template>
