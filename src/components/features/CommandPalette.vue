<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { useAIStore } from '../../stores/ai';
import { aiService } from '../../services/aiService';

const isOpen = ref(false);
const searchQuery = ref('');
const aiStore = useAIStore();

const commands = [
  { id: 'flash', label: 'Flash ROM', icon: 'system_update', shortcut: '⌘F', action: () => console.log('Flash') },
  { id: 'wipe', label: 'Wipe Data', icon: 'delete_forever', shortcut: '⌘W', action: () => console.log('Wipe') },
  { id: 'reboot', label: 'Reboot Device', icon: 'restart_alt', shortcut: '⌘R', action: () => console.log('Reboot') },
  { id: 'settings', label: 'Open Settings', icon: 'settings', shortcut: '⌘,', action: () => aiService.executeAction('nav_settings') },
  { id: 'scan', label: 'Scan Devices', icon: 'devices', shortcut: '⌘D', action: () => aiService.executeAction('adb_scan') },
  { id: 'logs', label: 'Analyze Logs', icon: 'analytics', shortcut: '⌘L', action: () => aiService.executeAction('analyze_logcat') },
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
  isOpen.value = false;
  searchQuery.value = '';
};

const askAI = () => {
  aiStore.isVisible = true;
  aiStore.addMessage('user', searchQuery.value);
  aiService.processUserMessage(searchQuery.value);
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
    <div class="relative w-full max-w-lg bg-[#1c1c1e] border border-white/10 rounded-xl shadow-2xl overflow-hidden">
      <div class="p-4 border-b border-white/10">
        <input 
          v-model="searchQuery" 
          placeholder="Type a command or ask AI..." 
          class="w-full bg-transparent text-white placeholder-white/30 focus:outline-none text-lg"
          autofocus
        />
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
            <span class="material-icons text-gray-400 group-hover:text-white">{{ cmd.icon }}</span>
            <span class="text-white">{{ cmd.label }}</span>
          </div>
          <span class="text-xs text-gray-400 bg-white/5 px-2 py-1 rounded">{{ cmd.shortcut }}</span>
        </button>

        <!-- AI Fallback -->
        <button
          v-if="searchQuery && filteredCommands.length === 0"
          @click="askAI"
          class="w-full flex items-center gap-3 p-3 rounded-lg hover:bg-primary/20 transition-colors group text-left border border-dashed border-white/10 hover:border-primary/50"
        >
          <div class="w-8 h-8 rounded-full bg-gradient-to-br from-primary to-secondary flex items-center justify-center">
            <span class="material-icons text-white text-sm">auto_awesome</span>
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
