<template>
  <div id="app" class="min-h-screen bg-gray-50">
    <Analytics />
    <nav class="bg-white shadow-md">
      <div class="container mx-auto px-4">
        <div class="flex justify-between items-center py-4">
          <h1 class="text-2xl font-bold text-blue-600">🔬 EMC Vue Analyzer</h1>
          <div class="flex items-center gap-4">
            <div class="text-sm text-gray-500">
              v{{ version }}
            </div>
            <!-- Contact Button -->
            <div class="relative">
              <button
                @click="showContactMenu = !showContactMenu"
                class="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors duration-200"
              >
                📧 Contact
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                </svg>
              </button>
              
              <!-- Contact Menu -->
              <div
                v-if="showContactMenu"
                @click.stop
                class="absolute right-0 mt-2 w-48 bg-white rounded-lg shadow-lg border border-gray-200 z-50"
              >
                <div class="py-2">
                  <a
                    href="mailto:tinix84@example.com"
                    class="flex items-center gap-2 px-4 py-2 text-gray-700 hover:bg-gray-100 transition-colors duration-200"
                  >
                    📧 Email
                  </a>
                  <a
                    href="https://github.com/tinix84"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="flex items-center gap-2 px-4 py-2 text-gray-700 hover:bg-gray-100 transition-colors duration-200"
                  >
                    🐙 GitHub
                  </a>
                  <a
                    href="https://linkedin.com/in/tinix84"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="flex items-center gap-2 px-4 py-2 text-gray-700 hover:bg-gray-100 transition-colors duration-200"
                  >
                    💼 LinkedIn
                  </a>
                  <button
                    @click="openFeedbackDialog"
                    class="flex items-center gap-2 px-4 py-2 text-gray-700 hover:bg-gray-100 transition-colors duration-200 w-full text-left"
                  >
                    � Feedback
                  </button>
                </div>
              </div>
            </div>
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
import { ref, onMounted, onUnmounted } from 'vue'
import { Analytics } from '@vercel/analytics/vue'

const version = ref('1.0.0')
const showContactMenu = ref(false)

// Close contact menu when clicking outside
const handleClickOutside = (event: MouseEvent) => {
  const target = event.target as HTMLElement
  if (!target.closest('.relative')) {
    showContactMenu.value = false
  }
}

// Open feedback dialog
const openFeedbackDialog = () => {
  showContactMenu.value = false
  const subject = encodeURIComponent('EMC Vue Analyzer - Feedback')
  const body = encodeURIComponent(`Hi,

I have some feedback about the EMC Vue Analyzer:

- Feature request: 
- Bug report: 
- General feedback: 

Best regards,
`)
  window.open(`mailto:tinix84@example.com?subject=${subject}&body=${body}`)
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}
</style>
