<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { useAIStore } from '../../../stores/ai';
import { useNotificationStore } from '../../../stores/notifications';

const isOpen = ref(false);
const searchQuery = ref('');
const aiStore = useAIStore();
const notificationStore = useNotificationStore();

const commands = [
  { id: 'flash', label: 'Flash ROM', icon: 'system_update', shortcut: '⌘F', action: () => console.log('Flash') },
  { id: 'wipe', label: 'Wipe Data', icon: 'delete_forever', shortcut: '⌘W', action: () => console.log('Wipe') },
  { id: 'reboot', label: 'Reboot Device', icon: 'restart_alt', shortcut: '⌘R', action: () => console.log('Reboot') },
  { id: 'settings', label: 'Open Settings', icon: 'settings', shortcut: '⌘,', action: () => console.log('Settings') },
  { id: 'scan', label: 'Scan Devices', icon: 'devices', shortcut: '⌘D', action: () => console.log('Scan') },
  { id: 'logs', label: 'Analyze Logs', icon: 'analytics', shortcut: '⌘L', action: () => console.log('Logs') },
];

const filteredCommands = computed(() => {
  const query = searchQuery.value.toLowerCase();
  return commands.filter(cmd => 
    cmd.label.toLowerCase().includes(query) || 
    cmd.id.toLowerCase().includes(query)
  );
});

const handleKeydown = (e: KeyboardEvent) => {
  if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
    e.preventDefault();
    isOpen.value = !isOpen.value;
  }
};

const executeCommand = (cmd: any) => {
  cmd.action();
  notificationStore.addNotification({
    title: 'Command Executed',
    message: `Running: ${cmd.label}`,
    type: 'info',
    duration: 2000
  });
  isOpen.value = false;
  searchQuery.value = '';
};

const askAI = () => {
  aiStore.isVisible = true;
  aiStore.addMessage('user', searchQuery.value);
  // Trigger AI processing here
  isOpen.value = false;
  searchQuery.value = '';
};

onMounted(() => document.addEventListener('keydown', handleKeydown));
onUnmounted(() => document.removeEventListener('keydown', handleKeydown));
</script>
<template>
  <div v-if="isOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <!-- Backdrop -->
    <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="isOpen = false"></div>
    
    <!-- Modal Content -->
    <div class="relative w-full max-w-lg bg-surface/90 border border-white/10 rounded-xl shadow-2xl overflow-hidden backdrop-blur-xl">
      <div class="p-4 border-b border-white/10 flex items-center gap-2">
        <span class="material-symbols-rounded text-white/50">search</span>
        <input 
          v-model="searchQuery" 
          placeholder="Type a command or ask AI..." 
          class="flex-1 bg-transparent text-white placeholder-white/30 focus:outline-none text-lg"
          autofocus
        />
        <button @click="isOpen = false" class="text-white/30 hover:text-white transition-colors">
          <span class="material-symbols-rounded">close</span>
        </button>
      </div>
      
      <div class="p-2 space-y-1 max-h-[300px] overflow-y-auto custom-scrollbar">
        <!-- Commands -->
        <button
          v-for="cmd in filteredCommands"
          :key="cmd.id"
          @click="executeCommand(cmd)"
          class="w-full flex items-center justify-between p-3 rounded-lg hover:bg-white/10 transition-colors group text-left focus:bg-white/10 outline-none"
        >
          <div class="flex items-center gap-3">
            <span class="material-symbols-rounded text-gray-400 group-hover:text-white">{{ cmd.icon }}</span>
            <span class="text-white">{{ cmd.label }}</span>
          </div>
          <span class="text-xs text-gray-400 bg-white/5 px-2 py-1 rounded">{{ cmd.shortcut }}</span>
        </button>

        <!-- AI Fallback -->
        <button
          v-if="searchQuery && filteredCommands.length === 0"
          @click="askAI"
          class="w-full flex items-center gap-3 p-3 rounded-lg hover:bg-blue-500/20 transition-colors group text-left border border-dashed border-white/10 hover:border-blue-500/50"
        >
          <div class="w-8 h-8 rounded-full bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center">
            <span class="material-symbols-rounded text-white text-sm">auto_awesome</span>
          </div>
          <div>
            <span class="text-white block text-sm">Ask AI Assistant</span>
            <span class="text-xs text-gray-400">"{{ searchQuery }}"</span>
          </div>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 2px;
}
</style>
