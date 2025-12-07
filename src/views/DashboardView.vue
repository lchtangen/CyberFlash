<script setup lang="ts">
import DeviceStatusPanel from '../components/features/DeviceStatusPanel.vue';
import AIChatInterface from '../components/features/AIChatInterface.vue';
import LogStreamViewer from '../components/features/LogStreamViewer.vue';
import { ref } from 'vue';

const logs = ref([]);
</script>

<template>
  <div class="p-6 space-y-6 h-full overflow-y-auto">
    <header class="flex justify-between items-center">
      <div>
        <h2 class="text-2xl font-bold text-white">Dashboard</h2>
        <p class="text-text-secondary">Device Overview & Status</p>
      </div>
      <div class="flex gap-3">
        <button class="px-4 py-2 bg-white/5 hover:bg-white/10 border border-white/10 rounded-lg text-white transition-colors flex items-center gap-2">
          <span class="material-icons text-sm">refresh</span>
          Refresh
        </button>
      </div>
    </header>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- Left Column -->
      <div class="lg:col-span-2 space-y-6">
        <DeviceStatusPanel />
        
        <div class="bg-white/5 border border-white/10 rounded-xl p-4 backdrop-blur-md">
          <h3 class="text-lg font-semibold text-white mb-4 flex items-center gap-2">
            <span class="material-icons text-primary">terminal</span>
            System Logs
          </h3>
          <LogStreamViewer :logs="logs" />
        </div>
      </div>

      <!-- Right Column -->
      <div class="space-y-6">
        <AIChatInterface />
        
        <div class="bg-white/5 border border-white/10 rounded-xl p-4 backdrop-blur-md">
          <h3 class="text-lg font-semibold text-white mb-4">Quick Actions</h3>
          <div class="grid grid-cols-2 gap-3">
            <button class="p-3 bg-white/5 hover:bg-primary/20 border border-white/10 hover:border-primary/50 rounded-lg transition-all group">
              <span class="material-icons text-2xl text-gray-400 group-hover:text-primary mb-2">adb</span>
              <span class="block text-sm text-gray-300 group-hover:text-white">ADB Shell</span>
            </button>
            <button class="p-3 bg-white/5 hover:bg-primary/20 border border-white/10 hover:border-primary/50 rounded-lg transition-all group">
              <span class="material-icons text-2xl text-gray-400 group-hover:text-primary mb-2">folder_open</span>
              <span class="block text-sm text-gray-300 group-hover:text-white">File Manager</span>
            </button>
            <button class="p-3 bg-white/5 hover:bg-primary/20 border border-white/10 hover:border-primary/50 rounded-lg transition-all group">
              <span class="material-icons text-2xl text-gray-400 group-hover:text-primary mb-2">system_update</span>
              <span class="block text-sm text-gray-300 group-hover:text-white">Fastboot</span>
            </button>
            <button class="p-3 bg-white/5 hover:bg-primary/20 border border-white/10 hover:border-primary/50 rounded-lg transition-all group">
              <span class="material-icons text-2xl text-gray-400 group-hover:text-primary mb-2">settings_backup_restore</span>
              <span class="block text-sm text-gray-300 group-hover:text-white">Recovery</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
