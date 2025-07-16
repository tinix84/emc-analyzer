<!-- components/ResultsTable.vue -->
<template>
  <div class="bg-white rounded-lg shadow-lg p-6">
    <h2 class="text-2xl font-bold text-gray-800 mb-4">📋 Compliance Results</h2>
    
    <!-- Results Table -->
    <div v-if="hasResults" class="overflow-x-auto">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-gray-50">
          <tr>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Frequency (MHz)
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Measured (dBμV)
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Limit (dBμV)
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Margin (dB)
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Status
            </th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          <tr 
            v-for="result in complianceResults" 
            :key="result.frequency"
            :class="{
              'bg-red-50': !result.compliant,
              'bg-green-50': result.compliant && result.margin > 6,
              'bg-yellow-50': result.compliant && result.margin <= 6
            }"
          >
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
              {{ (result.frequency / 1000000).toFixed(3) }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
              {{ result.measured.toFixed(1) }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
              {{ result.limit.toFixed(1) }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
              {{ result.margin.toFixed(1) }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <span 
                :class="{
                  'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium': true,
                  'bg-green-100 text-green-800': result.compliant && result.margin > 6,
                  'bg-yellow-100 text-yellow-800': result.compliant && result.margin <= 6,
                  'bg-red-100 text-red-800': !result.compliant
                }"
              >
                {{ getStatusText(result) }}
              </span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    
    <!-- No Results State -->
    <div v-else class="text-center py-16 text-gray-500">
      <div class="text-6xl mb-4">📊</div>
      <p class="text-lg">No compliance results available</p>
      <p class="text-sm">Load a standard and measurement data to see compliance analysis</p>
    </div>
    
    <!-- Summary Statistics -->
    <div v-if="hasResults" class="mt-6 grid grid-cols-1 md:grid-cols-4 gap-4">
      <div class="bg-blue-50 rounded-lg p-4">
        <div class="text-2xl font-bold text-blue-600">{{ totalPoints }}</div>
        <div class="text-sm text-blue-600">Total Points</div>
      </div>
      
      <div class="bg-green-50 rounded-lg p-4">
        <div class="text-2xl font-bold text-green-600">{{ passedPoints }}</div>
        <div class="text-sm text-green-600">Passed</div>
      </div>
      
      <div class="bg-red-50 rounded-lg p-4">
        <div class="text-2xl font-bold text-red-600">{{ failedPoints }}</div>
        <div class="text-sm text-red-600">Failed</div>
      </div>
      
      <div class="bg-purple-50 rounded-lg p-4">
        <div class="text-2xl font-bold text-purple-600">{{ overallCompliance }}%</div>
        <div class="text-sm text-purple-600">Compliance</div>
      </div>
    </div>
    
    <!-- Export Controls -->
    <div v-if="hasResults" class="mt-6 flex gap-4">
      <button 
        @click="exportToCSV"
        class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
      >
        📊 Export CSV
      </button>
      
      <button 
        @click="exportToPDF"
        class="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors"
      >
        📄 Export PDF
      </button>
      
      <button 
        @click="generateReport"
        class="px-4 py-2 bg-purple-600 text-white rounded-lg hover:bg-purple-700 transition-colors"
      >
        📋 Generate Report
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ComplianceResult } from '~/types/emc'

interface ComplianceResultExtended extends ComplianceResult {
  frequency: number
  measured: number
  limit: number
  margin: number
  compliant: boolean
}

const emcStore = useEMCStore()

// Computed properties for compliance results
const complianceResults = computed((): ComplianceResultExtended[] => {
  if (!emcStore.measurementData || !emcStore.currentStandard) {
    return []
  }
  
  // Mock compliance calculation (replace with actual WASM call)
  return emcStore.measurementData.map(point => {
    // Calculate limit for this frequency (mock calculation)
    let limit = 40 // Base limit
    
    if (point.frequency < 500000) {
      limit = 60 - 20 * Math.log10(point.frequency / 150000)
    } else if (point.frequency < 5000000) {
      limit = 50
    } else {
      limit = 50 + 20 * Math.log10(point.frequency / 5000000)
    }
    
    const margin = limit - point.amplitude
    const compliant = point.amplitude <= limit
    
    return {
      frequency: point.frequency,
      measured: point.amplitude,
      limit,
      margin,
      compliant,
      measurementType: 'peak', // Default
      standardName: emcStore.currentStandard?.name || '',
      notes: compliant ? '' : 'Exceeds limit'
    }
  })
})

const hasResults = computed(() => complianceResults.value.length > 0)

const totalPoints = computed(() => complianceResults.value.length)

const passedPoints = computed(() => 
  complianceResults.value.filter(r => r.compliant).length
)

const failedPoints = computed(() => 
  complianceResults.value.filter(r => !r.compliant).length
)

const overallCompliance = computed(() => {
  if (totalPoints.value === 0) return 0
  return Math.round((passedPoints.value / totalPoints.value) * 100)
})

// Helper functions
const getStatusText = (result: ComplianceResultExtended): string => {
  if (!result.compliant) return 'FAIL'
  if (result.margin <= 6) return 'MARGINAL'
  return 'PASS'
}

const exportToCSV = () => {
  if (!hasResults.value) return
  
  const headers = ['Frequency (Hz)', 'Measured (dBμV)', 'Limit (dBμV)', 'Margin (dB)', 'Status']
  const rows = complianceResults.value.map(result => [
    result.frequency.toString(),
    result.measured.toFixed(1),
    result.limit.toFixed(1),
    result.margin.toFixed(1),
    getStatusText(result)
  ])
  
  const csvContent = [headers, ...rows]
    .map(row => row.join(','))
    .join('\n')
  
  downloadFile(csvContent, 'emc-compliance-results.csv', 'text/csv')
}

const exportToPDF = () => {
  // Mock PDF export (would need a PDF library like jsPDF)
  console.log('PDF export would be implemented here')
  alert('PDF export functionality would be implemented with a PDF library')
}

const generateReport = () => {
  // Mock report generation
  console.log('Report generation would be implemented here')
  alert('Report generation functionality would create a detailed compliance report')
}

const downloadFile = (content: string, filename: string, contentType: string) => {
  const blob = new Blob([content], { type: contentType })
  const url = URL.createObjectURL(blob)
  
  const link = document.createElement('a')
  link.href = url
  link.download = filename
  document.body.appendChild(link)
  link.click()
  document.body.removeChild(link)
  
  URL.revokeObjectURL(url)
}
</script>
