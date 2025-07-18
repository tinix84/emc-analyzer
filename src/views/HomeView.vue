<template>
  <div class="space-y-8">
    <!-- Header -->
    <div class="text-center">
      <h1 class="text-4xl font-bold text-white mb-4">
        ⚡ EMC Power Analyzer
      </h1>
      <p class="text-xl text-gray-300">
        Upload measurement files, select EMC standards, and visualize compliance data
      </p>
    </div>

    <!-- Upload Section -->
    <div class="bg-gray-800 border border-gray-700 rounded-lg p-6">
      <h2 class="text-2xl font-semibold mb-4 flex items-center text-white">
        📁 Upload Measurement Files
      </h2>
      <FileUpload @files-uploaded="handleFilesUploaded" />
      
      <!-- Sample Files -->
      <div class="mt-4 p-4 bg-gray-700 rounded-lg border border-gray-600">
        <h3 class="text-sm font-medium text-white mb-2">🧪 Test with Sample Files:</h3>
        <div class="flex gap-2">
          <a 
            href="/sample-measurement.csv" 
            download 
            class="px-3 py-1 bg-purple-600 text-white rounded text-sm hover:bg-purple-700 transition-colors"
          >
            📄 Download CSV Sample
          </a>
          <a 
            href="/sample-measurement.txt" 
            download 
            class="px-3 py-1 bg-purple-600 text-white rounded text-sm hover:bg-purple-700 transition-colors"
          >
            📄 Download TXT Sample
          </a>
        </div>
        <p class="text-xs text-gray-300 mt-2">
          Download these 3-column format files (Frequency, Peak, Avg) to test the upload functionality
        </p>
      </div>
    </div>

    <!-- Standard Selection -->
    <div class="bg-gray-800 border border-gray-700 rounded-lg p-6">
      <h2 class="text-2xl font-semibold mb-4 flex items-center text-white">
        📋 Select EMC Standard
      </h2>
      <StandardSelector 
        :selected-standard="selectedStandard"
        @standard-changed="handleStandardChanged" 
      />
    </div>

    <!-- Visualization -->
    <div class="bg-gray-800 border border-gray-700 rounded-lg p-6" v-if="measurementData.length > 0 || selectedStandard">
      <h2 class="text-2xl font-semibold mb-4 flex items-center text-white">
        📈 {{ measurementData.length > 0 ? 'Measurement Visualization' : 'EMC Standard Limits' }}
      </h2>
      <SemiLogChart 
        :measurement-data="measurementData"
        :standard-masks="standardMasks"
      />
    </div>

    <!-- Data Summary -->
    <div class="bg-gray-800 border border-gray-700 rounded-lg p-6" v-if="measurementData.length > 0">
      <h2 class="text-2xl font-semibold mb-4 text-white">
        📊 Data Summary
      </h2>
      <DataSummary :data="measurementData" :standard="selectedStandard" />
    </div>

    <!-- Storage Info -->
    <div class="bg-gray-800 border border-gray-700 rounded-lg p-4 text-sm">
      <h3 class="font-semibold text-white mb-2">💾 Data Storage Information</h3>
      <div class="space-y-1 text-gray-300">
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
import { ref, computed, watch } from 'vue'
import { useEMCStore } from '@/stores/emcStore'
import FileUpload from '@/components/FileUpload.vue'
import StandardSelector from '@/components/StandardSelector.vue'
import SemiLogChart from '@/components/SemiLogChart.vue'
import DataSummary from '@/components/DataSummary.vue'

const emcStore = useEMCStore()

const selectedStandard = ref('')
const measurementData = ref<Array<{frequency: number, amplitude: number}>>([])
const standardMasks = ref<{ [key: string]: Array<{frequency: number, amplitude: number}> }>({})

// Load multiple masks when standard changes
watch(selectedStandard, async (newStandard) => {
  if (newStandard) {
    try {
      const masks = await emcStore.getStandardMasks(newStandard)
      standardMasks.value = masks
      console.log('📊 Loaded masks for', newStandard, ':', Object.keys(masks))
    } catch (error) {
      console.error('❌ Error loading masks:', error)
      standardMasks.value = {}
    }
  } else {
    standardMasks.value = {}
  }
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
  console.log('🎯 Standard changed from', selectedStandard.value, 'to', standard)
  selectedStandard.value = standard
  console.log('📊 Loading multiple masks for standard:', standard)
}

const clearAllData = () => {
  if (confirm('🗑️ Are you sure you want to clear all measurement data and settings?\n\nThis action cannot be undone.')) {
    measurementData.value = []
    selectedStandard.value = ''
    emcStore.clearData()
    console.log('🗑️ All data cleared')
  }
}

const processFile = async (file: File) => {
  try {
    console.log(`📂 Processing file: ${file.name} (${file.size} bytes)`)
    const text = await file.text()
    console.log('📝 File content preview:', text.substring(0, 200) + '...')
    
    const data = parseCSV(text)
    console.log(`✅ Parsed ${data.length} data points from ${file.name}`)
    
    if (data.length === 0) {
      alert(`⚠️ No valid data found in ${file.name}\n\nSupported formats:\n\n🔹 3-column format:\nFrequency,Peak,Avg\n[MHz],[dBuV],[dBuV]\n150,45.2,42.1\n\n🔹 2-column format:\nfrequency,amplitude\n150,45.2\n200,52.1`)
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

const parseCSV = (text: string): Array<{frequency: number, amplitude: number, peak?: number, avg?: number}> => {
  console.log('🔍 Parsing CSV data...')
  const lines = text.split('\n').filter(line => line.trim())
  const data: Array<{frequency: number, amplitude: number, peak?: number, avg?: number}> = []
  
  console.log(`📝 Found ${lines.length} lines`)
  
  // Skip header if exists
  let headerLine = ''
  let unitLine = ''
  let hasHeader = false
  let startIndex = 0
  
  // Check for header patterns
  for (let i = 0; i < Math.min(3, lines.length); i++) {
    const line = lines[i].toLowerCase()
    if (line.includes('frequency') || line.includes('peak') || line.includes('avg')) {
      headerLine = lines[i]
      hasHeader = true
      startIndex = i + 1
      break
    }
  }
  
  // Check for unit line after header
  if (hasHeader && startIndex < lines.length) {
    const nextLine = lines[startIndex].toLowerCase()
    if (nextLine.includes('[mhz]') || nextLine.includes('[dbuv]') || nextLine.includes('mhz') || nextLine.includes('dbuv')) {
      unitLine = lines[startIndex]
      startIndex++
    }
  }
  
  console.log(`📋 ${hasHeader ? `Header detected: "${headerLine}"` : 'No header detected'}`)
  if (unitLine) {
    console.log(`📏 Units detected: "${unitLine}"`)
  }
  
  // Determine column format
  let isThreeColumnFormat = false
  if (hasHeader) {
    const headerLower = headerLine.toLowerCase()
    isThreeColumnFormat = headerLower.includes('peak') && headerLower.includes('avg')
  }
  
  console.log(`📊 Format: ${isThreeColumnFormat ? '3-column (Frequency, Peak, Avg)' : '2-column (Frequency, Amplitude)'}`)
  
  for (let i = startIndex; i < lines.length; i++) {
    const line = lines[i].trim()
    if (!line) continue
    
    // Try different separators: comma, semicolon, tab, space
    let columns = line.split(/[,;\t]/)
    if (columns.length < 2) {
      columns = line.split(/\s+/) // space separated
    }
    
    if (isThreeColumnFormat && columns.length >= 3) {
      // 3-column format: Frequency, Peak, Avg
      const frequency = parseFloat(columns[0].trim())
      const peak = parseFloat(columns[1].trim())
      const avg = parseFloat(columns[2].trim())
      
      if (!isNaN(frequency) && !isNaN(peak) && !isNaN(avg)) {
        data.push({ 
          frequency, 
          amplitude: peak, // Use peak as default amplitude for backward compatibility
          peak,
          avg
        })
      } else {
        console.warn(`⚠️ Line ${i + 1}: Invalid 3-column data - freq: "${columns[0]}", peak: "${columns[1]}", avg: "${columns[2]}"`)
      }
    } else if (columns.length >= 2) {
      // 2-column format: Frequency, Amplitude (backward compatibility)
      const frequency = parseFloat(columns[0].trim())
      const amplitude = parseFloat(columns[1].trim())
      
      if (!isNaN(frequency) && !isNaN(amplitude)) {
        data.push({ frequency, amplitude })
      } else {
        console.warn(`⚠️ Line ${i + 1}: Invalid 2-column data - freq: "${columns[0]}", amp: "${columns[1]}"`)
      }
    } else {
      console.warn(`⚠️ Line ${i + 1}: Not enough columns - "${line}"`)
    }
  }
  
  console.log(`✅ Successfully parsed ${data.length} valid data points`)
  if (data.length > 0) {
    const stats: any = {
      freqMin: Math.min(...data.map(d => d.frequency)),
      freqMax: Math.max(...data.map(d => d.frequency)),
      ampMin: Math.min(...data.map(d => d.amplitude)),
      ampMax: Math.max(...data.map(d => d.amplitude))
    }
    
    if (isThreeColumnFormat) {
      stats.peakMin = Math.min(...data.map(d => d.peak || 0))
      stats.peakMax = Math.max(...data.map(d => d.peak || 0))
      stats.avgMin = Math.min(...data.map(d => d.avg || 0))
      stats.avgMax = Math.max(...data.map(d => d.avg || 0))
    }
    
    console.log('📊 Data range:', stats)
  }
  
  return data
}
</script>
