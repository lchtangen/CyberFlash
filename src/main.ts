import { createApp } from "vue";
import { createPinia } from "pinia";
import "./style.css";
import App from "./App.vue";

const pinia = createPinia();
const app = createApp(App);

console.log("CyberFlash V2: Mounting App...");

app.use(pinia);
app.mount("#app");
console.log("CyberFlash V2: App Mounted");
