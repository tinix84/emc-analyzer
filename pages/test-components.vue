<!-- test-components-only.vue -->
<template>
  <div class="min-h-screen bg-gradient-to-br from-blue-50 via-purple-50 to-pink-50">
    <div class="container mx-auto px-4 py-8">
      <h1 class="text-4xl font-bold text-center mb-8">🧪 GUI Components Test (No WASM)</h1>
      
      <!-- Mock Data Controls -->
      <div class="bg-white rounded-lg shadow-lg p-6 mb-8">
        <h2 class="text-2xl font-bold mb-4">📊 Mock Data Controls</h2>
        <div class="flex gap-4">
          <button 
            @click="loadMockStandard" 
            class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
          >
            Load Mock Standard
          </button>
          <button 
            @click="loadMockMeasurements" 
            class="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700"
          >
            Load Mock Measurements
          </button>
          <button 
            @click="clearData" 
            class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700"
          >
            Clear Data
          </button>
        </div>
      </div>

      <!-- Component Grid -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 mb-8">
        <!-- Standard Loader (Modified for testing) -->
        <div class="bg-white rounded-lg shadow-lg p-6">
          <h2 class="text-2xl font-bold text-gray-800 mb-4">🏷️ Standard Selection</h2>
          <div class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Standard</label>
              <select v-model="selectedStandard" class="w-full p-3 border border-gray-300 rounded-lg">
                <option value="">Select Standard</option>
                <option value="CISPR25">CISPR25</option>
                <option value="EN55032">EN55032</option>
                <option value="IEC61800-3">IEC61800-3</option>
              </select>
            </div>
            
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">EMC Class</label>
              <select v-model="selectedClass" class="w-full p-3 border border-gray-300 rounded-lg">
                <option value="">Select Class</option>
                <option value="Class3">Class 3</option>
                <option value="Class5">Class 5</option>
              </select>
            </div>
            
            <button 
              @click="loadStandardMock"
              :disabled="!selectedStandard || !selectedClass"
              class="w-full py-3 bg-purple-600 text-white rounded-lg hover:bg-purple-700 disabled:bg-gray-400"
            >
              Load Standard (Mock)
            </button>
          </div>
        </div>

        <!-- File Upload (Mock) -->
        <div class="bg-white rounded-lg shadow-lg p-6">
          <h2 class="text-2xl font-bold text-gray-800 mb-4">📁 File Upload</h2>
          <div class="border-2 border-dashed border-gray-300 rounded-lg p-8 text-center">
            <div class="text-4xl mb-4">📊</div>
            <p class="text-gray-600 mb-4">Mock file upload area</p>
            <button 
              @click="mockFileUpload"
              class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
            >
              Simulate File Upload
            </button>
          </div>
        </div>
      </div>

      <!-- Chart Component -->
      <div class="mb-8">
        <EMCChart 
          :standard="mockStandard" 
          :measurement-data="mockMeasurementData"
        />
      </div>

      <!-- Results Table -->
      <ResultsTable />
      
      <!-- Debug Info -->
      <div class="bg-gray-100 rounded-lg p-6 mt-8">
        <h3 class="text-xl font-bold mb-4">🔍 Debug Info</h3>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <h4 class="font-semibold">Current Standard:</h4>
            <pre class="text-sm bg-white p-2 rounded">{{ JSON.stringify(mockStandard, null, 2) }}</pre>
          </div>
          <div>
            <h4 class="font-semibold">Measurement Data:</h4>
            <pre class="text-sm bg-white p-2 rounded">{{ JSON.stringify(mockMeasurementData?.slice(0, 5), null, 2) }}</pre>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { EMCStandard } from '~/types/emc'

// Mock data states
const selectedStandard = ref('')
const selectedClass = ref('')
const mockStandard = ref<EMCStandard | null>(null)
const mockMeasurementData = ref<Array<{ frequency: number; amplitude: number }> | null>(null)

// Mock store simulation
const emcStore = {
  currentStandard: computed(() => mockStandard.value),
  measurementData: computed(() => mockMeasurementData.value),
}

// Provide the mock store
provide('emcStore', emcStore)

const loadMockStandard = () => {
  mockStandard.value = {
    name: "CISPR25 Class3 (Mock)",
    f_avg_limit_mask: [150000, 500000, 5000000, 30000000],
    dbuv_avg_limit_mask: [60, 50, 50, 70],
    f_qp_limit_mask: [150000, 500000, 5000000, 30000000],
    dbuv_qp_limit_mask: [66, 56, 56, 76],
    f_pk_limit_mask: [150000, 500000, 5000000, 30000000],
    dbuv_pk_limit_mask: [80, 70, 70, 90],
  }
}

const loadMockMeasurements = () => {
  // Generate some mock measurement data
  const data = []
  for (let f = 150000; f <= 30000000; f *= 1.5) {
    const amplitude = 45 + Math.random() * 30 + 5 * Math.sin(Math.log10(f))
    data.push({ frequency: f, amplitude })
  }
  mockMeasurementData.value = data
}

const clearData = () => {
  mockStandard.value = null
  mockMeasurementData.value = null
  selectedStandard.value = ''
  selectedClass.value = ''
}

const loadStandardMock = () => {
  mockStandard.value = {
    name: `${selectedStandard.value} ${selectedClass.value} (Mock)`,
    f_avg_limit_mask: [150000, 500000, 5000000, 30000000],
    dbuv_avg_limit_mask: selectedClass.value === 'Class5' ? [50, 40, 40, 60] : [60, 50, 50, 70],
    f_qp_limit_mask: [150000, 500000, 5000000, 30000000],
    dbuv_qp_limit_mask: selectedClass.value === 'Class5' ? [56, 46, 46, 66] : [66, 56, 56, 76],
    f_pk_limit_mask: [150000, 500000, 5000000, 30000000],
    dbuv_pk_limit_mask: selectedClass.value === 'Class5' ? [70, 60, 60, 80] : [80, 70, 70, 90],
  }
}

const mockFileUpload = () => {
  // Simulate file upload with some delay
  setTimeout(() => {
    loadMockMeasurements()
    alert('Mock file uploaded successfully!')
  }, 500)
}

// Auto-load some sample data
onMounted(() => {
  setTimeout(() => {
    loadMockStandard()
    loadMockMeasurements()
  }, 1000)
})

useHead({
  title: 'GUI Components Test - No WASM',
  meta: [
    { name: 'description', content: 'Testing Vue components without WASM dependency' }
  ]
})
</script>
