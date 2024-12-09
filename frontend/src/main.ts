import "./assets/main.css";

import { createApp } from "vue";
import { createPinia } from "pinia";
import { clerkPlugin } from "vue-clerk";
import piniaPluginPersistedstate from "pinia-plugin-persistedstate";

import App from "./App.vue";
import router from "./router";
import PrimeVue from "primevue/config";
import Aura from "@primevue/themes/aura";
import { Ripple } from "primevue";

const pinia = createPinia();
const app = createApp(App);

app.directive("ripple", Ripple);

pinia.use(piniaPluginPersistedstate);
app.use(PrimeVue, {
    theme: {
        preset: Aura,
    },
    ripple: true,
});
app.use(pinia);
app.use(router);
app.use(clerkPlugin, {
    publishableKey: import.meta.env.VITE_CLERK_PUBLISHABLE_KEY,
});

app.mount("#app");
