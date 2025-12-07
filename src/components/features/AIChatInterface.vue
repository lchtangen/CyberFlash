<script setup lang="ts">
import { ref } from 'vue';

interface Message {
  id: string;
  role: 'user' | 'ai';
  content: string;
  timestamp: Date;
}

const messages = ref<Message[]>([
  { id: '1', role: 'ai', content: 'Hello! I am CyberFlash AI. How can I help you with flashing today?', timestamp: new Date() }
]);
const userInput = ref('');
const isLoading = ref(false);

const sendMessage = () => {
  if (!userInput.value.trim()) return;
  
  messages.value.push({
    id: Date.now().toString(),
    role: 'user',
    content: userInput.value,
    timestamp: new Date()
  });
  
  userInput.value = '';
  isLoading.value = true;
  
  // Simulate AI response
  setTimeout(() => {
    messages.value.push({
      id: (Date.now() + 1).toString(),
      role: 'ai',
      content: 'I can analyze your device logs or suggest the best ROM for your OnePlus 7 Pro.',
      timestamp: new Date()
    });
    isLoading.value = false;
  }, 1500);
};
</script>
<template>
  <div class="flex flex-col h-[500px] w-full max-w-md bg-white/5 border border-white/10 rounded-xl backdrop-blur-md overflow-hidden">
    <div class="p-4 border-b border-white/10 flex items-center gap-3 bg-black/20">
      <div class="w-8 h-8 rounded-full bg-gradient-to-br from-primary to-secondary flex items-center justify-center shadow-[0_0_15px_rgba(0,240,255,0.3)]">
        <span class="material-icons text-white text-sm">smart_toy</span>
      </div>
      <div>
        <h3 class="font-bold text-white tracking-wide">AI Assistant</h3>
        <p class="text-xs text-primary/80 font-mono">Powered by Claude Sonnet 4.5</p>
      </div>
    </div>
    
    <div class="flex-1 overflow-hidden relative">
      <div class="h-full overflow-y-auto custom-scrollbar p-4 space-y-4">
        <div 
          v-for="msg in messages" 
          :key="msg.id" 
          class="flex gap-3"
          :class="msg.role === 'user' ? 'flex-row-reverse' : ''"
        >
          <div 
            class="w-8 h-8 rounded-full flex items-center justify-center shrink-0 border border-white/10"
            :class="msg.role === 'user' ? 'bg-primary/20 text-primary' : 'bg-secondary/20 text-secondary'"
          >
            <span class="material-icons text-sm">{{ msg.role === 'user' ? 'person' : 'smart_toy' }}</span>
          </div>
          
          <div 
            class="p-3 rounded-lg max-w-[80%] text-sm leading-relaxed border"
            :class="msg.role === 'user' ? 'bg-primary/10 border-primary/30 text-white rounded-tr-none' : 'bg-white/5 border-white/10 text-gray-200 rounded-tl-none'"
          >
            {{ msg.content }}
          </div>
        </div>
        
        <div v-if="isLoading" class="flex gap-3">
           <div class="w-8 h-8 rounded-full bg-secondary/20 text-secondary flex items-center justify-center shrink-0 border border-white/10">
             <span class="material-icons text-sm">smart_toy</span>
           </div>
           <div class="bg-white/5 border border-white/10 p-3 rounded-lg rounded-tl-none flex gap-1 items-center">
             <div class="w-1.5 h-1.5 bg-primary rounded-full animate-bounce"></div>
             <div class="w-1.5 h-1.5 bg-primary rounded-full animate-bounce delay-75"></div>
             <div class="w-1.5 h-1.5 bg-primary rounded-full animate-bounce delay-150"></div>
           </div>
        </div>
      </div>
    </div>
    
    <div class="p-4 border-t border-white/10 flex gap-2 bg-black/20">
      <input 
        v-model="userInput" 
        placeholder="Ask AI..." 
        class="flex-1 bg-black/40 border border-white/10 rounded px-3 py-2 text-sm text-white focus:outline-none focus:border-primary/50 focus:ring-1 focus:ring-primary/50 transition-all placeholder-white/20"
        @keydown.enter="sendMessage"
      />
      <button @click="sendMessage" :disabled="!userInput || isLoading" class="p-2 bg-primary text-black rounded hover:bg-primary/90 transition-colors disabled:opacity-50">
        <span class="material-icons">send</span>
      </button>
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
