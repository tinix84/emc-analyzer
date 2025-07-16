<!-- // components/EMCFileUpload.vue -->
<template>
  <div class="bg-white rounded-xl p-6 shadow-lg">
    <h3 class="text-xl font-bold mb-4 text-gray-800">📁 Measurement Data Upload</h3>
    
    <div class="space-y-4">
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-2">
          Upload Spectrum Data (CSV):
        </label>
        <input 
          type="file"
          @change="handleFileUpload"
          accept=".csv,.txt"
          class="w-full p-3 border-2 border-gray-300 rounded-lg focus:border-blue-500 focus:outline-none transition-colors file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:text-sm file:font-semibold file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100"
        >
      </div>

      <div>
        <label class="block text-sm font-medium text-gray-700 mb-2">Measurement Type:</label>
        <select 
          v-model="measurementType"
          class="w-full p-3 border-2 border-gray-300 rounded-lg focus:border-blue-500 focus:outline-none transition-colors"
        >
          <option value="avg">Average</option>
          <option value="qp">Quasi-Peak</option>
          <option value="pk">Peak</option>
        </select>
      </div>

      <button 
        @click="analyzeCompliance"
        :disabled="!canAnalyze || emcStore.isLoading"
        class="w-full bg-gradient-to-r from-green-500 to-blue-600 text-white py-3 px-6 rounded-lg font-medium hover:from-green-600 hover:to-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200 transform hover:scale-105"
      >
        <span v-if="emcStore.isLoading">Analyzing...</span>
        <span v-else>Analyze Compliance</span>
      </button>

      <div class="p-4 bg-gray-50 rounded-lg text-sm text-gray-600">
        <strong>📝 Sample CSV format:</strong><br>
        <code class="font-mono">
          Frequency(Hz), Amplitude(dBµV)<br>
          150000, 45.2<br>
          500000, 40.5<br>
          1000000, 42.1
        </code>
      </div>
    </div>

    <div v-if="emcStore.measurementData" class="mt-4 p-4 bg-blue-50 border border-blue-200 text-blue-700 rounded-lg">
      ✅ Loaded {{ emcStore.measurementData.frequencies.length }} measurement points
    </div>
  </div>
</template>

<script setup lang="ts">
import type { SpectrumData } from '~/types/emc'

const emcStore = useEMCStore()
const measurementType = ref('avg')

const canAnalyze = computed(() => 
  emcStore.currentStandard && emcStore.measurementData
)

const handleFileUpload = (event: Event) => {
  const file = (event.target as HTMLInputElement).files?.[0]
  if (!file) return

  const reader = new FileReader()
  reader.onload = (e) => {
    try {
      const text = e.target?.result as string
      const data = parseCSV(text)
      emcStore.setMeasurementData(data)
    } catch (error) {
      console.error('Error parsing CSV:', error)
    }
  }
  
  reader.readAsText(file)
}

const parseCSV = (text: string): SpectrumData => {
  const lines = text.trim().split('\n')
  const frequencies: number[] = []
  const amplitudes: number[] = []
  
  const startIndex = lines[0].toLowerCase().includes('frequency') ? 1 : 0
  
  for (let i = startIndex; i < lines.length; i++) {
    const values = lines[i].split(',').map(v => v.trim())
    if (values.length >= 2 && !isNaN(Number(values[0])) && !isNaN(Number(values[1]))) {
      frequencies.push(Number(values[0]))
      amplitudes.push(Number(values[1]))
    }
  }
  
  return { frequencies, amplitudes }
}

const analyzeCompliance = async () => {
  try {
    await emcStore.analyzeCompliance(measurementType.value)
  } catch (error) {
    console.error('Failed to analyze compliance:', error)
  }
}
</script>
