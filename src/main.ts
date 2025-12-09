import { createApp } from "vue";
import { createPinia } from "pinia";
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate';
import "./style.css";
import "./assets/animations.css";
import App from "./App.vue";

const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);
const app = createApp(App);

// Global Error Handler
app.config.errorHandler = (err, _instance, info) => {
  console.error("Global Error:", err);
  console.error("Info:", info);
  // In a real app, we might want to send this to a logging service or show a toast
  // For now, we just log it to console which will be picked up by the terminal if connected
};

console.log("CyberFlash V2: Mounting App...");

app.use(pinia);
app.mount("#app");
console.log("CyberFlash V2: App Mounted");
