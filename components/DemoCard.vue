<template>
  <div class="bg-white rounded-xl shadow-lg hover:shadow-xl transition-all duration-300 border border-gray-200 hover:border-blue-300">
    <div class="p-6">
      <!-- Icon and Title -->
      <div class="flex items-center mb-4">
        <div class="text-4xl mr-4">{{ icon }}</div>
        <h3 class="text-xl font-bold text-gray-800">{{ title }}</h3>
      </div>
      
      <!-- Description -->
      <p class="text-gray-600 mb-6 text-sm leading-relaxed">
        {{ description }}
      </p>
      
      <!-- Action Button -->
      <div class="flex justify-between items-center">
        <a 
          :href="url" 
          :target="external ? '_blank' : '_self'"
          :rel="external ? 'noopener noreferrer' : ''"
          class="inline-flex items-center px-4 py-2 bg-gradient-to-r from-blue-500 to-purple-600 text-white rounded-lg hover:from-blue-600 hover:to-purple-700 transition-all duration-200 font-medium text-sm"
        >
          <span>{{ external ? 'Open' : 'View Demo' }}</span>
          <svg 
            class="ml-2 w-4 h-4" 
            fill="none" 
            stroke="currentColor" 
            viewBox="0 0 24 24"
          >
            <path 
              stroke-linecap="round" 
              stroke-linejoin="round" 
              stroke-width="2" 
              :d="external ? 'M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14' : 'M9 5l7 7-7 7'"
            />
          </svg>
        </a>
        
        <!-- Status Badge -->
        <span class="px-2 py-1 bg-green-100 text-green-700 rounded-full text-xs font-medium">
          Ready
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  title: string
  description: string
  url: string
  icon: string
  external?: boolean
}

defineProps<Props>()
</script>

<style scoped>
/* Additional hover effects */
.bg-white:hover {
  transform: translateY(-2px);
}
</style>
