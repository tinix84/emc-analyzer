<!-- components/EMCChart.vue -->
<template>
  <div class="bg-white rounded-lg shadow-lg p-6">
    <h2 class="text-2xl font-bold text-gray-800 mb-4">📊 EMC Analysis Chart</h2>
    
    <!-- Chart Container -->
    <div v-if="hasData" class="relative">
      <canvas 
        ref="chartCanvas" 
        :width="chartWidth" 
        :height="chartHeight"
        class="w-full h-96 border border-gray-200 rounded"
      ></canvas>
    </div>
    
    <!-- No Data State -->
    <div v-else class="text-center py-16 text-gray-500">
      <div class="text-6xl mb-4">📈</div>
      <p class="text-lg">No data to display</p>
      <p class="text-sm">Load an EMC standard and upload measurement data to see the chart</p>
    </div>
    
    <!-- Chart Controls -->
    <div v-if="hasData" class="mt-4 flex flex-wrap gap-4">
      <div class="flex items-center gap-2">
        <input 
          id="showLimit" 
          v-model="showLimit" 
          type="checkbox" 
          class="rounded border-gray-300"
        >
        <label for="showLimit" class="text-sm font-medium text-gray-700">Show Limit Line</label>
      </div>
      
      <div class="flex items-center gap-2">
        <input 
          id="showMeasurement" 
          v-model="showMeasurement" 
          type="checkbox" 
          class="rounded border-gray-300"
        >
        <label for="showMeasurement" class="text-sm font-medium text-gray-700">Show Measurement Data</label>
      </div>
      
      <div class="flex items-center gap-2">
        <label for="frequencyRange" class="text-sm font-medium text-gray-700">Frequency Range:</label>
        <select 
          id="frequencyRange" 
          v-model="frequencyRange" 
          class="rounded border-gray-300 text-sm"
        >
          <option value="150k-30M">150 kHz - 30 MHz</option>
          <option value="30M-1G">30 MHz - 1 GHz</option>
          <option value="1G-40G">1 GHz - 40 GHz</option>
          <option value="full">Full Range</option>
        </select>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { EMCStandard } from '~/types/emc'

interface Props {
  standard: EMCStandard | null
  measurementData: Array<{ frequency: number; amplitude: number }> | null
}

const props = defineProps<Props>()

// Chart configuration
const chartCanvas = ref<HTMLCanvasElement>()
const chartWidth = ref(800)
const chartHeight = ref(400)

// Chart controls
const showLimit = ref(true)
const showMeasurement = ref(true)
const frequencyRange = ref('150k-30M')

// Computed properties
const hasData = computed(() => {
  return props.standard || (props.measurementData && props.measurementData.length > 0)
})

// Chart rendering
const renderChart = () => {
  if (!chartCanvas.value) return
  
  const ctx = chartCanvas.value.getContext('2d')
  if (!ctx) return
  
  // Clear canvas
  ctx.clearRect(0, 0, chartWidth.value, chartHeight.value)
  
  // Set up basic styling
  ctx.strokeStyle = '#374151'
  ctx.lineWidth = 1
  ctx.font = '12px Inter, sans-serif'
  
  // Draw grid
  drawGrid(ctx)
  
  // Draw axes
  drawAxes(ctx)
  
  // Draw limit line if standard is loaded and enabled
  if (props.standard && showLimit.value) {
    drawLimitLine(ctx)
  }
  
  // Draw measurement data if available and enabled
  if (props.measurementData && props.measurementData.length > 0 && showMeasurement.value) {
    drawMeasurementData(ctx)
  }
  
  // Draw labels and title
  drawLabels(ctx)
}

const drawGrid = (ctx: CanvasRenderingContext2D) => {
  ctx.strokeStyle = '#e5e7eb'
  ctx.lineWidth = 0.5
  
  // Vertical grid lines (frequency)
  for (let i = 1; i < 10; i++) {
    const x = (chartWidth.value * i) / 10
    ctx.beginPath()
    ctx.moveTo(x, 0)
    ctx.lineTo(x, chartHeight.value)
    ctx.stroke()
  }
  
  // Horizontal grid lines (amplitude)
  for (let i = 1; i < 8; i++) {
    const y = (chartHeight.value * i) / 8
    ctx.beginPath()
    ctx.moveTo(0, y)
    ctx.lineTo(chartWidth.value, y)
    ctx.stroke()
  }
}

const drawAxes = (ctx: CanvasRenderingContext2D) => {
  ctx.strokeStyle = '#374151'
  ctx.lineWidth = 2
  
  // X-axis
  ctx.beginPath()
  ctx.moveTo(0, chartHeight.value)
  ctx.lineTo(chartWidth.value, chartHeight.value)
  ctx.stroke()
  
  // Y-axis
  ctx.beginPath()
  ctx.moveTo(0, 0)
  ctx.lineTo(0, chartHeight.value)
  ctx.stroke()
}

const drawLimitLine = (ctx: CanvasRenderingContext2D) => {
  if (!props.standard) return
  
  ctx.strokeStyle = '#dc2626'
  ctx.lineWidth = 2
  ctx.setLineDash([5, 5])
  
  ctx.beginPath()
  
  // Generate points for the limit line
  const points = generateLimitPoints()
  
  points.forEach((point, index) => {
    const x = frequencyToX(point.frequency)
    const y = amplitudeToY(point.amplitude)
    
    if (index === 0) {
      ctx.moveTo(x, y)
    } else {
      ctx.lineTo(x, y)
    }
  })
  
  ctx.stroke()
  ctx.setLineDash([])
}

const drawMeasurementData = (ctx: CanvasRenderingContext2D) => {
  if (!props.measurementData) return
  
  ctx.strokeStyle = '#2563eb'
  ctx.fillStyle = '#2563eb'
  ctx.lineWidth = 2
  
  ctx.beginPath()
  
  props.measurementData.forEach((point, index) => {
    const x = frequencyToX(point.frequency)
    const y = amplitudeToY(point.amplitude)
    
    if (index === 0) {
      ctx.moveTo(x, y)
    } else {
      ctx.lineTo(x, y)
    }
  })
  
  ctx.stroke()
  
  // Draw data points
  props.measurementData.forEach((point) => {
    const x = frequencyToX(point.frequency)
    const y = amplitudeToY(point.amplitude)
    
    ctx.beginPath()
    ctx.arc(x, y, 3, 0, 2 * Math.PI)
    ctx.fill()
  })
}

const drawLabels = (ctx: CanvasRenderingContext2D) => {
  ctx.fillStyle = '#374151'
  ctx.font = '14px Inter, sans-serif'
  
  // X-axis label
  ctx.fillText('Frequency (Hz)', chartWidth.value / 2 - 40, chartHeight.value - 10)
  
  // Y-axis label (rotated)
  ctx.save()
  ctx.translate(15, chartHeight.value / 2)
  ctx.rotate(-Math.PI / 2)
  ctx.fillText('Amplitude (dBμV)', 0, 0)
  ctx.restore()
}

// Helper functions for coordinate conversion
const frequencyToX = (frequency: number): number => {
  // Convert frequency to log scale and map to canvas width
  const logMin = Math.log10(150000) // 150 kHz
  const logMax = Math.log10(30000000) // 30 MHz
  const logFreq = Math.log10(frequency)
  
  return ((logFreq - logMin) / (logMax - logMin)) * chartWidth.value
}

const amplitudeToY = (amplitude: number): number => {
  // Map amplitude range (-20 to 80 dBμV) to canvas height
  const minAmp = -20
  const maxAmp = 80
  
  return chartHeight.value - ((amplitude - minAmp) / (maxAmp - minAmp)) * chartHeight.value
}

const generateLimitPoints = (): Array<{ frequency: number; amplitude: number }> => {
  // Generate sample limit points (this would normally use the WASM function)
  const points = []
  const startFreq = 150000 // 150 kHz
  const endFreq = 30000000 // 30 MHz
  
  for (let f = startFreq; f <= endFreq; f *= 1.1) {
    // Sample limit calculation (replace with actual WASM call)
    let amplitude = 40 // Base limit
    
    if (f < 500000) {
      amplitude = 60 - 20 * Math.log10(f / 150000)
    } else if (f < 5000000) {
      amplitude = 50
    } else {
      amplitude = 50 + 20 * Math.log10(f / 5000000)
    }
    
    points.push({ frequency: f, amplitude })
  }
  
  return points
}

// Watch for changes and re-render
watch([() => props.standard, () => props.measurementData, showLimit, showMeasurement, frequencyRange], () => {
  nextTick(() => {
    renderChart()
  })
}, { deep: true })

// Initial render
onMounted(() => {
  nextTick(() => {
    renderChart()
  })
})
</script>
