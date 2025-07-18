<template>
  <div class="space-y-6">
    <!-- Compliance Status -->
    <div v-if="complianceStatus" class="space-y-4">
      <!-- Overall Status -->
      <div class="grid md:grid-cols-3 gap-4">
        <div 
          class="p-4 rounded-lg border-2"
          :class="{
            'border-green-500 bg-green-50': complianceStatus.isCompliant,
            'border-red-500 bg-red-50': !complianceStatus.isCompliant
          }"
        >
          <div class="text-center">
            <div class="text-3xl mb-2">
              {{ complianceStatus.isCompliant ? '✅' : '❌' }}
            </div>
            <div class="font-semibold text-lg">
              {{ complianceStatus.isCompliant ? 'COMPLIANT' : 'NON-COMPLIANT' }}
            </div>
            <div class="text-sm text-gray-600 mt-1">
              Overall Status
            </div>
          </div>
        </div>

        <div class="p-4 bg-gray-50 rounded-lg">
          <div class="text-center">
            <div class="text-3xl mb-2">🔍</div>
            <div class="font-semibold text-lg">{{ complianceStatus.violations }}</div>
            <div class="text-sm text-gray-600">
              Total Violations
            </div>
          </div>
        </div>

        <div class="p-4 bg-gray-50 rounded-lg">
          <div class="text-center">
            <div class="text-3xl mb-2">📊</div>
            <div class="font-semibold text-lg">{{ complianceStatus.totalPoints }}</div>
            <div class="text-sm text-gray-600">
              Total Data Points
            </div>
          </div>
        </div>
      </div>

      <!-- Detailed Status for 3-column data -->
      <div v-if="complianceStatus.hasThreeColumns" class="grid md:grid-cols-2 gap-4">
        <div class="p-4 bg-red-50 rounded-lg border border-red-200">
          <div class="text-center">
            <div class="text-2xl mb-2">🔴</div>
            <div class="font-semibold text-lg text-red-800">{{ complianceStatus.peakViolations }}</div>
            <div class="text-sm text-red-600">
              Peak Violations
            </div>
          </div>
        </div>

        <div class="p-4 bg-green-50 rounded-lg border border-green-200">
          <div class="text-center">
            <div class="text-2xl mb-2">🟢</div>
            <div class="font-semibold text-lg text-green-800">{{ complianceStatus.avgViolations }}</div>
            <div class="text-sm text-green-600">
              Average Violations
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Statistical Analysis -->
    <div class="grid md:grid-cols-2 gap-6">
      <!-- Frequency Analysis -->
      <div class="bg-white border rounded-lg p-4">
        <h3 class="text-lg font-semibold mb-4 flex items-center">
          📈 Frequency Analysis
        </h3>
        <div class="space-y-3">
          <div class="flex justify-between">
            <span class="text-gray-600">Frequency Range:</span>
            <span class="font-medium">
              {{ formatFrequency(freqStats.min) }} - {{ formatFrequency(freqStats.max) }}
            </span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">Frequency Span:</span>
            <span class="font-medium">{{ formatFrequency(freqStats.span) }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">Data Density:</span>
            <span class="font-medium">{{ freqStats.density.toFixed(2) }} points/MHz</span>
          </div>
        </div>
      </div>

      <!-- Amplitude Analysis -->
      <div class="bg-white border rounded-lg p-4">
        <h3 class="text-lg font-semibold mb-4 flex items-center">
          📊 Amplitude Analysis
        </h3>
        <div class="space-y-3">
          <div v-if="amplitudeStats.hasThreeColumns">
            <!-- 3-column data stats -->
            <div class="mb-4">
              <h4 class="font-medium text-gray-700 mb-2">Peak Values</h4>
              <div class="space-y-2 text-sm">
                <div class="flex justify-between">
                  <span class="text-gray-600">Maximum:</span>
                  <span class="font-medium">{{ amplitudeStats.peak?.max.toFixed(2) }} dBμV</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-600">Minimum:</span>
                  <span class="font-medium">{{ amplitudeStats.peak?.min.toFixed(2) }} dBμV</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-600">Average:</span>
                  <span class="font-medium">{{ amplitudeStats.peak?.avg.toFixed(2) }} dBμV</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-600">Std Deviation:</span>
                  <span class="font-medium">{{ amplitudeStats.peak?.std.toFixed(2) }} dB</span>
                </div>
              </div>
            </div>
            <div>
              <h4 class="font-medium text-gray-700 mb-2">Average Values</h4>
              <div class="space-y-2 text-sm">
                <div class="flex justify-between">
                  <span class="text-gray-600">Maximum:</span>
                  <span class="font-medium">{{ amplitudeStats.average?.max.toFixed(2) }} dBμV</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-600">Minimum:</span>
                  <span class="font-medium">{{ amplitudeStats.average?.min.toFixed(2) }} dBμV</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-600">Average:</span>
                  <span class="font-medium">{{ amplitudeStats.average?.avg.toFixed(2) }} dBμV</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-600">Std Deviation:</span>
                  <span class="font-medium">{{ amplitudeStats.average?.std.toFixed(2) }} dB</span>
                </div>
              </div>
            </div>
          </div>
          <div v-else>
            <!-- 2-column data stats -->
            <div class="flex justify-between">
              <span class="text-gray-600">Maximum:</span>
              <span class="font-medium">{{ amplitudeStats.max?.toFixed(2) }} dBμV</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-600">Minimum:</span>
              <span class="font-medium">{{ amplitudeStats.min?.toFixed(2) }} dBμV</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-600">Average:</span>
              <span class="font-medium">{{ amplitudeStats.avg?.toFixed(2) }} dBμV</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-600">Std Deviation:</span>
              <span class="font-medium">{{ amplitudeStats.std?.toFixed(2) }} dB</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Violations Detail -->
    <div v-if="violations.combined.length > 0" class="bg-red-50 border border-red-200 rounded-lg p-4">
      <h3 class="text-lg font-semibold text-red-800 mb-4 flex items-center">
        ⚠️ Compliance Violations
      </h3>
      
      <!-- Separate tables for 3-column data -->
      <div v-if="violations.hasThreeColumns" class="space-y-6">
        <!-- Peak Violations -->
        <div v-if="violations.peak.length > 0">
          <h4 class="font-medium text-red-700 mb-2">🔴 Peak Violations ({{ violations.peak.length }})</h4>
          <div class="overflow-x-auto">
            <table class="min-w-full text-sm">
              <thead>
                <tr class="bg-red-100">
                  <th class="px-3 py-2 text-left text-red-800">Frequency</th>
                  <th class="px-3 py-2 text-left text-red-800">Peak Value</th>
                  <th class="px-3 py-2 text-left text-red-800">Limit</th>
                  <th class="px-3 py-2 text-left text-red-800">Violation</th>
                </tr>
              </thead>
              <tbody>
                <tr 
                  v-for="(violation, index) in violations.peak.slice(0, 5)"
                  :key="index"
                  class="border-b border-red-200"
                >
                  <td class="px-3 py-2 text-red-700">{{ formatFrequency(violation.frequency) }}</td>
                  <td class="px-3 py-2 text-red-700 font-medium">{{ violation.measured.toFixed(2) }} dBμV</td>
                  <td class="px-3 py-2 text-red-700">{{ violation.limit.toFixed(2) }} dBμV</td>
                  <td class="px-3 py-2 text-red-700 font-medium">
                    +{{ violation.excess.toFixed(2) }} dB
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
        
        <!-- Average Violations -->
        <div v-if="violations.average.length > 0">
          <h4 class="font-medium text-red-700 mb-2">🟢 Average Violations ({{ violations.average.length }})</h4>
          <div class="overflow-x-auto">
            <table class="min-w-full text-sm">
              <thead>
                <tr class="bg-red-100">
                  <th class="px-3 py-2 text-left text-red-800">Frequency</th>
                  <th class="px-3 py-2 text-left text-red-800">Average Value</th>
                  <th class="px-3 py-2 text-left text-red-800">Limit</th>
                  <th class="px-3 py-2 text-left text-red-800">Violation</th>
                </tr>
              </thead>
              <tbody>
                <tr 
                  v-for="(violation, index) in violations.average.slice(0, 5)"
                  :key="index"
                  class="border-b border-red-200"
                >
                  <td class="px-3 py-2 text-red-700">{{ formatFrequency(violation.frequency) }}</td>
                  <td class="px-3 py-2 text-red-700 font-medium">{{ violation.measured.toFixed(2) }} dBμV</td>
                  <td class="px-3 py-2 text-red-700">{{ violation.limit.toFixed(2) }} dBμV</td>
                  <td class="px-3 py-2 text-red-700 font-medium">
                    +{{ violation.excess.toFixed(2) }} dB
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
      
      <!-- Combined table for 2-column data -->
      <div v-else class="overflow-x-auto">
        <table class="min-w-full text-sm">
          <thead>
            <tr class="bg-red-100">
              <th class="px-3 py-2 text-left text-red-800">Frequency</th>
              <th class="px-3 py-2 text-left text-red-800">Measured</th>
              <th class="px-3 py-2 text-left text-red-800">Limit</th>
              <th class="px-3 py-2 text-left text-red-800">Violation</th>
            </tr>
          </thead>
          <tbody>
            <tr 
              v-for="(violation, index) in violations.combined.slice(0, 10)"
              :key="index"
              class="border-b border-red-200"
            >
              <td class="px-3 py-2 text-red-700">{{ formatFrequency(violation.frequency) }}</td>
              <td class="px-3 py-2 text-red-700 font-medium">{{ violation.measured.toFixed(2) }} dBμV</td>
              <td class="px-3 py-2 text-red-700">{{ violation.limit.toFixed(2) }} dBμV</td>
              <td class="px-3 py-2 text-red-700 font-medium">
                +{{ violation.excess.toFixed(2) }} dB
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      
      <div v-if="violations.combined.length > 10" class="text-sm text-red-600 mt-2">
        Showing {{ violations.hasThreeColumns ? '5' : '10' }} of {{ violations.combined.length }} violations. 
        <button @click="showAllViolations = !showAllViolations" class="underline">
          {{ showAllViolations ? 'Show less' : 'Show all' }}
        </button>
      </div>
    </div>

    <!-- Export Options -->
    <div class="flex gap-4">
      <button
        @click="exportSummary"
        class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors duration-200"
      >
        📄 Export Summary
      </button>
      <button
        @click="exportViolations"
        v-if="violations.combined.length > 0"
        class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors duration-200"
      >
        ⚠️ Export Violations
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useEMCStore } from '../stores/emcStore'

const props = defineProps<{
  data: Array<{frequency: number, amplitude: number, peak?: number, avg?: number}>
  standard: string
}>()

const emcStore = useEMCStore()
const showAllViolations = ref(false)

// Get masks for current standard
const currentMasks = ref<{ [key: string]: Array<{frequency: number, amplitude: number}> } | null>(null)

// Load masks when standard changes
watch(() => emcStore.currentStandard, async (newStandard) => {
  if (newStandard) {
    try {
      currentMasks.value = await emcStore.getStandardMasks(newStandard.id)
      console.log('📊 DataSummary: Loaded masks for', newStandard.name, currentMasks.value)
    } catch (error) {
      console.error('❌ DataSummary: Error loading masks:', error)
      currentMasks.value = null
    }
  } else {
    currentMasks.value = null
  }
}, { immediate: true })

// Helper function to interpolate limit from mask
const interpolateLimit = (frequency: number, mask: Array<{frequency: number, amplitude: number}>) => {
  if (!mask || mask.length === 0) return null
  
  // Find the two closest points for interpolation
  let i = 0
  while (i < mask.length - 1 && mask[i].frequency < frequency) {
    i++
  }
  
  if (i === 0) {
    // Frequency is below the first point
    return mask[0].amplitude
  } else if (i === mask.length - 1 && mask[i].frequency < frequency) {
    // Frequency is above the last point
    return mask[mask.length - 1].amplitude
  } else {
    // Interpolate between two points
    const p1 = mask[i - 1]
    const p2 = mask[i]
    const ratio = (frequency - p1.frequency) / (p2.frequency - p1.frequency)
    return p1.amplitude + ratio * (p2.amplitude - p1.amplitude)
  }
}

const complianceStatus = computed(() => emcStore.complianceStatus)

const freqStats = computed(() => {
  if (props.data.length === 0) return { min: 0, max: 0, span: 0, density: 0 }
  
  const frequencies = props.data.map(d => d.frequency)
  const min = Math.min(...frequencies)
  const max = Math.max(...frequencies)
  const span = max - min
  const density = span > 0 ? props.data.length / span : 0
  
  return { min, max, span, density }
})

const amplitudeStats = computed(() => {
  if (props.data.length === 0) return { min: 0, max: 0, avg: 0, std: 0 }
  
  // Check if we have 3-column data
  const hasThreeColumns = props.data.some(point => 
    point.peak !== undefined && point.avg !== undefined
  )
  
  if (hasThreeColumns) {
    // Separate stats for Peak and Average
    const peaks = props.data.map(d => d.peak || d.amplitude)
    const avgs = props.data.map(d => d.avg || d.amplitude)
    
    return {
      peak: {
        min: Math.min(...peaks),
        max: Math.max(...peaks),
        avg: peaks.reduce((sum, val) => sum + val, 0) / peaks.length,
        std: Math.sqrt(peaks.reduce((sum, val) => sum + Math.pow(val - (peaks.reduce((s, v) => s + v, 0) / peaks.length), 2), 0) / peaks.length)
      },
      average: {
        min: Math.min(...avgs),
        max: Math.max(...avgs),
        avg: avgs.reduce((sum, val) => sum + val, 0) / avgs.length,
        std: Math.sqrt(avgs.reduce((sum, val) => sum + Math.pow(val - (avgs.reduce((s, v) => s + v, 0) / avgs.length), 2), 0) / avgs.length)
      },
      hasThreeColumns: true
    }
  } else {
    // Standard stats for 2-column data
    const amplitudes = props.data.map(d => d.amplitude)
    const min = Math.min(...amplitudes)
    const max = Math.max(...amplitudes)
    const avg = amplitudes.reduce((sum, val) => sum + val, 0) / amplitudes.length
    
    const variance = amplitudes.reduce((sum, val) => sum + Math.pow(val - avg, 2), 0) / amplitudes.length
    const std = Math.sqrt(variance)
    
    return { min, max, avg, std, hasThreeColumns: false }
  }
})

const violations = computed(() => {
  if (!emcStore.currentStandard || props.data.length === 0) {
    return { peak: [], average: [], combined: [], hasThreeColumns: false }
  }
  
  const hasThreeColumns = props.data.some(point => 
    point.peak !== undefined && point.avg !== undefined
  )
  
  if (hasThreeColumns) {
    // For 3-column data, use PK mask for Peak and AVG mask for Average
    const peakViolations: Array<{
      frequency: number
      measured: number
      limit: number
      excess: number
      type: string
    }> = []
    
    const avgViolations: Array<{
      frequency: number
      measured: number
      limit: number
      excess: number
      type: string
    }> = []
    
    if (currentMasks.value) {
      // Use interpolated masks for proper comparison
      const pkMask = currentMasks.value.pk || currentMasks.value.PK
      const avgMask = currentMasks.value.avg || currentMasks.value.AVG
      
      console.log('📊 DataSummary: Using masks for violation analysis')
      console.log('  - PK mask available:', !!pkMask, pkMask?.length, 'points')
      console.log('  - AVG mask available:', !!avgMask, avgMask?.length, 'points')
      
      props.data.forEach((point, index) => {
        // Check Peak violations against PK mask
        if (pkMask && point.peak !== undefined) {
          const peakLimit = interpolateLimit(point.frequency, pkMask)
          console.log(`  Point ${index}: Freq=${point.frequency}MHz, Peak=${point.peak}dBuV, PK_limit=${peakLimit?.toFixed(2)}dBuV`)
          if (peakLimit !== null && point.peak > peakLimit) {
            console.log(`    ⚠️ Peak violation: ${point.peak} > ${peakLimit.toFixed(2)} (+${(point.peak - peakLimit).toFixed(2)}dB)`)
            peakViolations.push({
              frequency: point.frequency,
              measured: point.peak,
              limit: peakLimit,
              excess: point.peak - peakLimit,
              type: 'Peak'
            })
          }
        }
        
        // Check Average violations against AVG mask
        if (avgMask && point.avg !== undefined) {
          const avgLimit = interpolateLimit(point.frequency, avgMask)
          console.log(`  Point ${index}: Freq=${point.frequency}MHz, Avg=${point.avg}dBuV, AVG_limit=${avgLimit?.toFixed(2)}dBuV`)
          if (avgLimit !== null && point.avg > avgLimit) {
            console.log(`    ⚠️ Average violation: ${point.avg} > ${avgLimit.toFixed(2)} (+${(point.avg - avgLimit).toFixed(2)}dB)`)
            avgViolations.push({
              frequency: point.frequency,
              measured: point.avg,
              limit: avgLimit,
              excess: point.avg - avgLimit,
              type: 'Average'
            })
          }
        }
      })
    } else {
      // Fallback to frequency ranges if masks not available
      const standard = emcStore.currentStandard
      props.data.forEach(point => {
        const applicableRange = standard.frequencyRanges.find(
          range => point.frequency >= range.startFreq && point.frequency <= range.endFreq
        )
        
        if (applicableRange) {
          // Check Peak violations
          const peakValue = point.peak || point.amplitude
          if (peakValue > applicableRange.limit) {
            peakViolations.push({
              frequency: point.frequency,
              measured: peakValue,
              limit: applicableRange.limit,
              excess: peakValue - applicableRange.limit,
              type: 'Peak'
            })
          }
          
          // Check Average violations
          const avgValue = point.avg || point.amplitude
          if (avgValue > applicableRange.limit) {
            avgViolations.push({
              frequency: point.frequency,
              measured: avgValue,
              limit: applicableRange.limit,
              excess: avgValue - applicableRange.limit,
              type: 'Average'
            })
          }
        }
      })
    }
    
    // Combine both violation types - this ensures we get BOTH Peak and Average violations
    const combined = [...peakViolations, ...avgViolations].sort((a, b) => b.excess - a.excess)
    
    return {
      peak: peakViolations.sort((a, b) => b.excess - a.excess),
      average: avgViolations.sort((a, b) => b.excess - a.excess),
      combined,
      hasThreeColumns: true
    }
  } else {
    // Standard violations for 2-column data
    const violationsList: Array<{
      frequency: number
      measured: number
      limit: number
      excess: number
      type: string
    }> = []
    
    if (currentMasks.value) {
      // Use PK mask as default for 2-column data
      const defaultMask = currentMasks.value.pk || currentMasks.value.PK || currentMasks.value.avg || currentMasks.value.AVG
      
      if (defaultMask) {
        props.data.forEach(point => {
          const limit = interpolateLimit(point.frequency, defaultMask)
          if (limit !== null && point.amplitude > limit) {
            violationsList.push({
              frequency: point.frequency,
              measured: point.amplitude,
              limit: limit,
              excess: point.amplitude - limit,
              type: 'Measurement'
            })
          }
        })
      }
    } else {
      // Fallback to frequency ranges
      const standard = emcStore.currentStandard
      props.data.forEach(point => {
        const applicableRange = standard.frequencyRanges.find(
          range => point.frequency >= range.startFreq && point.frequency <= range.endFreq
        )
        
        if (applicableRange && point.amplitude > applicableRange.limit) {
          violationsList.push({
            frequency: point.frequency,
            measured: point.amplitude,
            limit: applicableRange.limit,
            excess: point.amplitude - applicableRange.limit,
            type: 'Measurement'
          })
        }
      })
    }
    
    return {
      peak: [],
      average: [],
      combined: violationsList.sort((a, b) => b.excess - a.excess),
      hasThreeColumns: false
    }
  }
})

const formatFrequency = (freq: number): string => {
  if (freq >= 1000) {
    return `${(freq / 1000).toFixed(2)} GHz`
  } else if (freq >= 1) {
    return `${freq.toFixed(2)} MHz`
  } else {
    return `${(freq * 1000).toFixed(0)} kHz`
  }
}

const exportSummary = () => {
  const summary = {
    standard: emcStore.currentStandard?.name || 'No standard selected',
    compliance: complianceStatus.value,
    frequencyStats: freqStats.value,
    amplitudeStats: amplitudeStats.value,
    violationsCount: violations.value.combined.length,
    peakViolations: violations.value.peak?.length || 0,
    avgViolations: violations.value.average?.length || 0,
    hasThreeColumns: violations.value.hasThreeColumns || false,
    exportDate: new Date().toISOString()
  }
  
  const blob = new Blob([JSON.stringify(summary, null, 2)], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const link = document.createElement('a')
  link.href = url
  link.download = 'emc-analysis-summary.json'
  link.click()
  URL.revokeObjectURL(url)
}

const exportViolations = () => {
  if (violations.value.combined.length === 0) return
  
  const headers = violations.value.hasThreeColumns 
    ? 'Type,Frequency (MHz),Measured (dBμV),Limit (dBμV),Violation (dB)'
    : 'Frequency (MHz),Measured (dBμV),Limit (dBμV),Violation (dB)'
  
  const rows = violations.value.combined.map(v => {
    const baseRow = `${v.frequency},${v.measured.toFixed(2)},${v.limit.toFixed(2)},${v.excess.toFixed(2)}`
    return violations.value.hasThreeColumns ? `${v.type},${baseRow}` : baseRow
  })
  
  const csv = [headers, ...rows].join('\n')
  
  const blob = new Blob([csv], { type: 'text/csv' })
  const url = URL.createObjectURL(blob)
  const link = document.createElement('a')
  link.href = url
  link.download = 'emc-violations.csv'
  link.click()
  URL.revokeObjectURL(url)
}
</script>
