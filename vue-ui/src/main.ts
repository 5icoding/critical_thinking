import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import router from './router' // 引入 router
// import PrimeVue from 'primevue/config';
// // theme
// import 'primevue/resources/themes/aura-light-green/theme.css';

// import 'primevue/resources/primevue.min.css';

// import 'primeicons/primeicons.css';

import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'



const app =createApp(App);
app.use(router) // 注册 router

// app.use(PrimeVue, { unstyled: true });

app.use(ElementPlus)

app.mount('#app')
