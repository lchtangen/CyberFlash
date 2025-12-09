<script setup lang="ts">
import { ref } from 'vue';
import { useNotificationStore } from '../../../stores/notifications';

const notificationStore = useNotificationStore();
const isVisible = ref(false);
const analysis = ref('');

// Simulate AI monitoring
// In a real app, this would listen to error events and trigger an AI analysis
/*
const checkErrors = () => {
  const lastLog = flashStore.logs[flashStore.logs.length - 1];
  if (lastLog && lastLog.toLowerCase().includes('error')) {
    isVisible.value = true;
    analysis.value = "It looks like the flashing process failed. This usually happens if the bootloader is not unlocked. Please verify your unlock status.";
  }
};
*/

// Watch for log changes (simplified for this demo)
// watch(() => flashStore.logs.length, checkErrors);

const dismiss = () => {
  isVisible.value = false;
};

const fixIt = () => {
  // Logic to apply fix
  notificationStore.addNotification({
    title: 'AI Fix Applied',
    message: 'Corrective actions have been queued.',
    type: 'success'
  });
  isVisible.value = false;
};
</script>

<template>
  <div v-if="isVisible" class="fixed bottom-6 left-6 z-50 max-w-md animate-slide-up">
    <div class="bg-surface/90 backdrop-blur-xl border border-primary/50 rounded-2xl p-4 shadow-[0_0_30px_rgba(10,132,255,0.2)]">
      <div class="flex items-start gap-4">
        <div class="w-10 h-10 rounded-full bg-primary/20 flex items-center justify-center shrink-0">
          <span class="material-symbols-rounded text-primary">smart_toy</span>
        </div>
        <div class="flex-1">
          <h4 class="font-bold text-white mb-1">AI Guardian</h4>
          <p class="text-sm text-gray-300 leading-relaxed">{{ analysis }}</p>
          <div class="mt-3 flex gap-2">
            <button @click="fixIt" class="text-xs bg-primary/20 text-primary px-3 py-1.5 rounded-lg hover:bg-primary/30 transition-colors">
              Fix it for me
            </button>
            <button @click="dismiss" class="text-xs text-gray-500 hover:text-white px-3 py-1.5 transition-colors">
              Dismiss
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.animate-slide-up {
  animation: slideUp 0.4s cubic-bezier(0.16, 1, 0.3, 1);
}
@keyframes slideUp {
  from { transform: translateY(50px); opacity: 0; }
  to { transform: translateY(0); opacity: 1; }
}
</style>
