<script setup lang="ts">
import { ref, onMounted } from 'vue';
// import { check } from '@tauri-apps/plugin-updater'; 
import { useNotificationStore } from '../../stores/notifications';

const updateAvailable = ref(false);
const version = ref('v2.0.0');
const notificationStore = useNotificationStore();

const checkForUpdates = async () => {
  // Simulation logic for now
  // In production:
  // const update = await check();
  // if (update?.available) { ... }
  
  // Simulating no update for now to avoid annoyance, or maybe a silent check
  // If update found:
  // updateAvailable.value = true;
  // notificationStore.addNotification({
  //   title: 'Update Available',
  //   message: `Version ${version.value} is ready to install.`,
  //   type: 'info'
  // });
};

onMounted(() => {
  checkForUpdates();
});
</script>

<template>
  <div v-if="updateAvailable" class="fixed top-6 right-6 z-50 bg-surface border border-primary/50 rounded-xl p-4 shadow-2xl flex items-center gap-4 animate-slide-down">
    <div class="w-10 h-10 rounded-full bg-primary/20 flex items-center justify-center text-primary">
      <span class="material-symbols-rounded">system_update</span>
    </div>
    <div>
      <h4 class="font-bold text-white">Update Available</h4>
      <p class="text-xs text-gray-400">Version {{ version }} is ready.</p>
    </div>
    <button class="px-3 py-1.5 bg-primary text-white text-xs font-bold rounded-lg hover:bg-primary-hover">
      Update
    </button>
    <button @click="updateAvailable = false" class="text-gray-500 hover:text-white">
      <span class="material-symbols-rounded text-sm">close</span>
    </button>
  </div>
</template>

<style scoped>
.animate-slide-down {
  animation: slideDown 0.3s ease-out;
}
@keyframes slideDown {
  from { transform: translateY(-20px); opacity: 0; }
  to { transform: translateY(0); opacity: 1; }
}
</style>
