<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';

interface PermissionEntry {
  package: string;
  permission: string;
  status: string;
}

const entries = ref<PermissionEntry[]>([]);
const loading = ref(false);
const revoking = ref<string | null>(null);

const fetchPermissions = async () => {
  loading.value = true;
  try {
    entries.value = await invoke('audit_permissions');
  } catch (e) {
    console.error(e);
  } finally {
    loading.value = false;
  }
};

const revoke = async (entry: PermissionEntry) => {
  if (!confirm(`Revoke ${entry.permission} from ${entry.package}?`)) return;
  
  revoking.value = entry.package + entry.permission;
  try {
    await invoke('revoke_permission', { package: entry.package, permission: entry.permission });
    // Remove from list locally
    entries.value = entries.value.filter(e => !(e.package === entry.package && e.permission === entry.permission));
  } catch (e) {
    console.error(e);
    alert('Failed to revoke: ' + e);
  } finally {
    revoking.value = null;
  }
};

onMounted(fetchPermissions);
</script>

<template>
  <GlassCard>
    <div class="flex items-center justify-between mb-6">
      <div class="flex items-center gap-3">
        <div class="p-3 rounded-xl bg-primary/20 text-primary">
          <span class="material-symbols-rounded text-2xl">visibility_off</span>
        </div>
        <div>
          <h2 class="text-xl font-bold text-white">Privacy Dashboard</h2>
          <p class="text-sm text-white/60">Audit sensitive app permissions.</p>
        </div>
      </div>
      <VisionButton @click="fetchPermissions" icon="refresh" size="sm" variant="secondary" :loading="loading">Refresh</VisionButton>
    </div>

    <div class="bg-black/20 rounded-xl border border-white/5 h-[300px] overflow-y-auto custom-scrollbar">
      <div v-if="loading && entries.length === 0" class="text-center py-12 text-white/40">
        <span class="material-symbols-rounded animate-spin text-3xl mb-2">refresh</span>
        <p>Scanning permissions...</p>
      </div>
      
      <div v-else-if="entries.length === 0" class="text-center py-12 text-white/40">
        <span class="material-symbols-rounded text-4xl mb-2">check_circle</span>
        <p>No sensitive permissions found (or device offline).</p>
      </div>

      <div v-else class="divide-y divide-white/5">
        <div 
          v-for="(entry, i) in entries" 
          :key="i"
          class="p-3 flex items-center justify-between hover:bg-white/5 transition-colors"
        >
          <div class="overflow-hidden">
            <div class="font-bold text-white text-sm truncate">{{ entry.package }}</div>
            <div class="text-xs text-white/60 flex items-center gap-1">
              <span class="material-symbols-rounded text-[10px]" v-if="entry.permission === 'FINE_LOCATION'">location_on</span>
              <span class="material-symbols-rounded text-[10px]" v-if="entry.permission === 'CAMERA'">photo_camera</span>
              <span class="material-symbols-rounded text-[10px]" v-if="entry.permission === 'RECORD_AUDIO'">mic</span>
              {{ entry.permission }}
            </div>
          </div>
          
          <VisionButton 
            @click="revoke(entry)" 
            size="sm" 
            variant="danger" 
            :loading="revoking === entry.package + entry.permission"
          >
            Revoke
          </VisionButton>
        </div>
      </div>
    </div>
  </GlassCard>
</template>
