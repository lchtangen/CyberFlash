<script setup lang="ts">
import { useNotificationStore, type Notification } from '../../../stores/notifications';
import { storeToRefs } from 'pinia';
import { ref } from 'vue';

defineProps<{
  isOpen?: boolean;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
}>();

const notificationStore = useNotificationStore();
const { notifications, history, unreadCount } = storeToRefs(notificationStore);
const selectedNotification = ref<Notification | null>(null);

const openDetails = (notification: Notification) => {
  selectedNotification.value = notification;
};

const closeDetails = () => {
  selectedNotification.value = null;
};

const getIcon = (type: string) => {
  switch (type) {
    case 'success': return 'check_circle';
    case 'warning': return 'warning';
    case 'error': return 'error';
    case 'info': default: return 'info';
  }
};

const getColorClass = (type: string) => {
  switch (type) {
    case 'success': return 'text-success bg-success/10 border-success/20';
    case 'warning': return 'text-warning bg-warning/10 border-warning/20';
    case 'error': return 'text-error bg-error/10 border-error/20';
    case 'info': default: return 'text-primary bg-primary/10 border-primary/20';
  }
};

const formatTime = (timestamp: number) => {
  const date = new Date(timestamp);
  return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
};

const dismiss = (id: string) => {
  notificationStore.removeNotification(id);
};

const clearHistory = () => {
  notificationStore.clearAll();
};
</script>

<template>
  <!-- Toasts Layer (Always Visible, Click-through) -->
  <div class="fixed top-20 right-6 z-[60] flex flex-col gap-3 w-80 pointer-events-none">
    <TransitionGroup 
      name="notification" 
      tag="div" 
      class="flex flex-col gap-3"
    >
      <div 
        v-for="notification in notifications" 
        :key="notification.id"
        @click="openDetails(notification)"
        class="pointer-events-auto relative overflow-hidden rounded-2xl border backdrop-blur-xl shadow-2xl p-4 transition-all duration-300 group cursor-pointer hover:scale-[1.02]"
        :class="[
          getColorClass(notification.type),
          'bg-surface/80 border-white/10'
        ]"
      >
        <!-- Glass Reflection Effect -->
        <div class="absolute inset-0 bg-gradient-to-br from-white/10 to-transparent opacity-50 pointer-events-none"></div>
        
        <div class="relative flex gap-3">
          <!-- Icon -->
          <div 
            class="w-10 h-10 rounded-full flex items-center justify-center shrink-0 border backdrop-blur-md"
            :class="getColorClass(notification.type)"
          >
            <span class="material-symbols-rounded text-xl">{{ getIcon(notification.type) }}</span>
          </div>

          <!-- Content -->
          <div class="flex-1 min-w-0 pt-0.5">
            <h4 class="text-sm font-bold text-white leading-tight mb-1">{{ notification.title }}</h4>
            <p class="text-xs text-gray-300 leading-relaxed break-words line-clamp-2">{{ notification.message }}</p>
          </div>

          <!-- Close Button -->
          <button 
            @click.stop="dismiss(notification.id)"
            class="shrink-0 w-6 h-6 rounded-full flex items-center justify-center text-white/50 hover:text-white hover:bg-white/10 transition-colors -mt-1 -mr-1"
          >
            <span class="material-symbols-rounded text-sm">close</span>
          </button>
        </div>

        <!-- Progress Bar -->
        <div v-if="notification.duration && notification.duration > 0" class="absolute bottom-0 left-0 h-0.5 bg-current opacity-30 w-full">
           <div 
             class="h-full bg-current opacity-100 origin-left animate-shrink"
             :style="{ animationDuration: `${notification.duration}ms` }"
           ></div>
        </div>
      </div>
    </TransitionGroup>
  </div>

  <!-- Notification Panel (History) -->
  <Transition name="panel">
    <div 
      v-if="isOpen"
      class="fixed top-16 right-6 bottom-6 w-96 z-50 flex flex-col bg-surface/90 backdrop-blur-2xl border border-white/10 rounded-2xl shadow-2xl overflow-hidden"
    >
      <!-- Header -->
      <div class="p-4 border-b border-white/10 flex items-center justify-between bg-white/5">
        <div class="flex items-center gap-2">
          <h3 class="font-bold text-white">Notifications</h3>
          <span v-if="unreadCount > 0" class="px-2 py-0.5 rounded-full bg-primary text-[10px] font-bold text-white">{{ unreadCount }} New</span>
        </div>
        <div class="flex items-center gap-1">
          <button 
            @click="notificationStore.markAllAsRead()" 
            class="p-2 rounded-lg hover:bg-white/10 text-text-secondary hover:text-white transition-colors text-xs"
            title="Mark all as read"
          >
            <span class="material-symbols-rounded text-lg">done_all</span>
          </button>
          <button 
            @click="clearHistory" 
            class="p-2 rounded-lg hover:bg-white/10 text-text-secondary hover:text-white transition-colors text-xs"
            title="Clear all"
          >
            <span class="material-symbols-rounded text-lg">delete_sweep</span>
          </button>
        </div>
      </div>

      <!-- List -->
      <div class="flex-1 overflow-y-auto custom-scrollbar p-2 space-y-2">
        <div v-if="history.length === 0" class="h-full flex flex-col items-center justify-center text-text-secondary opacity-50">
          <span class="material-symbols-rounded text-4xl mb-2">notifications_off</span>
          <p class="text-sm">No notifications</p>
        </div>

        <div 
          v-for="item in history" 
          :key="item.id"
          @click="openDetails(item)"
          class="p-3 rounded-xl border border-white/5 bg-white/5 hover:bg-white/10 transition-colors group cursor-pointer"
        >
          <div class="flex gap-3">
            <div 
              class="w-8 h-8 rounded-full flex items-center justify-center shrink-0 border border-white/10"
              :class="getColorClass(item.type).replace('bg-', 'bg-opacity-20 ')"
            >
              <span class="material-symbols-rounded text-sm">{{ getIcon(item.type) }}</span>
            </div>
            <div class="flex-1 min-w-0">
              <div class="flex justify-between items-start mb-1">
                <h4 class="text-sm font-medium text-white truncate pr-2">{{ item.title }}</h4>
                <span class="text-[10px] text-text-secondary whitespace-nowrap">{{ formatTime(item.timestamp) }}</span>
              </div>
              <p class="text-xs text-text-secondary leading-relaxed line-clamp-2">{{ item.message }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </Transition>

  <!-- Details Modal -->
  <Transition name="modal">
    <div v-if="selectedNotification" class="fixed inset-0 z-[70] flex items-center justify-center p-6">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="closeDetails"></div>
      
      <div class="relative w-full max-w-md bg-surface border border-white/10 rounded-2xl shadow-2xl overflow-hidden flex flex-col max-h-[80vh] animate-in fade-in zoom-in-95 duration-200">
        <!-- Header -->
        <div class="p-6 pb-4 border-b border-white/10 flex items-start gap-4">
          <div 
            class="w-12 h-12 rounded-full flex items-center justify-center shrink-0 border backdrop-blur-md"
            :class="getColorClass(selectedNotification.type)"
          >
            <span class="material-symbols-rounded text-2xl">{{ getIcon(selectedNotification.type) }}</span>
          </div>
          <div class="flex-1">
            <h3 class="text-lg font-bold text-white leading-tight">{{ selectedNotification.title }}</h3>
            <p class="text-xs text-text-secondary mt-1">{{ formatTime(selectedNotification.timestamp) }}</p>
          </div>
          <button 
            @click="closeDetails"
            class="shrink-0 w-8 h-8 rounded-full flex items-center justify-center text-white/50 hover:text-white hover:bg-white/10 transition-colors"
          >
            <span class="material-symbols-rounded">close</span>
          </button>
        </div>

        <!-- Content -->
        <div class="p-6 overflow-y-auto custom-scrollbar">
          <p class="text-sm text-gray-200 whitespace-pre-wrap leading-relaxed font-mono bg-black/20 p-4 rounded-lg border border-white/5">{{ selectedNotification.message }}</p>
        </div>

        <!-- Footer -->
        <div class="p-4 border-t border-white/10 bg-white/5 flex justify-end">
          <button 
            @click="closeDetails"
            class="px-4 py-2 rounded-lg bg-white/10 hover:bg-white/20 text-white text-sm font-medium transition-colors"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  </Transition>

  <!-- Backdrop for closing -->
  <div v-if="isOpen" @click="emit('close')" class="fixed inset-0 z-40 bg-black/20 backdrop-blur-sm"></div>
</template>

<style scoped>
.notification-enter-active,
.notification-leave-active {
  transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.notification-enter-from {
  opacity: 0;
  transform: translateX(30px) scale(0.9);
}

.notification-leave-to {
  opacity: 0;
  transform: translateX(30px) scale(0.9);
}

.panel-enter-active,
.panel-leave-active {
  transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
}

.panel-enter-from,
.panel-leave-to {
  opacity: 0;
  transform: translateX(20px);
}

.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.2s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

@keyframes shrink {
  from { width: 100%; }
  to { width: 0%; }
}

.animate-shrink {
  animation-name: shrink;
  animation-timing-function: linear;
  animation-fill-mode: forwards;
}
</style>
