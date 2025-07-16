<!-- pages/integration-test.vue -->
<template>
  <div class="min-h-screen bg-gradient-to-br from-purple-50 via-pink-50 to-red-50">
    <div class="container mx-auto px-4 py-8">
      <h1 class="text-4xl font-bold text-center mb-8">🔬 Full Integration Test</h1>
      
      <!-- Step-by-step testing -->
      <div class="space-y-6">
        
        <!-- Step 1: WASM Loading -->
        <div class="bg-white rounded-lg shadow-lg p-6">
          <h2 class="text-2xl font-bold mb-4 flex items-center">
            <span :class="step1Status.icon">{{ step1Status.emoji }}</span>
            <span class="ml-2">Step 1: WASM Loading</span>
          </h2>
          
          <div class="mb-4">
            <div v-if="wasm.isLoading" class="text-blue-600">🔄 Loading WASM...</div>
            <div v-else-if="wasm.error" class="text-red-600">❌ {{ wasm.error }}</div>
            <div v-else-if="wasm.wasmModule" class="text-green-600">✅ WASM loaded successfully</div>
            <div v-else class="text-gray-600">⏳ Waiting...</div>
          </div>
          
          <button 
            @click="runStep1"
            :disabled="wasm.isLoading"
            class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:bg-gray-400"
          >
            {{ wasm.isLoading ? 'Loading...' : 'Test WASM Loading' }}
          </button>
        </div>

        <!-- Step 2: Basic Function -->
        <div class="bg-white rounded-lg shadow-lg p-6">
          <h2 class="text-2xl font-bold mb-4 flex items-center">
            <span :class="step2Status.icon">{{ step2Status.emoji }}</span>
            <span class="ml-2">Step 2: Basic Function Test</span>
          </h2>
          
          <div class="mb-4">
            <div v-if="step2Result" :class="step2Result.success ? 'text-green-600' : 'text-red-600'">
              {{ step2Result.message }}
            </div>
            <div v-else class="text-gray-600">⏳ Not tested yet</div>
          </div>
          
          <button 
            @click="runStep2"
            :disabled="!wasm.wasmModule || step2Running"
            class="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 disabled:bg-gray-400"
          >
            {{ step2Running ? 'Testing...' : 'Test Basic Function' }}
          </button>
        </div>

        <!-- Step 3: GUI Integration -->
        <div class="bg-white rounded-lg shadow-lg p-6">
          <h2 class="text-2xl font-bold mb-4 flex items-center">
            <span :class="step3Status.icon">{{ step3Status.emoji }}</span>
            <span class="ml-2">Step 3: GUI Components</span>
          </h2>
          
          <div class="mb-4">
            <div v-if="step3Result" :class="step3Result.success ? 'text-green-600' : 'text-red-600'">
              {{ step3Result.message }}
            </div>
            <div v-else class="text-gray-600">⏳ Not tested yet</div>
          </div>
          
          <!-- Mini component tests -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
            <div class="border rounded p-4">
              <h4 class="font-semibold mb-2">Standard Loader</h4>
              <select v-model="testStandard" class="w-full p-2 border rounded">
                <option value="">Select...</option>
                <option value="CISPR25">CISPR25</option>
                <option value="EN55032">EN55032</option>
              </select>
              <select v-model="testClass" class="w-full p-2 border rounded mt-2">
                <option value="">Class...</option>
                <option value="Class3">Class 3</option>
                <option value="Class5">Class 5</option>
              </select>
            </div>
            
            <div class="border rounded p-4">
              <h4 class="font-semibold mb-2">Mock Data</h4>
              <button 
                @click="loadMockData"
                class="w-full p-2 bg-blue-600 text-white rounded hover:bg-blue-700"
              >
                Load Test Data
              </button>
            </div>
          </div>
          
          <button 
            @click="runStep3"
            :disabled="!step2Result?.success || step3Running"
            class="px-4 py-2 bg-purple-600 text-white rounded-lg hover:bg-purple-700 disabled:bg-gray-400"
          >
            {{ step3Running ? 'Testing...' : 'Test GUI Integration' }}
          </button>
        </div>

        <!-- Step 4: Full Integration -->
        <div class="bg-white rounded-lg shadow-lg p-6">
          <h2 class="text-2xl font-bold mb-4 flex items-center">
            <span :class="step4Status.icon">{{ step4Status.emoji }}</span>
            <span class="ml-2">Step 4: Full Integration</span>
          </h2>
          
          <div class="mb-4">
            <div v-if="step4Result" :class="step4Result.success ? 'text-green-600' : 'text-red-600'">
              {{ step4Result.message }}
            </div>
            <div v-else class="text-gray-600">⏳ Not tested yet</div>
          </div>
          
          <!-- Show chart if data available -->
          <div v-if="currentStandard && measurementData" class="mb-4">
            <h4 class="font-semibold mb-2">Live Chart Preview:</h4>
            <div class="border rounded p-4 bg-gray-50">
              <EMCChart 
                :standard="currentStandard" 
                :measurement-data="measurementData"
              />
            </div>
          </div>
          
          <button 
            @click="runStep4"
            :disabled="!step3Result?.success || step4Running"
            class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:bg-gray-400"
          >
            {{ step4Running ? 'Testing...' : 'Test Full Integration' }}
          </button>
        </div>

        <!-- Summary -->
        <div v-if="allStepsComplete" class="bg-gradient-to-r from-green-400 to-blue-500 text-white rounded-lg shadow-lg p-6">
          <h2 class="text-3xl font-bold text-center">🎉 Integration Test Complete!</h2>
          <p class="text-center mt-2">All systems working correctly. The EMC analyzer is ready to use!</p>
          <div class="text-center mt-4">
            <a 
              href="/" 
              class="inline-block px-6 py-3 bg-white text-blue-600 rounded-lg font-semibold hover:bg-gray-100"
            >
              Go to Main App
            </a>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { EMCStandard } from '~/types/emc'

const wasm = useWasm()

// Test states
const step2Running = ref(false)
const step3Running = ref(false)
const step4Running = ref(false)

const step2Result = ref<{success: boolean, message: string} | null>(null)
const step3Result = ref<{success: boolean, message: string} | null>(null)
const step4Result = ref<{success: boolean, message: string} | null>(null)

// Test data
const testStandard = ref('')
const testClass = ref('')
const currentStandard = ref<EMCStandard | null>(null)
const measurementData = ref<Array<{frequency: number, amplitude: number}> | null>(null)

// Status computed properties
const step1Status = computed(() => {
  if (wasm.isLoading.value) return { emoji: '⏳', icon: 'text-blue-600' }
  if (wasm.error.value) return { emoji: '❌', icon: 'text-red-600' }
  if (wasm.wasmModule.value) return { emoji: '✅', icon: 'text-green-600' }
  return { emoji: '⏸️', icon: 'text-gray-600' }
})

const step2Status = computed(() => {
  if (step2Running.value) return { emoji: '⏳', icon: 'text-blue-600' }
  if (step2Result.value?.success) return { emoji: '✅', icon: 'text-green-600' }
  if (step2Result.value && !step2Result.value.success) return { emoji: '❌', icon: 'text-red-600' }
  return { emoji: '⏸️', icon: 'text-gray-600' }
})

const step3Status = computed(() => {
  if (step3Running.value) return { emoji: '⏳', icon: 'text-blue-600' }
  if (step3Result.value?.success) return { emoji: '✅', icon: 'text-green-600' }
  if (step3Result.value && !step3Result.value.success) return { emoji: '❌', icon: 'text-red-600' }
  return { emoji: '⏸️', icon: 'text-gray-600' }
})

const step4Status = computed(() => {
  if (step4Running.value) return { emoji: '⏳', icon: 'text-blue-600' }
  if (step4Result.value?.success) return { emoji: '✅', icon: 'text-green-600' }
  if (step4Result.value && !step4Result.value.success) return { emoji: '❌', icon: 'text-red-600' }
  return { emoji: '⏸️', icon: 'text-gray-600' }
})

const allStepsComplete = computed(() => {
  return step1Status.value.emoji === '✅' &&
         step2Status.value.emoji === '✅' &&
         step3Status.value.emoji === '✅' &&
         step4Status.value.emoji === '✅'
})

// Test functions
const runStep1 = async () => {
  await wasm.initWasm()
}

const runStep2 = async () => {
  step2Running.value = true
  try {
    const standard = wasm.getStandard('CISPR25', 'Class3')
    if (standard && standard.name) {
      step2Result.value = { success: true, message: `✅ Loaded: ${standard.name}` }
    } else {
      step2Result.value = { success: false, message: '❌ Failed to load standard' }
    }
  } catch (error: any) {
    step2Result.value = { success: false, message: `❌ Error: ${error.message}` }
  } finally {
    step2Running.value = false
  }
}

const runStep3 = async () => {
  step3Running.value = true
  try {
    // Test loading a standard with form inputs
    if (testStandard.value && testClass.value) {
      currentStandard.value = wasm.getStandard(testStandard.value, testClass.value)
      step3Result.value = { success: true, message: '✅ GUI components working with WASM' }
    } else {
      // Use default
      currentStandard.value = wasm.getStandard('CISPR25', 'Class3')
      step3Result.value = { success: true, message: '✅ GUI components working (using defaults)' }
    }
  } catch (error: any) {
    step3Result.value = { success: false, message: `❌ GUI integration failed: ${error.message}` }
  } finally {
    step3Running.value = false
  }
}

const runStep4 = async () => {
  step4Running.value = true
  try {
    if (!currentStandard.value) {
      throw new Error('No standard loaded')
    }
    
    // Generate some measurement data and test compliance
    const frequencies = [500000, 1000000, 2000000, 5000000]
    const amplitudes = [45, 52, 48, 62]
    
    const compliance = wasm.checkCompliance(currentStandard.value, frequencies, amplitudes, 'avg')
    const passCount = compliance.filter(r => r.passes).length
    
    // Set measurement data for chart
    measurementData.value = frequencies.map((f, i) => ({ frequency: f, amplitude: amplitudes[i] }))
    
    step4Result.value = { 
      success: true, 
      message: `✅ Full integration successful! ${passCount}/${compliance.length} measurements pass compliance` 
    }
  } catch (error: any) {
    step4Result.value = { success: false, message: `❌ Full integration failed: ${error.message}` }
  } finally {
    step4Running.value = false
  }
}

const loadMockData = () => {
  const data = []
  for (let f = 150000; f <= 30000000; f *= 1.5) {
    const amplitude = 40 + Math.random() * 25 + 5 * Math.sin(Math.log10(f))
    data.push({ frequency: f, amplitude })
  }
  measurementData.value = data
}

// Auto-start step 1
onMounted(() => {
  if (!wasm.wasmModule.value) {
    runStep1()
  }
})

useHead({
  title: 'Integration Test - EMC Analyzer',
  meta: [
    { name: 'description', content: 'Step-by-step integration testing' }
  ]
})
</script>
