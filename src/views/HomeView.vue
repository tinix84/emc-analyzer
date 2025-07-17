<template>
  <div class="space-y-8">
    <!-- Header -->
    <div class="text-center">
      <h1 class="text-4xl font-bold text-gray-800 mb-4">
        📊 EMC Measurement Analyzer
      </h1>
      <p class="text-xl text-gray-600">
        Upload measurement files, select EMC standards, and visualize compliance data
      </p>
    </div>

    <!-- Upload Section -->
    <div class="bg-white rounded-lg shadow-md p-6">
      <h2 class="text-2xl font-semibold mb-4 flex items-center">
        📁 Upload Measurement Files
      </h2>
      <FileUpload @files-uploaded="handleFilesUploaded" />
      
      <!-- Sample Files -->
      <div class="mt-4 p-4 bg-gray-50 rounded-lg">
        <h3 class="text-sm font-medium text-gray-700 mb-2">🧪 Test with Sample Files:</h3>
        <div class="flex gap-2">
          <a 
            href="/sample-measurement.csv" 
            download 
            class="px-3 py-1 bg-blue-600 text-white rounded text-sm hover:bg-blue-700 transition-colors"
          >
            📄 Download CSV Sample
          </a>
          <a 
            href="/sample-measurement.txt" 
            download 
            class="px-3 py-1 bg-green-600 text-white rounded text-sm hover:bg-green-700 transition-colors"
          >
            📄 Download TXT Sample
          </a>
        </div>
        <p class="text-xs text-gray-500 mt-2">
          Download these files to test the upload functionality
        </p>
      </div>
    </div>

    <!-- Standard Selection -->
    <div class="bg-white rounded-lg shadow-md p-6">
      <h2 class="text-2xl font-semibold mb-4 flex items-center">
        📋 Select EMC Standard
      </h2>
      <StandardSelector 
        :selected-standard="selectedStandard"
        @standard-changed="handleStandardChanged" 
      />
    </div>

    <!-- Visualization -->
    <div class="bg-white rounded-lg shadow-md p-6" v-if="measurementData.length > 0">
      <h2 class="text-2xl font-semibold mb-4 flex items-center">
        📈 Measurement Visualization
      </h2>
      <SemiLogChart 
        :measurement-data="measurementData"
        :standard-mask="standardMask"
      />
    </div>

    <!-- Data Summary -->
    <div class="bg-white rounded-lg shadow-md p-6" v-if="measurementData.length > 0">
      <h2 class="text-2xl font-semibold mb-4">
        📊 Data Summary
      </h2>
      <DataSummary :data="measurementData" :standard="selectedStandard" />
    </div>

    <!-- Storage Info -->
    <div class="bg-blue-50 border border-blue-200 rounded-lg p-4 text-sm">
      <h3 class="font-semibold text-blue-800 mb-2">💾 Data Storage Information</h3>
      <div class="space-y-1 text-blue-700">
        <div><strong>Current Data:</strong> {{ measurementData.length }} measurement points</div>
        <div><strong>Storage:</strong> Browser localStorage (persists between sessions)</div>
        <div><strong>Standard:</strong> {{ selectedStandard || 'Not selected' }}</div>
        <div class="mt-2">
          <button 
            @click="clearAllData"
            class="px-3 py-1 bg-red-600 text-white rounded text-xs hover:bg-red-700 transition-colors"
            v-if="measurementData.length > 0"
          >
            🗑️ Clear All Data
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useEMCStore } from '@/stores/emcStore'
import FileUpload from '@/components/FileUpload.vue'
import StandardSelector from '@/components/StandardSelector.vue'
import SemiLogChart from '@/components/SemiLogChart.vue'
import DataSummary from '@/components/DataSummary.vue'

const emcStore = useEMCStore()

const selectedStandard = ref('')
const measurementData = ref<Array<{frequency: number, amplitude: number}>>([])

const standardMask = computed(() => {
  return emcStore.getStandardMask(selectedStandard.value)
})

const handleFilesUploaded = (files: File[]) => {
  console.log('Files uploaded:', files)
  // Process uploaded files
  files.forEach(file => {
    if (file.name.endsWith('.csv') || file.name.endsWith('.txt')) {
      processFile(file)
    }
  })
}

const handleStandardChanged = (standard: string) => {
  selectedStandard.value = standard
  console.log('Standard selected:', standard)
}

const clearAllData = () => {
  if (confirm('🗑️ Are you sure you want to clear all measurement data and settings?\n\nThis action cannot be undone.')) {
    measurementData.value = []
    selectedStandard.value = ''
    emcStore.clearData()
    console.log('🗑️ All data cleared by user')
  }
}

const processFile = async (file: File) => {
  try {
    console.log(`📄 Processing file: ${file.name} (${file.size} bytes)`)
    const text = await file.text()
    console.log('📝 File content preview:', text.substring(0, 200) + '...')
    
    const data = parseCSV(text)
    console.log(`✅ Parsed ${data.length} data points from ${file.name}`)
    
    if (data.length === 0) {
      alert(`⚠️ No valid data found in ${file.name}\n\nExpected format:\nfrequency,amplitude\n150,45.2\n200,52.1\n...`)
      return
    }
    
    measurementData.value = data
    emcStore.setMeasurementData(data)
    
    console.log('📊 Sample data points:', data.slice(0, 3))
  } catch (error) {
    console.error('❌ Error processing file:', error)
    alert(`❌ Error processing ${file.name}:\n${error}`)
  }
}

const parseCSV = (text: string): Array<{frequency: number, amplitude: number}> => {
  console.log('🔍 Parsing CSV data...')
  const lines = text.split('\n').filter(line => line.trim())
  const data: Array<{frequency: number, amplitude: number}> = []
  
  console.log(`📝 Found ${lines.length} lines`)
  
  // Skip header if exists
  const firstLine = lines[0]?.toLowerCase() || ''
  const hasHeader = firstLine.includes('frequency') || 
                   firstLine.includes('freq') || 
                   firstLine.includes('hz')
  
  const startIndex = hasHeader ? 1 : 0
  console.log(`📋 ${hasHeader ? 'Header detected, starting from line 2' : 'No header, starting from line 1'}`)
  
  for (let i = startIndex; i < lines.length; i++) {
    const line = lines[i].trim()
    if (!line) continue
    
    // Try different separators: comma, semicolon, tab, space
    let columns = line.split(/[,;\t]/)
    if (columns.length < 2) {
      columns = line.split(/\s+/) // space separated
    }
    
    if (columns.length >= 2) {
      const frequency = parseFloat(columns[0].trim())
      const amplitude = parseFloat(columns[1].trim())
      
      if (!isNaN(frequency) && !isNaN(amplitude)) {
        data.push({ frequency, amplitude })
      } else {
        console.warn(`⚠️ Line ${i + 1}: Invalid data - freq: "${columns[0]}", amp: "${columns[1]}"`)
      }
    } else {
      console.warn(`⚠️ Line ${i + 1}: Not enough columns - "${line}"`)
    }
  }
  
  console.log(`✅ Successfully parsed ${data.length} valid data points`)
  if (data.length > 0) {
    console.log('📊 Data range:', {
      freqMin: Math.min(...data.map(d => d.frequency)),
      freqMax: Math.max(...data.map(d => d.frequency)),
      ampMin: Math.min(...data.map(d => d.amplitude)),
      ampMax: Math.max(...data.map(d => d.amplitude))
    })
  }
  
  return data
}
</script>
