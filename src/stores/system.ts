import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useSystemStore = defineStore('system', () => {
  const isDriverCheckComplete = ref(false);
  const isPermissionCheckComplete = ref(false);
  const activeModal = ref<'drivers' | 'permissions' | null>(null);

  function setDriverCheckComplete(value: boolean) {
    isDriverCheckComplete.value = value;
  }

  function setPermissionCheckComplete(value: boolean) {
    isPermissionCheckComplete.value = value;
  }

  function setActiveModal(modal: 'drivers' | 'permissions' | null) {
    activeModal.value = modal;
  }

  return {
    isDriverCheckComplete,
    isPermissionCheckComplete,
    activeModal,
    setDriverCheckComplete,
    setPermissionCheckComplete,
    setActiveModal,
  };
});
