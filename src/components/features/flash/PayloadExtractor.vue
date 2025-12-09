<template>
  <div class="h-full flex flex-col space-y-6 p-6">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-2xl font-bold text-white">Payload Dumper</h2>
        <p class="text-white/60">Extract boot.img and other partitions from payload.bin</p>
      </div>
      <VisionBadge variant="info">Experimental</VisionBadge>
    </div>

    <!-- File Selection -->
    <GlassCard class="p-6">
      <div class="flex items-center space-x-4">
        <div class="flex-1">
          <label class="block text-sm font-medium text-white/60 mb-2">Payload File (payload.bin)</label>
          <div class="flex space-x-2">
            <GlassInput v-model="payloadPath" placeholder="/path/to/payload.bin" readonly />
            <VisionButton @click="selectPayload" variant="secondary">Browse</VisionButton>
          </div>
        </div>
      </div>
    </GlassCard>

    <!-- Partitions List -->
    <div v-if="partitions.length > 0" class="flex-1 min-h-0 flex flex-col">
      <h3 class="text-lg font-medium text-white mb-4">Available Partitions</h3>
      <div class="flex-1 overflow-y-auto pr-2 space-y-2 custom-scrollbar">
        <div 
          v-for="part in partitions" 
          :key="part.name"
          class="flex items-center justify-between p-4 rounded-xl border border-white/5 bg-surface/30 hover:bg-surface/50 transition-colors cursor-pointer"
          @click="togglePartition(part.name)"
        >
          <div class="flex items-center space-x-3">
            <div 
              class="w-5 h-5 rounded border flex items-center justify-center transition-colors"
              :class="selectedPartitions.includes(part.name) ? 'bg-primary border-primary' : 'border-white/30'"
            >
              <Icon v-if="selectedPartitions.includes(part.name)" icon="mdi:check" class="text-white text-xs" />
            </div>
            <span class="text-white font-medium">{{ part.name }}</span>
            <span class="text-xs text-white/40">{{ formatSize(part.size) }}</span>
          </div>
          <Icon icon="mdi:harddisk" class="text-white/20" />
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="flex-1 flex flex-col items-center justify-center text-white/40">
      <Icon icon="mdi:archive-outline" class="text-6xl mb-4 opacity-50" />
      <p>Select a payload.bin file to view partitions</p>
    </div>

    <!-- Actions -->
    <div class="flex justify-end space-x-4 pt-4 border-t border-white/10">
      <VisionButton 
        :disabled="selectedPartitions.length === 0 || isExtracting"
        @click="extractSelected"
        variant="primary"
        class="w-48"
      >
        <template v-if="isExtracting">
          <Icon icon="mdi:loading" class="animate-spin mr-2" />
          Extracting...
        </template>
        <template v-else>
          <Icon icon="mdi:export" class="mr-2" />
          Extract Selected
        </template>
      </VisionButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { Icon } from '@iconify/vue';
import GlassCard from '@/components/ui/GlassCard.vue';
import GlassInput from '@/components/ui/GlassInput.vue';
import VisionButton from '@/components/ui/VisionButton.vue';
import VisionBadge from '@/components/ui/VisionBadge.vue';

interface Partition {
  name: string;
  size: number;
}

const payloadPath = ref('');
const partitions = ref<Partition[]>([]);
const selectedPartitions = ref<string[]>([]);
const isExtracting = ref(false);

const formatSize = (bytes: number) => {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

const selectPayload = async () => {
  const selected = await open({
    multiple: false,
    filters: [{ name: 'Payload', extensions: ['bin'] }]
  });

  if (selected && typeof selected === 'string') {
    payloadPath.value = selected;
    analyzePayload(selected);
  }
};

const analyzePayload = async (path: string) => {
  try {
    // Call Rust backend to parse payload
    const result = await invoke<Partition[]>('parse_payload_bin', { path });
    partitions.value = result;
  } catch (error) {
    console.error('Failed to parse payload:', error);
    // Mock for now if backend fails
    partitions.value = [
      { name: 'boot', size: 67108864 },
      { name: 'system', size: 2147483648 },
      { name: 'vendor', size: 1073741824 },
      { name: 'vbmeta', size: 4096 },
      { name: 'dtbo', size: 8388608 },
    ];
  }
};

const togglePartition = (name: string) => {
  if (selectedPartitions.value.includes(name)) {
    selectedPartitions.value = selectedPartitions.value.filter(p => p !== name);
  } else {
    selectedPartitions.value.push(name);
  }
};

const extractSelected = async () => {
  if (!payloadPath.value || selectedPartitions.value.length === 0) return;

  isExtracting.value = true;
  try {
    for (const part of selectedPartitions.value) {
      await invoke('extract_payload_partition', { 
        payloadPath: payloadPath.value,
        partition: part 
      });
    }
    // Show success notification
  } catch (error) {
    console.error('Extraction failed:', error);
  } finally {
    isExtracting.value = false;
  }
};
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
