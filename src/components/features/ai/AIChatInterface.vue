<script setup lang="ts">
import { ref, nextTick, watch, computed } from 'vue';
import { useAIStore } from '../../../stores/ai';
import { useSettingsStore } from '../../../stores/settings';
import { useDeviceStore } from '../../../stores/device';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';

defineProps<{
  heightClass?: string;
}>();

defineEmits<{
  (e: 'close'): void;
}>();

const aiStore = useAIStore();
const settingsStore = useSettingsStore();
const deviceStore = useDeviceStore();

const modelName = computed(() => {
  if (settingsStore.aiModel === 'gemini-1.5-flash') return 'Gemini 1.5 Flash';
  if (settingsStore.aiModel === 'gemini-1.5-pro') return 'Gemini 1.5 Pro';
  return settingsStore.aiModel;
});

const userInput = ref('');
const chatContainer = ref<HTMLElement | null>(null);

const scrollToBottom = async () => {
  await nextTick();
  if (chatContainer.value) {
    chatContainer.value.scrollTop = chatContainer.value.scrollHeight;
  }
};

watch(() => aiStore.messages.length, scrollToBottom);

const getContext = () => {
  if (!deviceStore.isConnected) return "Status: Device Disconnected. Guide user to connect via USB and enable Debugging.";
  return `
    Status: Connected
    Model: ${deviceStore.deviceModel}
    Serial: ${deviceStore.serial}
    Battery: ${deviceStore.batteryLevel}%
    Connection Mode: ${deviceStore.connectionType}
    Bootloader: ${deviceStore.isBootloaderUnlocked ? 'Unlocked' : 'Locked'}
  `;
};

const sendMessage = async () => {
  if (!userInput.value.trim()) return;
  
  const userMsg = userInput.value;
  aiStore.addMessage('user', userMsg);
  userInput.value = '';
  aiStore.isThinking = true;

  try {
    const response = await invoke<string>('ask_gemini', { 
      prompt: userMsg,
      apiKey: settingsStore.effectiveGeminiApiKey,
      model: settingsStore.aiModel,
      context: getContext()
    });
    aiStore.addMessage('ai', response);
  } catch (e) {
    aiStore.addMessage('ai', `Error: ${e}`);
  } finally {
    aiStore.isThinking = false;
  }
};

const clearChat = () => {
  aiStore.messages = [];
};

const suggestions = [
  "How do I unlock bootloader?",
  "My device is bootlooping",
  "Flash recovery partition",
  "Check battery health"
];

const useSuggestion = (text: string) => {
  userInput.value = text;
  sendMessage();
};

// Simple formatter for code blocks (basic)
const formatMessage = (text: string) => {
  // This is a very basic formatter. For production, use markdown-it.
  // It wraps text in backticks with a code style.
  return text.replace(/`([^`]+)`/g, '<code class="bg-black/30 px-1.5 py-0.5 rounded text-primary font-mono text-xs">$1</code>')
             .replace(/\n/g, '<br>');
};
</script>

<template>
  <GlassCard 
    noPadding
    class="flex flex-col w-full transition-all duration-300 overflow-hidden border-white/10 shadow-2xl shadow-black/50"
    :class="heightClass || 'h-[600px]'"
  >
    <!-- Header -->
    <div class="px-5 py-4 border-b border-white/5 flex items-center justify-between bg-white/5 backdrop-blur-md z-10">
      <div class="flex items-center gap-4">
        <div class="relative">
          <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-primary/20 to-primary/5 flex items-center justify-center border border-white/10 shadow-inner">
            <span class="material-symbols-rounded text-primary text-xl drop-shadow-glow">smart_toy</span>
          </div>
          <div class="absolute -bottom-1 -right-1 w-3 h-3 rounded-full bg-surface border-2 border-surface flex items-center justify-center">
            <div class="w-1.5 h-1.5 rounded-full bg-success animate-pulse"></div>
          </div>
        </div>
        <div>
          <h3 class="font-bold text-white text-base tracking-wide flex items-center gap-2">
            CyberFlash AI
            <span class="px-1.5 py-0.5 rounded text-[10px] font-bold bg-primary/20 text-primary border border-primary/20">BETA</span>
          </h3>
          <p class="text-xs text-text-secondary font-medium">{{ modelName }} • Ready</p>
        </div>
      </div>
      
      <div class="flex items-center gap-1">
        <button 
          @click="clearChat"
          class="p-2 rounded-lg hover:bg-white/10 text-text-secondary hover:text-error transition-all duration-200 group"
          title="Clear Chat"
        >
          <span class="material-symbols-rounded text-lg group-hover:scale-110 transition-transform">delete_sweep</span>
        </button>

        <button 
          @click="$emit('close')"
          class="p-2 rounded-lg hover:bg-white/10 text-text-secondary hover:text-white transition-all duration-200 group"
          title="Close"
        >
          <span class="material-symbols-rounded text-lg group-hover:scale-110 transition-transform">close</span>
        </button>
      </div>
    </div>
    
    <!-- Chat Area -->
    <div class="flex-1 overflow-hidden relative bg-surface/30">
      <!-- Mesh Background -->
      <div class="absolute inset-0 mesh-gradient-bg opacity-20 pointer-events-none"></div>

      <div ref="chatContainer" class="h-full overflow-y-auto custom-scrollbar p-5 space-y-6 scroll-smooth pb-24 relative z-10">
        
        <!-- Empty State -->
        <div v-if="aiStore.messages.length === 0" class="h-full flex flex-col items-center justify-center text-center p-6 opacity-0 animate-fade-in" style="animation-fill-mode: forwards;">
          <div class="w-20 h-20 rounded-2xl bg-white/5 flex items-center justify-center mb-6 border border-white/5 shadow-xl backdrop-blur-sm">
            <span class="material-symbols-rounded text-5xl text-white/20">forum</span>
          </div>
          <h4 class="text-lg font-bold text-white mb-2">How can I help you?</h4>
          <p class="text-sm text-text-secondary font-medium mb-8 max-w-xs mx-auto leading-relaxed">
            I can analyze logs, suggest flash commands, and help troubleshoot your device.
          </p>
          
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 w-full max-w-md">
            <button 
              v-for="sug in suggestions" 
              :key="sug"
              @click="useSuggestion(sug)"
              class="px-4 py-3 rounded-xl bg-white/5 hover:bg-white/10 border border-white/5 hover:border-primary/30 text-xs text-text-secondary hover:text-white transition-all text-left flex items-center gap-3 group"
            >
              <span class="material-symbols-rounded text-primary/50 group-hover:text-primary transition-colors text-sm">lightbulb</span>
              {{ sug }}
            </button>
          </div>
        </div>

        <!-- Messages -->
        <div 
          v-for="msg in aiStore.messages" 
          :key="msg.id" 
          class="flex gap-4 group animate-slide-up"
          :class="msg.role === 'user' ? 'flex-row-reverse' : ''"
        >
          <!-- Avatar -->
          <div 
            class="w-9 h-9 rounded-full flex items-center justify-center shrink-0 shadow-lg border border-white/10 mt-1"
            :class="msg.role === 'ai' ? 'bg-surface/80 backdrop-blur-md' : 'bg-primary/80 backdrop-blur-md'"
          >
            <span class="material-symbols-rounded text-sm text-white">
              {{ msg.role === 'ai' ? 'smart_toy' : 'person' }}
            </span>
          </div>

          <!-- Bubble -->
          <div 
            class="max-w-[85%] p-4 rounded-2xl text-sm leading-relaxed shadow-lg backdrop-blur-xl border transition-all duration-300"
            :class="[
              msg.role === 'ai' 
                ? 'bg-white/5 text-white rounded-tl-none border-white/10 hover:bg-white/10' 
                : 'bg-primary/20 text-white rounded-tr-none border-primary/20 hover:bg-primary/30'
            ]"
          >
            <div v-html="formatMessage(msg.content)" class="prose prose-invert prose-sm max-w-none"></div>
          </div>
        </div>

        <!-- Thinking Indicator -->
        <div v-if="aiStore.isThinking" class="flex gap-4 animate-pulse">
          <div class="w-9 h-9 rounded-full bg-white/10 flex items-center justify-center shrink-0 border border-white/10 mt-1">
            <span class="material-symbols-rounded text-sm text-white animate-spin">sync</span>
          </div>
          <div class="bg-white/5 p-4 rounded-2xl rounded-tl-none border border-white/5 flex items-center gap-1.5">
            <span class="w-2 h-2 bg-primary/50 rounded-full animate-bounce"></span>
            <span class="w-2 h-2 bg-primary/50 rounded-full animate-bounce delay-100"></span>
            <span class="w-2 h-2 bg-primary/50 rounded-full animate-bounce delay-200"></span>
          </div>
        </div>
      </div>

      <!-- Floating Input Area -->
      <div class="absolute bottom-0 left-0 right-0 p-5 bg-gradient-to-t from-surface via-surface/90 to-transparent pt-10">
        <div class="relative flex items-center gap-2 bg-black/40 backdrop-blur-xl border border-white/10 rounded-2xl p-1.5 shadow-2xl ring-1 ring-white/5 focus-within:ring-primary/50 focus-within:border-primary/50 transition-all duration-300">
          <input 
            v-model="userInput"
            @keydown.enter="sendMessage"
            type="text" 
            placeholder="Ask CyberFlash AI..." 
            class="flex-1 bg-transparent border-none rounded-xl px-4 py-3 text-sm text-white placeholder-white/30 focus:outline-none focus:ring-0"
            :disabled="aiStore.isThinking"
          />
          <button 
            @click="sendMessage"
            :disabled="!userInput.trim() || aiStore.isThinking"
            class="p-3 rounded-xl bg-primary text-white hover:bg-primary-hover disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200 shadow-lg shadow-primary/20 hover:shadow-primary/40 active:scale-95"
          >
            <span class="material-symbols-rounded text-lg">send</span>
          </button>
        </div>
      </div>
    </div>
  </GlassCard>
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
  border-radius: 10px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.2);
}

@keyframes fade-in {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

.animate-fade-in {
  animation: fade-in 0.5s ease-out forwards;
}

.animate-slide-up {
  animation: fade-in 0.3s ease-out forwards;
}

.drop-shadow-glow {
  filter: drop-shadow(0 0 8px rgba(10, 132, 255, 0.3));
}
</style>
