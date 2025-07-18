<template>
  <div id="app" class="min-h-screen bg-gray-50">
    <Analytics />
    <nav class="bg-white shadow-md">
      <div class="container mx-auto px-4">
        <div class="flex justify-between items-center py-4">
          <h1 class="text-2xl font-bold text-blue-600">🔬 EMC Vue Analyzer</h1>
          <div class="text-sm text-gray-500">
            v{{ version }}
          </div>
        </div>
      </div>
    </nav>
    
    <main class="container mx-auto px-4 py-8">
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Analytics } from '@vercel/analytics/vue'

const version = ref('1.0.0')
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}
</style>
