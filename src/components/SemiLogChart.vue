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
      <!-- Peak Data -->
      <div class="flex items-center space-x-2" v-if="measurementData.length > 0 && measurementData.some(point => point.peak !== undefined)">
        <div class="w-4 h-4 bg-blue-500 rounded"></div>
        <span>Peak Data</span>
      </div>
      <!-- QP Data -->
      <div class="flex items-center space-x-2" v-if="measurementData.length > 0 && measurementData.some(point => point.qp !== undefined)">
        <div class="w-4 h-4 bg-green-500 rounded"></div>
        <span>QP Data</span>
      </div>
      <!-- Average Data -->
      <div class="flex items-center space-x-2" v-if="measurementData.length > 0 && measurementData.some(point => point.avg !== undefined)">
        <div class="w-4 h-4 bg-red-500 rounded"></div>
        <span>Average Data</span>
      </div>
      <!-- Single Measurement Data (fallback) -->
      <div class="flex items-center space-x-2" v-if="measurementData.length > 0 && !hasThreeColumnData">
        <div class="w-4 h-4 bg-blue-500 rounded"></div>
        <span>Measurement Data</span>
      </div>
      <!-- Multiple Masks Legend with updated colors -->
      <div v-if="showMask && standardMasks && Object.keys(standardMasks).length > 0" class="flex gap-4">
        <div v-if="standardMasks.avg && standardMasks.avg.length > 0" class="flex items-center space-x-2">
          <div class="w-4 h-4 bg-red-500 rounded"></div>
          <span>AVG Limit</span>
        </div>
        <div v-if="standardMasks.qp && standardMasks.qp.length > 0" class="flex items-center space-x-2">
          <div class="w-4 h-4 bg-green-500 rounded"></div>
          <span>QP Limit</span>
        </div>
        <div v-if="standardMasks.pk && standardMasks.pk.length > 0" class="flex items-center space-x-2">
          <div class="w-4 h-4 bg-blue-500 rounded"></div>
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
  measurementData: Array<{frequency: number, amplitude: number, peak?: number, avg?: number, qp?: number}>
  standardMasks?: { [key: string]: Array<{frequency: number, amplitude: number}> }
}>()

const chartCanvas = ref<HTMLCanvasElement>()
const chart = ref<Chart>()
const showMask = ref(true)
const showGrid = ref(true)

// Update management to prevent race conditions
const isUpdating = ref(false)
const updateTimeout = ref<number | null>(null)
const pendingDestroy = ref(false)

// Check if we have 3-column data
const hasThreeColumnData = computed(() => 
  props.measurementData.some(point => 
    (point.peak !== undefined && point.avg !== undefined) ||
    (point.peak !== undefined && point.qp !== undefined) ||
    (point.avg !== undefined && point.qp !== undefined)
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
      // Include peak, avg, and qp if available
      ...props.measurementData.flatMap(d => [d.peak, d.avg, d.qp].filter(v => v !== undefined) as number[])
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
  // Safety checks
  if (!chartCanvas.value || pendingDestroy.value || isUpdating.value) {
    return
  }

  const ctx = chartCanvas.value.getContext('2d')
  if (!ctx) {
    console.error('❌ Could not get canvas context')
    return
  }

  try {
    // Clear any existing chart
    if (chart.value) {
      try {
        chart.value.destroy()
      } catch (destroyError) {
        console.warn('⚠️ Error destroying existing chart:', destroyError)
      }
      chart.value = undefined
    }

    const datasets: any[] = []

    // Measurement data
    if (props.measurementData.length > 0) {
      // Check if we have multi-column data (peak, avg, qp)
      const hasMultiColumns = props.measurementData.some(point => 
        point.peak !== undefined || point.avg !== undefined || point.qp !== undefined
      )
      
      if (hasMultiColumns) {
        // Show Peak data if available
        if (props.measurementData.some(point => point.peak !== undefined)) {
          datasets.push({
            label: 'Peak Data',
            data: props.measurementData.map(point => ({
              x: Number(point.frequency),
              y: Number(point.peak || point.amplitude)
            })),
            borderColor: 'rgb(59, 130, 246)', // blue-500 - matches PK Limit
            backgroundColor: 'rgba(59, 130, 246, 0.1)',
            borderWidth: 2,
            pointRadius: 1,
            pointHoverRadius: 4,
            tension: 0.1,
            fill: false
          })
        }
        
        // Show QP data if available
        if (props.measurementData.some(point => point.qp !== undefined)) {
          datasets.push({
            label: 'QP Data',
            data: props.measurementData.map(point => ({
              x: Number(point.frequency),
              y: Number(point.qp || point.amplitude)
            })),
            borderColor: 'rgb(34, 197, 94)', // green-500 - matches QP Limit
            backgroundColor: 'rgba(34, 197, 94, 0.1)',
            borderWidth: 2,
            pointRadius: 1,
            pointHoverRadius: 4,
            tension: 0.1,
            fill: false
          })
        }
        
        // Show Average data if available
        if (props.measurementData.some(point => point.avg !== undefined)) {
          datasets.push({
            label: 'Average Data',
            data: props.measurementData.map(point => ({
              x: Number(point.frequency),
              y: Number(point.avg || point.amplitude)
            })),
            borderColor: 'rgb(239, 68, 68)', // red-500 - matches AVG Limit
            backgroundColor: 'rgba(239, 68, 68, 0.1)',
            borderWidth: 2,
            pointRadius: 1,
            pointHoverRadius: 4,
            tension: 0.1,
            fill: false
          })
        }
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

    // Multiple standard masks (AVG=red, QP=green, PK=blue) with double thickness
    if (showMask.value && props.standardMasks && Object.keys(props.standardMasks).length > 0) {
      const maskColors = {
        avg: { border: 'rgb(239, 68, 68)', background: 'rgba(239, 68, 68, 0.1)' }, // red-500
        qp: { border: 'rgb(34, 197, 94)', background: 'rgba(34, 197, 94, 0.1)' }, // green-500
        pk: { border: 'rgb(59, 130, 246)', background: 'rgba(59, 130, 246, 0.1)' }  // blue-500
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
            borderWidth: 4, // Double thickness for standards
            pointRadius: 0,
            tension: 0.1,
            fill: false,
            borderDash: maskType === 'pk' ? [5, 5] : [] // Only PK is dotted, AVG and QP are continuous
          })
        }
      })
    }

    // Create chart with defensive options
    chart.value = new Chart(ctx, {
      type: 'line',
      data: { 
        datasets: datasets
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        animation: false, // Disable animations to prevent recursion
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
    if (ctx && !pendingDestroy.value) {
      try {
        ctx.clearRect(0, 0, chartCanvas.value!.width, chartCanvas.value!.height)
        ctx.fillStyle = '#666'
        ctx.font = '16px Arial'
        ctx.fillText('Chart unavailable - check console for details', 50, 50)
      } catch (fallbackError) {
        console.error('❌ Even fallback rendering failed:', fallbackError)
      }
    }
  }
}

const updateChart = () => {
  // Prevent concurrent updates
  if (isUpdating.value || pendingDestroy.value || !chart.value) {
    return
  }

  isUpdating.value = true

  try {
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

    // Multiple standard masks (AVG=red, QP=green, PK=blue) with double thickness
    if (showMask.value && props.standardMasks && Object.keys(props.standardMasks).length > 0) {
      const maskColors = {
        avg: { border: 'rgb(239, 68, 68)', background: 'rgba(239, 68, 68, 0.1)' }, // red-500
        qp: { border: 'rgb(34, 197, 94)', background: 'rgba(34, 197, 94, 0.1)' }, // green-500
        pk: { border: 'rgb(59, 130, 246)', background: 'rgba(59, 130, 246, 0.1)' }  // blue-500
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
            borderWidth: 4, // Double thickness for standards
            pointRadius: 0,
            tension: 0.1,
            fill: false,
            borderDash: maskType === 'pk' ? [5, 5] : [] // Only PK is dotted, AVG and QP are continuous
          })
        }
      })
    }

    // Safely update chart data
    if (chart.value && !pendingDestroy.value) {
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
    }
  } catch (error) {
    console.error('⚠️ Chart update failed:', error)
    // Schedule recreation only if we're not already updating
    if (!pendingDestroy.value) {
      scheduleChartRecreation()
    }
  } finally {
    isUpdating.value = false
  }
}

// Debounced update function
const debouncedUpdateChart = () => {
  if (updateTimeout.value) {
    clearTimeout(updateTimeout.value)
  }
  
  updateTimeout.value = window.setTimeout(() => {
    updateChart()
    updateTimeout.value = null
  }, 150) // 150ms debounce
}

// Safe chart recreation
const scheduleChartRecreation = () => {
  if (pendingDestroy.value) return
  
  pendingDestroy.value = true
  
  setTimeout(() => {
    try {
      if (chart.value) {
        chart.value.destroy()
        chart.value = undefined
      }
      
      // Wait a bit more before recreation
      setTimeout(() => {
        pendingDestroy.value = false
        createChart()
      }, 200)
    } catch (error) {
      console.error('❌ Chart destruction failed:', error)
      pendingDestroy.value = false
    }
  }, 100)
}

const resetZoom = () => {
  if (chart.value && !isUpdating.value && !pendingDestroy.value) {
    // Simple reset by recreating the chart
    scheduleChartRecreation()
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

// Watchers with debouncing to prevent rapid updates
watch([() => props.measurementData, () => props.standardMasks], () => {
  try {
    debouncedUpdateChart()
  } catch (error) {
    console.error('⚠️ Error in chart watcher:', error)
  }
}, { deep: true, immediate: false })

watch([showMask, showGrid], () => {
  try {
    debouncedUpdateChart()
  } catch (error) {
    console.error('⚠️ Error in options watcher:', error)
  }
}, { immediate: false })

onMounted(() => {
  try {
    createChart()
  } catch (error) {
    console.error('⚠️ Error creating chart:', error)
  }
})

onUnmounted(() => {
  // Clear any pending timeouts
  if (updateTimeout.value) {
    clearTimeout(updateTimeout.value)
    updateTimeout.value = null
  }
  
  // Set flag to prevent any pending operations
  pendingDestroy.value = true
  isUpdating.value = false
  
  // Destroy chart safely
  if (chart.value) {
    try {
      chart.value.destroy()
      chart.value = undefined
    } catch (error) {
      console.error('⚠️ Error destroying chart:', error)
    }
  }
})
</script>
