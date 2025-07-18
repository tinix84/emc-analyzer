<template>
  <div class="space-y-3">
    <!-- Standard Selection - Single Column Layout -->
    <div class="grid grid-cols-1 gap-3">
      <div
        v-for="standard in standards"
        :key="standard.id"
        class="border rounded-lg p-3 cursor-pointer transition-all duration-200"
        :class="{
          'border-blue-500 bg-blue-900/20': selectedStandard === standard.id,
          'border-gray-600 hover:border-gray-500 bg-gray-700/50': selectedStandard !== standard.id
        }"
        @click="selectStandard(standard.id)"
      >
        <div class="flex items-center space-x-3">
          <div class="text-lg">📋</div>
          <div class="flex-1">
            <h3 class="font-semibold text-white">{{ standard.name }}</h3>
            <p class="text-sm text-gray-300">{{ standard.description }}</p>
          </div>
          
          <!-- Selection Indicator -->
          <div 
            v-if="selectedStandard === standard.id"
            class="text-blue-400 text-lg"
          >
            ✅
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useEMCStore } from '@/stores/emcStore'

const props = defineProps<{
  selectedStandard: string
}>()

const emit = defineEmits<{
  'standard-changed': [standardId: string]
}>()

const emcStore = useEMCStore()
const standards = computed(() => emcStore.standards)

const selectStandard = (standardId: string) => {
  emcStore.setCurrentStandard(standardId)
  emit('standard-changed', standardId)
}
</script>
