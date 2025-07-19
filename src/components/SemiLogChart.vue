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
          <span class="text-sm text-white">Show Standard Mask</span>
        </label>
        <label class="flex items-center space-x-2">
          <input 
            type="checkbox" 
            v-model="showGrid"
            class="rounded"
          >
          <span class="text-sm text-white">Show Grid</span>
        </label>
      </div>
      
      <div class="flex gap-2">
        <button
          @click="resetZoom"
          class="px-3 py-1 text-sm bg-gray-600 hover:bg-gray-500 text-white rounded transition-colors duration-200"
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
    <div class="bg-white border border-gray-600 rounded-lg p-4" style="height: 500px;">
      <canvas ref="chartCanvas"></canvas>
    </div>

    <!-- Chart Legend -->
    <div class="flex flex-wrap gap-4 text-sm text-white">
      <div class="flex items-center space-x-2" v-if="measurementData.length > 0">
        <div class="w-4 h-4 bg-red-500 rounded" v-if="hasThreeColumnData"></div>
        <div class="w-4 h-4 bg-blue-500 rounded" v-else></div>
        <span v-if="hasThreeColumnData">Peak Data</span>
        <span v-else>Measurement Data</span>
      </div>
      <div class="flex items-center space-x-2" v-if="hasThreeColumnData">
        <div class="w-4 h-4 bg-green-500 rounded"></div>
        <span>Average Data</span>
      </div>
      <!-- Multiple Masks Legend -->
      <div v-if="showMask && standardMasks && Object.keys(standardMasks).length > 0" class="flex gap-4">
        <div v-if="standardMasks.avg && standardMasks.avg.length > 0" class="flex items-center space-x-2">
          <div class="w-4 h-4 bg-green-500 rounded"></div>
          <span>AVG Limit</span>
        </div>
        <div v-if="standardMasks.qp && standardMasks.qp.length > 0" class="flex items-center space-x-2">
          <div class="w-4 h-4 bg-orange-500 rounded"></div>
          <span>QP Limit</span>
        </div>
        <div v-if="standardMasks.pk && standardMasks.pk.length > 0" class="flex items-center space-x-2">
          <div class="w-4 h-4 bg-purple-500 rounded"></div>
          <span>PK Limit</span>
        </div>
      </div>
    </div>

    <!-- Statistics -->
    <div v-if="measurementData.length > 0" class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
      <div class="bg-gray-700 p-3 rounded border border-gray-600">
        <div class="text-gray-300">Data Points</div>
        <div class="text-lg font-semibold text-white">{{ measurementData.length }}</div>
      </div>
      <div class="bg-gray-700 p-3 rounded border border-gray-600">
        <div class="text-gray-300">Freq Range</div>
        <div class="text-lg font-semibold text-white">
          {{ formatFrequency(minFreq) }} - {{ formatFrequency(maxFreq) }}
        </div>
      </div>
      <div class="bg-gray-700 p-3 rounded border border-gray-600">
        <div class="text-gray-300">Max Amplitude</div>
        <div class="text-lg font-semibold text-white">{{ maxAmplitude.toFixed(1) }} dBμV</div>
      </div>
      <div class="bg-gray-700 p-3 rounded border border-gray-600">
        <div class="text-gray-300">Min Amplitude</div>
        <div class="text-lg font-semibold text-white">{{ minAmplitude.toFixed(1) }} dBμV</div>
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
  measurementData: Array<{frequency: number, amplitude: number, peak?: number, avg?: number}>
  standardMasks?: { [key: string]: Array<{frequency: number, amplitude: number}> }
}>()

const chartCanvas = ref<HTMLCanvasElement>()
const chart = ref<Chart>()
const showMask = ref(true)
const showGrid = ref(true)

// Check if we have 3-column data
const hasThreeColumnData = computed(() => 
  props.measurementData.some(point => 
    point.peak !== undefined && point.avg !== undefined
  )
)

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

// Dynamic Y-axis maximum calculation
const dynamicYMax = computed(() => {
  let maxValue = 0

  // Get maximum from measurement data
  if (props.measurementData.length > 0) {
    const measurementMax = Math.max(
      ...props.measurementData.map(d => d.amplitude),
      // Include peak and avg if available
      ...props.measurementData.flatMap(d => [d.peak, d.avg].filter(v => v !== undefined) as number[])
    )
    maxValue = Math.max(maxValue, measurementMax)
  }

  // Get maximum from standard masks
  if (props.standardMasks && Object.keys(props.standardMasks).length > 0) {
    Object.values(props.standardMasks).forEach(maskData => {
      if (maskData && maskData.length > 0) {
        const maskMax = Math.max(...maskData.map(point => point.amplitude))
        maxValue = Math.max(maxValue, maskMax)
      }
    })
  }

  // Add 20% margin and round up to nearest 10
  const withMargin = maxValue * 1.2
  const result = Math.max(Math.ceil(withMargin / 10) * 10, 100) // Minimum 100 dBμV
  
  console.log(`📊 Dynamic Y-axis max: ${result} dBμV (based on max value: ${maxValue.toFixed(1)} dBμV)`)
  return result
})

const createChart = () => {
  if (!chartCanvas.value) return

  const ctx = chartCanvas.value.getContext('2d')
  if (!ctx) return

  // Clear any existing chart
  if (chart.value) {
    chart.value.destroy()
    chart.value = undefined
  }

  const datasets: any[] = []

  // Measurement data
  if (props.measurementData.length > 0) {
    // Check if we have 3-column data (peak and avg)
    const hasThreeColumns = props.measurementData.some(point => 
      point.peak !== undefined && point.avg !== undefined
    )
    
    if (hasThreeColumns) {
      // Show Peak data
      datasets.push({
        label: 'Peak Data',
        data: props.measurementData.map(point => ({
          x: Number(point.frequency),
          y: Number(point.peak || point.amplitude)
        })),
        borderColor: 'rgb(239, 68, 68)', // red-500
        backgroundColor: 'rgba(239, 68, 68, 0.1)',
        borderWidth: 2,
        pointRadius: 1,
        pointHoverRadius: 4,
        tension: 0.1,
        fill: false
      })
      
      // Show Average data
      datasets.push({
        label: 'Average Data',
        data: props.measurementData.map(point => ({
          x: Number(point.frequency),
          y: Number(point.avg || point.amplitude)
        })),
        borderColor: 'rgb(34, 197, 94)', // green-500
        backgroundColor: 'rgba(34, 197, 94, 0.1)',
        borderWidth: 2,
        pointRadius: 1,
        pointHoverRadius: 4,
        tension: 0.1,
        fill: false
      })
    } else {
      // Show single measurement data (backward compatibility)
      datasets.push({
        label: 'Measurement Data',
        data: props.measurementData.map(point => ({
          x: Number(point.frequency),
          y: Number(point.amplitude)
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
  }

  // Multiple standard masks (AVG, QP, PK)
  if (showMask.value && props.standardMasks && Object.keys(props.standardMasks).length > 0) {
    const maskColors = {
      avg: { border: 'rgb(34, 197, 94)', background: 'rgba(34, 197, 94, 0.1)' }, // green-500
      qp: { border: 'rgb(249, 115, 22)', background: 'rgba(249, 115, 22, 0.1)' }, // orange-500
      pk: { border: 'rgb(147, 51, 234)', background: 'rgba(147, 51, 234, 0.1)' }  // purple-500
    }

    Object.entries(props.standardMasks).forEach(([maskType, maskData]) => {
      if (maskData && maskData.length > 0) {
        const colors = maskColors[maskType as keyof typeof maskColors] || 
                      { border: 'rgb(156, 163, 175)', background: 'rgba(156, 163, 175, 0.1)' }
        
        datasets.push({
          label: `${maskType.toUpperCase()} Limit`,
          data: maskData.map(point => ({
            x: Number(point.frequency),
            y: Number(point.amplitude)
          })),
          borderColor: colors.border,
          backgroundColor: colors.background,
          borderWidth: 2,
          pointRadius: 0,
          tension: 0.1,
          fill: false,
          borderDash: maskType === 'pk' ? [5, 5] : maskType === 'qp' ? [10, 5] : []
        })
      }
    })
  }

  try {
    chart.value = new Chart(ctx, {
      type: 'line',
      data: { 
        datasets: datasets
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        parsing: {
          xAxisKey: 'x',
          yAxisKey: 'y'
        },
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
            min: 0.1,
            max: 1000,
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
            },
            min: 0,
            max: dynamicYMax.value
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

  console.log('📈 Updating chart with:')
  console.log('  - Measurement data points:', props.measurementData.length)
  console.log('  - Standard masks:', props.standardMasks ? Object.keys(props.standardMasks) : 'none')
  console.log('  - Show mask:', showMask.value)

  const datasets: any[] = []

  // Measurement data
  if (props.measurementData.length > 0) {
    datasets.push({
      label: 'Measurement Data',
      data: props.measurementData.map(point => ({
        x: Number(point.frequency),
        y: Number(point.amplitude)
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

  // Multiple standard masks (AVG, QP, PK)
  if (showMask.value && props.standardMasks && Object.keys(props.standardMasks).length > 0) {
    const maskColors = {
      avg: { border: 'rgb(34, 197, 94)', background: 'rgba(34, 197, 94, 0.1)' }, // green-500
      qp: { border: 'rgb(249, 115, 22)', background: 'rgba(249, 115, 22, 0.1)' }, // orange-500
      pk: { border: 'rgb(147, 51, 234)', background: 'rgba(147, 51, 234, 0.1)' }  // purple-500
    }

    Object.entries(props.standardMasks).forEach(([maskType, maskData]) => {
      if (maskData && maskData.length > 0) {
        const colors = maskColors[maskType as keyof typeof maskColors] || 
                      { border: 'rgb(156, 163, 175)', background: 'rgba(156, 163, 175, 0.1)' }
        
        datasets.push({
          label: `${maskType.toUpperCase()} Limit`,
          data: maskData.map(point => ({
            x: Number(point.frequency),
            y: Number(point.amplitude)
          })),
          borderColor: colors.border,
          backgroundColor: colors.background,
          borderWidth: 2,
          pointRadius: 0,
          tension: 0.1,
          fill: false,
          borderDash: maskType === 'pk' ? [5, 5] : maskType === 'qp' ? [10, 5] : []
        })
      }
    })
  }

  try {
    // Safely update chart data
    chart.value.data.datasets = datasets
    
    // Update grid settings
    if (chart.value.options.scales?.x?.grid) {
      chart.value.options.scales.x.grid.display = showGrid.value
    }
    if (chart.value.options.scales?.y?.grid) {
      chart.value.options.scales.y.grid.display = showGrid.value
    }
    
    // Update Y-axis maximum dynamically
    if (chart.value.options.scales?.y) {
      chart.value.options.scales.y.max = dynamicYMax.value
    }
    
    // Use 'none' mode to prevent animations and reduce recursion risk
    chart.value.update('none')
  } catch (error) {
    console.error('⚠️ Chart update failed, recreating chart:', error)
    // If update fails, recreate the chart safely
    try {
      if (chart.value) {
        chart.value.destroy()
        chart.value = undefined
      }
      setTimeout(() => createChart(), 100) // Delay recreation to prevent recursion
    } catch (recreateError) {
      console.error('❌ Chart recreation also failed:', recreateError)
    }
  }
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

// Watchers with better error handling
watch([() => props.measurementData, () => props.standardMasks], () => {
  try {
    updateChart()
  } catch (error) {
    console.error('⚠️ Error in chart watcher:', error)
  }
}, { deep: true, immediate: false })

watch([showMask, showGrid], () => {
  try {
    updateChart()
  } catch (error) {
    console.error('⚠️ Error in options watcher:', error)
  }
})

onMounted(() => {
  try {
    createChart()
  } catch (error) {
    console.error('⚠️ Error creating chart:', error)
  }
})

onUnmounted(() => {
  if (chart.value) {
    chart.value.destroy()
  }
})
</script>
