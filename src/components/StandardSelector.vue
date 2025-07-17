<template>
  <div class="space-y-4">
    <!-- Standard Selection -->
    <div class="grid md:grid-cols-2 gap-4">
      <div
        v-for="standard in standards"
        :key="standard.id"
        class="border rounded-lg p-4 cursor-pointer transition-all duration-200"
        :class="{
          'border-blue-500 bg-blue-50': selectedStandard === standard.id,
          'border-gray-200 hover:border-gray-300': selectedStandard !== standard.id
        }"
        @click="selectStandard(standard.id)"
      >
        <div class="flex items-start space-x-3">
          <div class="text-2xl">📋</div>
          <div class="flex-1">
            <h3 class="font-semibold text-gray-800">{{ standard.name }}</h3>
            <p class="text-sm text-gray-600 mt-1">{{ standard.description }}</p>
            
            <!-- Frequency Ranges Preview -->
            <div class="mt-3">
              <div class="text-xs text-gray-500 mb-1">Frequency Ranges:</div>
              <div class="space-y-1">
                <div 
                  v-for="(range, index) in standard.frequencyRanges.slice(0, 3)"
                  :key="index"
                  class="text-xs bg-gray-100 px-2 py-1 rounded"
                >
                  {{ formatFrequency(range.startFreq) }} - {{ formatFrequency(range.endFreq) }}: 
                  {{ range.limit }} dBμV
                </div>
                <div 
                  v-if="standard.frequencyRanges.length > 3"
                  class="text-xs text-gray-400"
                >
                  +{{ standard.frequencyRanges.length - 3 }} more ranges...
                </div>
              </div>
            </div>
          </div>
          
          <!-- Selection Indicator -->
          <div 
            v-if="selectedStandard === standard.id"
            class="text-blue-500 text-xl"
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

const selectedStandardDetails = computed(() => {
  return standards.value.find(s => s.id === props.selectedStandard) || null
})

const selectStandard = (standardId: string) => {
  emcStore.setCurrentStandard(standardId)
  emit('standard-changed', standardId)
}

const formatFrequency = (freq: number): string => {
  if (freq >= 1000) {
    return `${freq / 1000} GHz`
  } else if (freq >= 1) {
    return `${freq} MHz`
  } else {
    return `${freq * 1000} kHz`
  }
}
</script>
