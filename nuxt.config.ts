// nuxt.config.ts
export default defineNuxtConfig({
  devtools: { enabled: true },
  modules: [
    '@nuxtjs/tailwindcss',
    '@pinia/nuxt',
    '@vueuse/nuxt'
  ],
  css: ['~/assets/css/main.css'],
  ssr: false, // SPA mode for WASM
  nitro: {
    experimental: {
      wasm: true
    }
  },
  vite: {
    server: {
      fs: {
        allow: ['..']
      }
    },
    optimizeDeps: {
      exclude: ['@/wasm/emc_wasm']
    }
  }
})