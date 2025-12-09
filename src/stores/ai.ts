import { defineStore } from 'pinia';
import { ref } from 'vue';

export interface ChatMessage {
  id: string;
  role: 'user' | 'ai' | 'system';
  content: string;
  timestamp: number;
  isTyping?: boolean;
  toolsUsed?: string[];
}

export interface Suggestion {
  id: string;
  title: string;
  description: string;
  action: string; // e.g., 'run_scan', 'open_settings'
  icon: string;
  type: 'info' | 'warning' | 'action';
}

export const useAIStore = defineStore('ai', () => {
  const messages = ref<ChatMessage[]>([
    {
      id: 'init',
      role: 'ai',
      content: 'Hello! I am your CyberFlash AI Assistant. I can help you flash ROMs, troubleshoot device issues, or automate tasks. How can I assist you today?',
      timestamp: Date.now(),
    }
  ]);
  
  const isThinking = ref(false);
  const isVisible = ref(false);
  const apiKey = ref(''); // Add apiKey ref
  const suggestions = ref<Suggestion[]>([
    { id: '1', title: 'Scan Device', description: 'Check for connected ADB/Fastboot devices', action: 'adb_scan', icon: 'devices', type: 'action' },
    { id: '2', title: 'Analyze Logs', description: 'Scan recent logs for errors', action: 'analyze_logcat', icon: 'analytics', type: 'info' },
  ]);

  const addMessage = (role: 'user' | 'ai' | 'system', content: string) => {
    messages.value.push({
      id: Date.now().toString(),
      role,
      content,
      timestamp: Date.now(),
    });
  };

  const toggleVisibility = () => {
    isVisible.value = !isVisible.value;
  };

  const clearChat = () => {
    messages.value = [messages.value[0]]; // Keep welcome message
  };

  return {
    messages,
    isThinking,
    isVisible,
    apiKey, // Expose apiKey
    suggestions,
    addMessage,
    toggleVisibility,
    clearChat
  };
});
