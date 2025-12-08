<script setup lang="ts">
import { ref, nextTick, watch, computed } from 'vue';
import { useAIStore } from '../../stores/ai';
import { useSettingsStore } from '../../stores/settings';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../ui/GlassCard.vue';

const props = defineProps<{
  heightClass?: string;
}>();

const aiStore = useAIStore();
const settingsStore = useSettingsStore();

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

const sendMessage = async () => {
  if (!userInput.value.trim()) return;
  
  const userMsg = userInput.value;
  aiStore.addMessage('user', userMsg);
  userInput.value = '';
  aiStore.isThinking = true;

  try {
    const response = await invoke<string>('ask_gemini', { 
      prompt: userMsg,
      apiKey: settingsStore.geminiApiKey,
      model: settingsStore.aiModel
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
</script>
<template>
  <GlassCard 
    noPadding
    class="flex flex-col w-full transition-all duration-300"
    :class="heightClass || 'h-[500px]'"
  >
    <!-- Header -->
    <div class="px-4 py-3 border-b border-white/10 flex items-center justify-between bg-white/5">
      <div class="flex items-center gap-3">
        <div class="w-8 h-8 rounded-full bg-primary/20 flex items-center justify-center">
          <span class="material-symbols-rounded text-primary text-sm">smart_toy</span>
        </div>
        <div>
          <h3 class="font-bold text-white text-sm tracking-wide">{{ modelName }}</h3>
          <div class="flex items-center gap-1.5">
            <span class="w-1.5 h-1.5 rounded-full bg-success"></span>
            <p class="text-[10px] text-text-secondary font-mono uppercase">Online</p>
          </div>
        </div>
      </div>
      
      <button 
        @click="clearChat"
        class="p-1.5 rounded-lg hover:bg-white/10 text-text-secondary hover:text-white transition-colors"
        title="Clear Chat"
      >
        <span class="material-symbols-rounded text-sm">delete_sweep</span>
      </button>
    </div>
    
    <!-- Chat Area -->
    <div class="flex-1 overflow-hidden relative">
      <div ref="chatContainer" class="h-full overflow-y-auto custom-scrollbar p-4 space-y-4 scroll-smooth pb-4">
        <div v-if="aiStore.messages.length === 0" class="h-full flex flex-col items-center justify-center text-center p-6 opacity-50">
          <div class="w-16 h-16 rounded-full bg-white/5 flex items-center justify-center mb-4">
            <span class="material-symbols-rounded text-4xl text-white/20">forum</span>
          </div>
          <p class="text-sm text-text-secondary font-medium">How can I help you today?</p>
        </div>

        <div 
          v-for="msg in aiStore.messages" 
          :key="msg.id" 
          class="flex gap-4 group"
          :class="msg.role === 'user' ? 'flex-row-reverse' : ''"
        >
          <!-- Avatar -->
          <div 
            class="w-8 h-8 rounded-full flex items-center justify-center shrink-0 shadow-lg border border-white/10"
            :class="msg.role === 'ai' ? 'bg-surface/80 backdrop-blur-md' : 'bg-primary/80 backdrop-blur-md'"
          >
            <span class="material-symbols-rounded text-sm text-white">
              {{ msg.role === 'ai' ? 'smart_toy' : 'person' }}
            </span>
          </div>

          <!-- Bubble -->
          <div 
            class="max-w-[80%] p-3 rounded-xl text-sm leading-relaxed shadow-sm border border-white/5"
            :class="[
              msg.role === 'ai' 
                ? 'bg-white/10 text-white rounded-tl-none' 
                : 'bg-primary/20 text-white rounded-tr-none border-primary/20'
            ]"
          >
            <div>{{ msg.content }}</div>
          </div>
        </div>

        <!-- Thinking Indicator -->
        <div v-if="aiStore.isThinking" class="flex gap-4">
          <div class="w-8 h-8 rounded-full bg-white/10 flex items-center justify-center shrink-0 border border-white/10">
            <span class="material-symbols-rounded text-sm text-white animate-spin">sync</span>
          </div>
          <div class="bg-white/5 p-3 rounded-xl rounded-tl-none border border-white/5 flex items-center gap-1">
            <span class="w-1.5 h-1.5 bg-white/50 rounded-full animate-bounce"></span>
            <span class="w-1.5 h-1.5 bg-white/50 rounded-full animate-bounce delay-100"></span>
            <span class="w-1.5 h-1.5 bg-white/50 rounded-full animate-bounce delay-200"></span>
          </div>
        </div>
      </div>

      <!-- Input Area -->
      <div class="p-4 border-t border-white/10 bg-white/5">
        <div class="flex gap-2">
          <input 
            v-model="userInput"
            @keydown.enter="sendMessage"
            type="text" 
            placeholder="Ask Gemini..." 
            class="flex-1 bg-black/20 border border-white/10 rounded-lg px-4 py-2 text-sm text-white placeholder-white/30 focus:outline-none focus:border-primary/50 transition-colors"
            :disabled="aiStore.isThinking"
          />
          <button 
            @click="sendMessage"
            :disabled="!userInput.trim() || aiStore.isThinking"
            class="p-2 rounded-lg bg-primary text-white hover:bg-primary-hover disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
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
  border-radius: 2px;
}
</style>
