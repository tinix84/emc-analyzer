// nuxt.config.ts
export default defineNuxtConfig({
  devtools: { enabled: true },
  
  modules: [
    '@nuxtjs/tailwindcss',
    '@pinia/nuxt',
    '@vueuse/nuxt'
  ],
  
  css: ['~/assets/css/main.css'],
  
  // SPA mode for better WASM support
  ssr: false,
  
  // WASM support
  nitro: {
    experimental: {
      wasm: true
    }
  },
  
  // Vite configuration for WASM
  vite: {
    server: {
      fs: {
        allow: ['..']
      }
    },
    optimizeDeps: {
      exclude: ['@/wasm/emc_wasm']
    },
    // Add WASM support
    plugins: [],
    define: {
      global: 'globalThis'
    }
  },
  
  // App configuration
  app: {
    head: {
      title: 'EMC Spectrum Analyzer',
      meta: [
        { charset: 'utf-8' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1' },
        { name: 'description', content: 'EMC compliance analyzer with Rust WebAssembly' },
        { name: 'keywords', content: 'EMC, spectrum analyzer, compliance, CISPR, EN55032' }
      ],
      link: [
        { rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' }
      ]
    }
  },
  
  // TypeScript configuration - DISABLE strict checking
  typescript: {
    typeCheck: false
  },
  
  // Build configuration
  build: {
    transpile: ['chart.js']
  },
  
  // Runtime configuration
  runtimeConfig: {
    public: {
      appName: 'EMC Analyzer',
      appVersion: '1.0.0'
    }
  }
})