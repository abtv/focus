import { createApp } from "vue";
import PrimeVue from 'primevue/config';
import Aura from '@primevue/themes/aura';
import AutoComplete from 'primevue/autocomplete';
import Editor from 'primevue/editor';
import App from "./App.vue";
import Panel from 'primevue/panel';

createApp(App)
  .use(PrimeVue, {
    theme: {
      preset: Aura
    }
  })
  .component('AutoComplete', AutoComplete)
  .component('Editor', Editor)
  .component('Panel', Panel)
  .mount("#app");
