<template>
  <div class="space-y-4">
    <!-- Chart Controls -->
    <div class="flex flex-wrap gap-4 items-center justify-between">
      <div class="flex gap-4">
        <label class="flex items-center space-x-2">
          <input 
            type="checkbox" 
            v-model="showMask"
            class="rounded"
          >
          <span class="text-sm">Show Standard Mask</span>
        </label>
        <label class="flex items-center space-x-2">
          <input 
            type="checkbox" 
            v-model="showGrid"
            class="rounded"
          >
          <span class="text-sm">Show Grid</span>
        </label>
      </div>
      
      <div class="flex gap-2">
        <button
          @click="resetZoom"
          class="px-3 py-1 text-sm bg-gray-200 hover:bg-gray-300 rounded transition-colors duration-200"
        >
          Reset Zoom
        </button>
        <button
          @click="exportChart"
          class="px-3 py-1 text-sm bg-blue-600 text-white hover:bg-blue-700 rounded transition-colors duration-200"
        >
          Export PNG
        </button>
      </div>
    </div>

    <!-- Chart Container -->
    <div class="bg-white border rounded-lg p-4" style="height: 500px;">
      <canvas ref="chartCanvas"></canvas>
    </div>

    <!-- Chart Legend -->
    <div class="flex flex-wrap gap-4 text-sm">
      <div class="flex items-center space-x-2">
        <div class="w-4 h-4 bg-blue-500 rounded"></div>
        <span>Measurement Data</span>
      </div>
      <div v-if="showMask && standardMask.length > 0" class="flex items-center space-x-2">
        <div class="w-4 h-4 bg-red-500 rounded"></div>
        <span>EMC Limit</span>
      </div>
    </div>

    <!-- Statistics -->
    <div v-if="measurementData.length > 0" class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
      <div class="bg-gray-50 p-3 rounded">
        <div class="text-gray-600">Data Points</div>
        <div class="text-lg font-semibold">{{ measurementData.length }}</div>
      </div>
      <div class="bg-gray-50 p-3 rounded">
        <div class="text-gray-600">Freq Range</div>
        <div class="text-lg font-semibold">
          {{ formatFrequency(minFreq) }} - {{ formatFrequency(maxFreq) }}
        </div>
      </div>
      <div class="bg-gray-50 p-3 rounded">
        <div class="text-gray-600">Max Amplitude</div>
        <div class="text-lg font-semibold">{{ maxAmplitude.toFixed(1) }} dBμV</div>
      </div>
      <div class="bg-gray-50 p-3 rounded">
        <div class="text-gray-600">Min Amplitude</div>
        <div class="text-lg font-semibold">{{ minAmplitude.toFixed(1) }} dBμV</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, computed } from 'vue'
import {
  Chart,
  CategoryScale,
  LinearScale,
  LogarithmicScale,
  LineElement,
  PointElement,
  LineController,
  Title,
  Tooltip,
  Legend,
  Filler
} from 'chart.js'

// Register Chart.js components
Chart.register(
  CategoryScale,
  LinearScale,
  LogarithmicScale,
  LineElement,
  PointElement,
  LineController,
  Title,
  Tooltip,
  Legend,
  Filler
)

const props = defineProps<{
  measurementData: Array<{frequency: number, amplitude: number}>
  standardMask: Array<{frequency: number, amplitude: number}>
}>()

const chartCanvas = ref<HTMLCanvasElement>()
const chart = ref<Chart>()
const showMask = ref(true)
const showGrid = ref(true)

// Computed statistics
const minFreq = computed(() => 
  props.measurementData.length > 0 
    ? Math.min(...props.measurementData.map(d => d.frequency))
    : 0
)

const maxFreq = computed(() => 
  props.measurementData.length > 0 
    ? Math.max(...props.measurementData.map(d => d.frequency))
    : 0
)

const minAmplitude = computed(() => 
  props.measurementData.length > 0 
    ? Math.min(...props.measurementData.map(d => d.amplitude))
    : 0
)

const maxAmplitude = computed(() => 
  props.measurementData.length > 0 
    ? Math.max(...props.measurementData.map(d => d.amplitude))
    : 0
)

const createChart = () => {
  if (!chartCanvas.value) return

  const ctx = chartCanvas.value.getContext('2d')
  if (!ctx) return

  // Clear any existing chart
  if (chart.value) {
    chart.value.destroy()
  }

  const datasets = []

  // Measurement data
  if (props.measurementData.length > 0) {
    datasets.push({
      label: 'Measurement Data',
      data: props.measurementData.map(point => ({
        x: point.frequency,
        y: point.amplitude
      })),
      borderColor: 'rgb(59, 130, 246)', // blue-500
      backgroundColor: 'rgba(59, 130, 246, 0.1)',
      borderWidth: 2,
      pointRadius: 1,
      pointHoverRadius: 4,
      tension: 0.1,
      fill: false
    })
  }

  // Standard mask
  if (showMask.value && props.standardMask.length > 0) {
    datasets.push({
      label: 'EMC Limit',
      data: props.standardMask.map(point => ({
        x: point.frequency,
        y: point.amplitude
      })),
      borderColor: 'rgb(239, 68, 68)', // red-500
      backgroundColor: 'rgba(239, 68, 68, 0.1)',
      borderWidth: 2,
      pointRadius: 0,
      tension: 0.1,
      fill: false
    })
  }

  try {
    chart.value = new Chart(ctx, {
      type: 'line',
      data: { datasets },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        interaction: {
          intersect: false,
          mode: 'index'
        },
        plugins: {
          title: {
            display: true,
            text: 'EMC Measurement - Semi-Logarithmic Plot'
          },
          tooltip: {
            callbacks: {
              title: (context) => {
                return `Frequency: ${formatFrequency(context[0].parsed.x)}`
              },
              label: (context) => {
                return `${context.dataset.label}: ${context.parsed.y.toFixed(2)} dBμV`
              }
            }
          }
        },
        scales: {
          x: {
            type: 'logarithmic',
            display: true,
            title: {
              display: true,
              text: 'Frequency (MHz)'
            },
            grid: {
              display: showGrid.value
            },
            ticks: {
              callback: function(value) {
                return formatFrequency(value as number)
              }
            }
          },
          y: {
            type: 'linear',
            display: true,
            title: {
              display: true,
              text: 'Amplitude (dBμV)'
            },
            grid: {
              display: showGrid.value
            }
          }
        }
      }
    })
    
    console.log('📊 Chart created successfully')
  } catch (error) {
    console.error('❌ Error creating chart:', error)
    // Fallback: show simple message
    if (ctx) {
      ctx.fillStyle = '#666'
      ctx.font = '16px Arial'
      ctx.fillText('Chart unavailable - check console for details', 50, 50)
    }
  }
}

const updateChart = () => {
  if (!chart.value) return

  const datasets: any[] = []

  // Measurement data
  if (props.measurementData.length > 0) {
    datasets.push({
      label: 'Measurement Data',
      data: props.measurementData.map(point => ({
        x: point.frequency,
        y: point.amplitude
      })),
      borderColor: 'rgb(59, 130, 246)',
      backgroundColor: 'rgba(59, 130, 246, 0.1)',
      borderWidth: 2,
      pointRadius: 1,
      pointHoverRadius: 4,
      tension: 0.1,
      fill: false
    })
  }

  // Standard mask
  if (showMask.value && props.standardMask.length > 0) {
    datasets.push({
      label: 'EMC Limit',
      data: props.standardMask.map(point => ({
        x: point.frequency,
        y: point.amplitude
      })),
      borderColor: 'rgb(239, 68, 68)',
      backgroundColor: 'rgba(239, 68, 68, 0.1)',
      borderWidth: 2,
      pointRadius: 0,
      tension: 0.1,
      fill: false
    })
  }

  chart.value.data.datasets = datasets
  
  if (chart.value.options.scales?.x?.grid) {
    chart.value.options.scales.x.grid.display = showGrid.value
  }
  if (chart.value.options.scales?.y?.grid) {
    chart.value.options.scales.y.grid.display = showGrid.value
  }
  
  chart.value.update()
}

const resetZoom = () => {
  if (chart.value) {
    // Simple reset by recreating the chart
    createChart()
  }
}

const exportChart = () => {
  if (chart.value) {
    const url = chart.value.toBase64Image()
    const link = document.createElement('a')
    link.download = 'emc-measurement-chart.png'
    link.href = url
    link.click()
  }
}

const formatFrequency = (freq: number): string => {
  if (freq >= 1000) {
    return `${(freq / 1000).toFixed(1)} GHz`
  } else if (freq >= 1) {
    return `${freq.toFixed(1)} MHz`
  } else {
    return `${(freq * 1000).toFixed(0)} kHz`
  }
}

// Watchers
watch([() => props.measurementData, () => props.standardMask], updateChart, { deep: true })
watch([showMask, showGrid], updateChart)

onMounted(() => {
  createChart()
})

onUnmounted(() => {
  if (chart.value) {
    chart.value.destroy()
  }
})
</script>
