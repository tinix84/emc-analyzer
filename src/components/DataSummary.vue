<template>
  <div class="space-y-6">
    <!-- Compliance Status -->
    <div v-if="complianceStatus" class="grid md:grid-cols-3 gap-4">
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
            Violations Found
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
          <div class="flex justify-between">
            <span class="text-gray-600">Maximum:</span>
            <span class="font-medium">{{ amplitudeStats.max.toFixed(2) }} dBμV</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">Minimum:</span>
            <span class="font-medium">{{ amplitudeStats.min.toFixed(2) }} dBμV</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">Average:</span>
            <span class="font-medium">{{ amplitudeStats.avg.toFixed(2) }} dBμV</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">Std Deviation:</span>
            <span class="font-medium">{{ amplitudeStats.std.toFixed(2) }} dB</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Violations Detail -->
    <div v-if="violations.length > 0" class="bg-red-50 border border-red-200 rounded-lg p-4">
      <h3 class="text-lg font-semibold text-red-800 mb-4 flex items-center">
        ⚠️ Compliance Violations
      </h3>
      <div class="overflow-x-auto">
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
              v-for="(violation, index) in violations.slice(0, 10)"
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
        <div v-if="violations.length > 10" class="text-sm text-red-600 mt-2">
          Showing 10 of {{ violations.length }} violations. 
          <button @click="showAllViolations = !showAllViolations" class="underline">
            {{ showAllViolations ? 'Show less' : 'Show all' }}
          </button>
        </div>
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
        v-if="violations.length > 0"
        class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors duration-200"
      >
        ⚠️ Export Violations
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useEMCStore } from '@/stores/emcStore'

const props = defineProps<{
  data: Array<{frequency: number, amplitude: number}>
  standard: string
}>()

const emcStore = useEMCStore()
const showAllViolations = ref(false)

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
  
  const amplitudes = props.data.map(d => d.amplitude)
  const min = Math.min(...amplitudes)
  const max = Math.max(...amplitudes)
  const avg = amplitudes.reduce((sum, val) => sum + val, 0) / amplitudes.length
  
  const variance = amplitudes.reduce((sum, val) => sum + Math.pow(val - avg, 2), 0) / amplitudes.length
  const std = Math.sqrt(variance)
  
  return { min, max, avg, std }
})

const violations = computed(() => {
  if (!emcStore.currentStandard || props.data.length === 0) return []
  
  const standard = emcStore.currentStandard
  const violationsList: Array<{
    frequency: number
    measured: number
    limit: number
    excess: number
  }> = []
  
  props.data.forEach(point => {
    const applicableRange = standard.frequencyRanges.find(
      range => point.frequency >= range.startFreq && point.frequency <= range.endFreq
    )
    
    if (applicableRange && point.amplitude > applicableRange.limit) {
      violationsList.push({
        frequency: point.frequency,
        measured: point.amplitude,
        limit: applicableRange.limit,
        excess: point.amplitude - applicableRange.limit
      })
    }
  })
  
  return violationsList.sort((a, b) => b.excess - a.excess) // Sort by worst violations first
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
    violationsCount: violations.value.length,
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
  if (violations.value.length === 0) return
  
  const csv = [
    'Frequency (MHz),Measured (dBμV),Limit (dBμV),Violation (dB)',
    ...violations.value.map(v => 
      `${v.frequency},${v.measured.toFixed(2)},${v.limit.toFixed(2)},${v.excess.toFixed(2)}`
    )
  ].join('\n')
  
  const blob = new Blob([csv], { type: 'text/csv' })
  const url = URL.createObjectURL(blob)
  const link = document.createElement('a')
  link.href = url
  link.download = 'emc-violations.csv'
  link.click()
  URL.revokeObjectURL(url)
}
</script>
