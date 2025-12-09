<script setup lang="ts">
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';
import GlassSelect from '../../ui/GlassSelect.vue';
import GlassInput from '../../ui/GlassInput.vue';

const selectedPartition = ref('userdata');
const newSize = ref<number | string>('');
const isResizing = ref(false);
const message = ref<string | null>(null);

const partitions = [
  { value: 'userdata', label: 'Userdata' },
  { value: 'system', label: 'System' },
  { value: 'vendor', label: 'Vendor' },
  { value: 'product', label: 'Product' }
];

const resizePartition = async () => {
  const size = Number(newSize.value);
  if (!size || size <= 0) {
    message.value = 'Invalid size.';
    return;
  }
  
  isResizing.value = true;
  message.value = null;
  
  try {
    await invoke('resize_partition', { 
      partition: selectedPartition.value, 
      sizeMb: size 
    });
    message.value = `Successfully resized ${selectedPartition.value} to ${size}MB.`;
  } catch (e) {
    message.value = 'Error: ' + e;
  } finally {
    isResizing.value = false;
  }
};

// Visual representation of resize (mock)
const sizePercentage = computed(() => {
  // Just a visual mock, assuming max 128GB for visualization
  const max = 128000; 
  const val = Number(newSize.value) || 0;
  return Math.min((val / max) * 100, 100);
});
</script>

<template>
  <GlassCard>
    <div class="flex items-center gap-3 mb-6">
      <div class="p-3 rounded-xl bg-warning/20 text-warning">
        <span class="material-symbols-rounded text-2xl">pie_chart</span>
      </div>
      <div>
        <h2 class="text-xl font-bold text-white">Partition Resizer</h2>
        <p class="text-sm text-white/60">Modify partition tables safely.</p>
      </div>
    </div>

    <div class="space-y-6">
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <GlassSelect
          v-model="selectedPartition"
          label="Target Partition"
          :options="partitions"
          icon="storage"
        />
        
        <GlassInput
          v-model="newSize"
          label="New Size (MB)"
          type="number"
          placeholder="e.g. 64000"
          icon="straighten"
        />
      </div>

      <!-- Visual Feedback -->
      <div class="p-4 rounded-xl bg-surface/50 border border-white/5">
        <div class="flex justify-between text-xs text-white/60 mb-2">
          <span>0 MB</span>
          <span>128 GB (Max)</span>
        </div>
        <div class="h-4 bg-white/10 rounded-full overflow-hidden relative">
          <div 
            class="absolute top-0 left-0 h-full bg-gradient-to-r from-warning to-error transition-all duration-500"
            :style="{ width: `${sizePercentage}%` }"
          ></div>
        </div>
        <div class="mt-2 text-center text-xs font-mono text-warning">
          Projected: {{ ((Number(newSize) || 0) / 1024).toFixed(2) }} GB
        </div>
      </div>

      <div class="p-3 rounded-lg bg-error/10 border border-error/20 text-error text-xs flex items-start gap-2">
        <span class="material-symbols-rounded text-sm mt-0.5">warning</span>
        <span>
          <b>Warning:</b> Resizing partitions can lead to data loss. 
          Ensure you have a full backup before proceeding.
        </span>
      </div>

      <VisionButton 
        @click="resizePartition" 
        :loading="isResizing" 
        variant="danger"
        icon="aspect_ratio" 
        class="w-full"
      >
        Resize Partition
      </VisionButton>

      <div v-if="message" class="p-3 rounded-lg text-center text-xs font-bold" :class="message.includes('Error') ? 'bg-error/20 text-error' : 'bg-success/20 text-success'">
        {{ message }}
      </div>
    </div>
  </GlassCard>
</template>
