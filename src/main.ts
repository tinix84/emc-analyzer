import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createRouter, createWebHistory } from 'vue-router'
import './assets/css/main.css'
import App from './App.vue'
import HomeView from './views/HomeView.vue'

const routes = [
  { path: '/', name: 'Home', component: HomeView }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

const pinia = createPinia()
const app = createApp(App)

app.use(pinia)
app.use(router)
app.mount('#app')
