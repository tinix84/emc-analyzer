<template>
  <nav class="bg-white shadow-md mb-8">
    <div class="container mx-auto px-4">
      <div class="flex justify-between items-center py-4">
        <!-- Logo -->
        <div class="flex items-center">
          <h1 class="text-2xl font-bold text-blue-600">🔬 EMC Analyzer</h1>
        </div>
        
        <!-- Navigation Links -->
        <div class="hidden md:flex space-x-6">
          <NuxtLink 
            to="/" 
            class="text-gray-700 hover:text-blue-600 transition-colors duration-200"
            :class="{ 'text-blue-600 font-semibold': $route.path === '/' }"
          >
            Home
          </NuxtLink>
          
          <!-- Demo Links Dropdown -->
          <div class="relative group">
            <button class="text-gray-700 hover:text-blue-600 transition-colors duration-200 flex items-center">
              Demo Pages
              <svg class="ml-1 w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
              </svg>
            </button>
            
            <!-- Dropdown Menu -->
            <div class="absolute left-0 mt-2 w-64 bg-white rounded-md shadow-lg opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all duration-200 z-50">
              <div class="py-2">
                <a 
                  href="/vue-emc-prototype.html" 
                  class="block px-4 py-2 text-gray-700 hover:bg-blue-50 hover:text-blue-600 transition-colors duration-200"
                >
                  🔬 Vue EMC Prototype
                </a>
                <a 
                  href="/vue-emc-wasm-demo.html" 
                  class="block px-4 py-2 text-gray-700 hover:bg-blue-50 hover:text-blue-600 transition-colors duration-200"
                >
                  🦀 Vue WASM Demo
                </a>
                <a 
                  href="/pure-vue-wasm-test.html" 
                  class="block px-4 py-2 text-gray-700 hover:bg-blue-50 hover:text-blue-600 transition-colors duration-200"
                >
                  ⚡ Pure Vue WASM Test
                </a>
                <a 
                  href="/simple-wasm-test.html" 
                  class="block px-4 py-2 text-gray-700 hover:bg-blue-50 hover:text-blue-600 transition-colors duration-200"
                >
                  🧪 Simple WASM Test
                </a>
                <a 
                  href="/test.html" 
                  class="block px-4 py-2 text-gray-700 hover:bg-blue-50 hover:text-blue-600 transition-colors duration-200"
                >
                  🔧 Basic Test
                </a>
              </div>
            </div>
          </div>
          
          <a 
            href="/data" 
            class="text-gray-700 hover:text-blue-600 transition-colors duration-200"
          >
            Data Files
          </a>
          
          <a 
            href="https://github.com/tinix84/nuxtjs-boilerplate" 
            target="_blank"
            rel="noopener noreferrer"
            class="text-gray-700 hover:text-blue-600 transition-colors duration-200 flex items-center"
          >
            GitHub
            <svg class="ml-1 w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
            </svg>
          </a>
        </div>
        
        <!-- Mobile Menu Button -->
        <div class="md:hidden">
          <button 
            @click="mobileMenuOpen = !mobileMenuOpen"
            class="text-gray-700 hover:text-blue-600 transition-colors duration-200"
          >
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
            </svg>
          </button>
        </div>
      </div>
      
      <!-- Mobile Menu -->
      <div v-if="mobileMenuOpen" class="md:hidden border-t border-gray-200 py-4">
        <div class="space-y-2">
          <NuxtLink 
            to="/" 
            class="block py-2 text-gray-700 hover:text-blue-600 transition-colors duration-200"
            @click="mobileMenuOpen = false"
          >
            Home
          </NuxtLink>
          
          <div class="space-y-1">
            <div class="text-gray-500 text-sm font-medium py-1">Demo Pages:</div>
            <a 
              href="/vue-emc-prototype.html" 
              class="block py-2 pl-4 text-gray-700 hover:text-blue-600 transition-colors duration-200 text-sm"
              @click="mobileMenuOpen = false"
            >
              🔬 Vue EMC Prototype
            </a>
            <a 
              href="/vue-emc-wasm-demo.html" 
              class="block py-2 pl-4 text-gray-700 hover:text-blue-600 transition-colors duration-200 text-sm"
              @click="mobileMenuOpen = false"
            >
              🦀 Vue WASM Demo
            </a>
            <a 
              href="/pure-vue-wasm-test.html" 
              class="block py-2 pl-4 text-gray-700 hover:text-blue-600 transition-colors duration-200 text-sm"
              @click="mobileMenuOpen = false"
            >
              ⚡ Pure Vue WASM Test
            </a>
            <a 
              href="/simple-wasm-test.html" 
              class="block py-2 pl-4 text-gray-700 hover:text-blue-600 transition-colors duration-200 text-sm"
              @click="mobileMenuOpen = false"
            >
              🧪 Simple WASM Test
            </a>
            <a 
              href="/test.html" 
              class="block py-2 pl-4 text-gray-700 hover:text-blue-600 transition-colors duration-200 text-sm"
              @click="mobileMenuOpen = false"
            >
              🔧 Basic Test
            </a>
          </div>
        </div>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

const mobileMenuOpen = ref(false)

// Close mobile menu on route change
const route = useRoute()
watch(route, () => {
  mobileMenuOpen.value = false
})
</script>

<style scoped>
/* Additional styling for dropdown */
.group:hover .group-hover\:opacity-100 {
  opacity: 1;
}

.group:hover .group-hover\:visible {
  visibility: visible;
}
</style>
