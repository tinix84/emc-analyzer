<!-- pages/test-wasm.vue -->
<template>
  <div class="min-h-screen bg-gradient-to-br from-green-50 via-blue-50 to-purple-50">
    <div class="container mx-auto px-4 py-8">
      <h1 class="text-4xl font-bold text-center mb-8">🧪 WASM Function Test</h1>
      
      <!-- WASM Status -->
      <div class="bg-white rounded-lg shadow-lg p-6 mb-8">
        <h2 class="text-2xl font-bold mb-4">📊 WASM Status</h2>
        
        <!-- Loading State -->
        <div v-if="wasm.isLoading" class="text-center py-8">
          <div class="inline-flex items-center px-6 py-3 bg-blue-100 rounded-lg text-blue-700">
            <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-blue-700" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            Loading WASM module...
          </div>
        </div>

        <!-- Success State -->
        <div v-else-if="!wasm.error && wasm.wasmModule" class="text-center py-4">
          <div class="inline-flex items-center px-6 py-3 bg-green-100 rounded-lg text-green-700">
            ✅ WASM module loaded successfully!
          </div>
        </div>

        <!-- Error State -->
        <div v-else-if="wasm.error" class="text-center py-4">
          <div class="inline-flex items-center px-6 py-3 bg-red-100 rounded-lg text-red-700 mb-4">
            ❌ {{ wasm.error }}
          </div>
          <button 
            @click="wasm.initWasm()" 
            class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700"
          >
            🔄 Retry WASM Loading
          </button>
        </div>

        <!-- Manual Control Buttons -->
        <div class="flex gap-4 mt-4">
          <button 
            @click="wasm.initWasm()"
            class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
          >
            🔄 Reload WASM
          </button>
          <button 
            @click="testBasicFunction"
            :disabled="!wasm.wasmModule"
            class="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 disabled:bg-gray-400"
          >
            🧪 Test Basic Function
          </button>
          <button 
            @click="testAllFunctions"
            :disabled="!wasm.wasmModule"
            class="px-4 py-2 bg-purple-600 text-white rounded-lg hover:bg-purple-700 disabled:bg-gray-400"
          >
            🚀 Test All Functions
          </button>
        </div>
      </div>

      <!-- Test Results -->
      <div class="bg-white rounded-lg shadow-lg p-6">
        <h2 class="text-2xl font-bold mb-4">📋 Test Results</h2>
        
        <div v-if="testResults.length === 0" class="text-gray-500 text-center py-8">
          No tests run yet. Click a test button above.
        </div>
        
        <div v-else class="space-y-4">
          <div 
            v-for="(result, index) in testResults" 
            :key="index"
            :class="{
              'border-l-4 p-4 rounded': true,
              'border-green-500 bg-green-50': result.success,
              'border-red-500 bg-red-50': !result.success,
              'border-blue-500 bg-blue-50': result.info
            }"
          >
            <div class="font-semibold">{{ result.title }}</div>
            <div class="text-sm mt-1">{{ result.message }}</div>
            <div v-if="result.data" class="text-xs mt-2 bg-gray-100 p-2 rounded">
              <pre>{{ JSON.stringify(result.data, null, 2) }}</pre>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface TestResult {
  title: string
  message: string
  success: boolean
  info?: boolean
  data?: any
}

const wasm = useWasm()
const testResults = ref<TestResult[]>([])

const addResult = (title: string, message: string, success: boolean, data?: any) => {
  testResults.value.push({
    title,
    message,
    success,
    data
  })
}

const testBasicFunction = async () => {
  testResults.value = [] // Clear previous results
  
  try {
    addResult('🔍 Module Check', 'Checking if WASM module is loaded...', true)
    
    if (!wasm.wasmModule.value) {
      addResult('❌ Module Missing', 'WASM module is not loaded', false)
      return
    }
    
    addResult('✅ Module Loaded', `WASM module found: ${typeof wasm.wasmModule.value}`, true)
    
    // Check available functions
    const functions = Object.keys(wasm.wasmModule.value).filter(k => typeof wasm.wasmModule.value[k] === 'function')
    addResult('📋 Available Functions', `Found ${functions.length} functions`, true, functions)
    
    // Test get_emc_standard
    addResult('🧪 Testing get_emc_standard...', 'Calling WASM function', true)
    const standard = wasm.getStandard('CISPR25', 'Class3')
    
    if (standard && standard.name) {
      addResult('✅ Function Success', `Standard loaded: ${standard.name}`, true, {
        name: standard.name,
        frequencyPoints: standard.f_avg_limit_mask?.length || 0,
        firstFreq: standard.f_avg_limit_mask?.[0],
        firstLimit: standard.dbuv_avg_limit_mask?.[0]
      })
    } else {
      addResult('❌ Function Failed', 'Standard not loaded properly', false, standard)
    }
    
  } catch (error: any) {
    addResult('❌ Test Error', `Error: ${error.message}`, false, error.stack)
  }
}

const testAllFunctions = async () => {
  testResults.value = [] // Clear previous results
  
  try {
    // Test 1: Get Standard
    addResult('🧪 Test 1', 'Getting EMC Standard...', true)
    const standard = wasm.getStandard('CISPR25', 'Class3')
    addResult('✅ Test 1 Success', `Loaded: ${standard.name}`, true, standard)
    
    // Test 2: Calculate Limit
    addResult('🧪 Test 2', 'Calculating limit at 1 MHz...', true)
    const limit = wasm.calculateLimit(standard, 1000000)
    addResult('✅ Test 2 Success', `AVG limit: ${limit.dbuv_avg_limit} dBμV`, true, limit)
    
    // Test 3: Check Compliance
    addResult('🧪 Test 3', 'Checking compliance...', true)
    const frequencies = [500000, 1000000, 5000000]
    const amplitudes = [45, 55, 65]
    const compliance = wasm.checkCompliance(standard, frequencies, amplitudes, 'avg')
    
    const passCount = compliance.filter(r => r.passes).length
    addResult('✅ Test 3 Success', `${passCount}/${compliance.length} measurements pass`, true, compliance)
    
    // Test 4: Generate Mask
    addResult('🧪 Test 4', 'Generating EMC mask...', true)
    const mask = wasm.generateMask(standard)
    addResult('✅ Test 4 Success', `Generated mask with ${mask.frequencies?.length || 0} points`, true, {
      pointCount: mask.frequencies?.length || 0,
      firstPoint: mask.frequencies?.[0],
      lastPoint: mask.frequencies?.slice(-1)[0]
    })
    
    addResult('🎉 All Tests Complete', 'All WASM functions working correctly!', true)
    
  } catch (error: any) {
    addResult('❌ Test Failed', `Error: ${error.message}`, false, error.stack)
  }
}

// Auto-test on mount if WASM is already loaded
onMounted(() => {
  if (wasm.wasmModule.value) {
    setTimeout(testBasicFunction, 1000)
  }
})

useHead({
  title: 'WASM Function Test',
  meta: [
    { name: 'description', content: 'Testing WASM functions directly' }
  ]
})
</script>
