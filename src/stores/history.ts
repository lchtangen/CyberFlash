import { defineStore } from 'pinia';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export interface ActivityItem {
  id: number;
  action: string;
  details?: string;
  status: 'success' | 'error' | 'warning' | 'info';
  timestamp: string;
  device_serial?: string;
}

export const useHistoryStore = defineStore('history', () => {
  const activities = ref<ActivityItem[]>([]);
  const isLoading = ref(false);

  // Listen for real-time updates
  listen<ActivityItem>('history:update', (event) => {
    // Avoid duplicates if we did optimistic update
    const exists = activities.value.find(a => a.id === event.payload.id);
    if (!exists) {
      activities.value.unshift(event.payload);
    }
  });

  const fetchHistory = async (limit = 50) => {
    isLoading.value = true;
    try {
      activities.value = await invoke<ActivityItem[]>('get_activity_log', { limit });
    } catch (error) {
      console.error('Failed to fetch history:', error);
    } finally {
      isLoading.value = false;
    }
  };

  const addEntry = async (
    action: string, 
    status: 'success' | 'error' | 'warning' | 'info', 
    details?: string, 
    device_serial?: string
  ) => {
    try {
      // Optimistic update
      const tempId = Date.now();
      const newItem: ActivityItem = {
        id: tempId,
        action,
        status,
        details,
        device_serial,
        timestamp: new Date().toLocaleString() // Backend will overwrite this but good for immediate display
      };
      
      activities.value.unshift(newItem);
      
      // Persist
      await invoke('add_activity', { 
        action, 
        details, 
        status, 
        deviceSerial: device_serial 
      });
      
      // Refresh to get real ID and timestamp
      await fetchHistory();
    } catch (error) {
      console.error('Failed to add activity:', error);
    }
  };

  const clearHistory = async () => {
    try {
      await invoke('clear_activity_log');
      activities.value = [];
    } catch (error) {
      console.error('Failed to clear history:', error);
    }
  };

  return {
    activities,
    isLoading,
    fetchHistory,
    addEntry,
    clearHistory
  };
});
