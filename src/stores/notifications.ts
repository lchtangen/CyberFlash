import { defineStore } from 'pinia';
import { ref } from 'vue';

export type NotificationType = 'info' | 'success' | 'warning' | 'error';

export interface Notification {
  id: string;
  title: string;
  message: string;
  type: NotificationType;
  timestamp: number;
  duration?: number; // ms, if 0 or undefined, it stays until dismissed
}

export const useNotificationStore = defineStore('notifications', () => {
  const notifications = ref<Notification[]>([]);
  const history = ref<Notification[]>([]);
  const unreadCount = ref(0);

  function addNotification(payload: {
    title: string;
    message: string;
    type?: NotificationType;
    duration?: number;
  }) {
    const { title, message, type = 'info', duration = 5000 } = payload;
    const id = Date.now().toString() + Math.random().toString(36).substring(2, 9);
    const notification: Notification = {
      id,
      title,
      message,
      type,
      timestamp: Date.now(),
      duration
    };

    // Add to active notifications (Toasts)
    notifications.value.push(notification);
    
    // Add to history
    history.value.unshift(notification);
    unreadCount.value++;

    if (duration > 0) {
      setTimeout(() => {
        removeNotification(id);
      }, duration);
    }
  }

  function removeNotification(id: string) {
    const index = notifications.value.findIndex(n => n.id === id);
    if (index !== -1) {
      notifications.value.splice(index, 1);
    }
  }

  function markAllAsRead() {
    unreadCount.value = 0;
  }

  function clearAll() {
    notifications.value = [];
    history.value = [];
    unreadCount.value = 0;
  }

  return {
    notifications,
    history,
    unreadCount,
    addNotification,
    removeNotification,
    markAllAsRead,
    clearAll
  };
});
