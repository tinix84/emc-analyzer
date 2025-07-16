<!-- components/WasmDebugPanel.vue -->
<template>
  <div class="bg-gray-900 text-green-400 p-4 rounded-lg font-mono text-sm mb-8">
    <h3 class="text-white font-bold mb-3">🔍 WASM Debug Panel</h3>
    
    <div class="space-y-2">
      <div>
        <span class="text-gray-400">Loading State:</span> 
        <span :class="wasm.isLoading ? 'text-yellow-400' : 'text-green-400'">
          {{ wasm.isLoading ? 'LOADING...' : 'IDLE' }}
        </span>
      </div>
      
      <div>
        <span class="text-gray-400">WASM Module:</span> 
        <span :class="wasm.wasmModule ? 'text-green-400' : 'text-red-400'">
          {{ wasm.wasmModule ? 'LOADED ✅' : 'NOT LOADED ❌' }}
        </span>
      </div>
      
      <div v-if="wasm.error">
        <span class="text-gray-400">Error:</span> 
        <span class="text-red-400">{{ wasm.error }}</span>
      </div>
      
      <div>
        <span class="text-gray-400">Client Side:</span> 
        <span :class="isClient ? 'text-green-400' : 'text-red-400'">
          {{ isClient ? 'YES ✅' : 'NO ❌' }}
        </span>
      </div>
      
      <div v-if="wasm.wasmModule">
        <span class="text-gray-400">Available Functions:</span>
        <div class="ml-4 mt-1">
          <div v-for="func in wasmFunctions" :key="func">
            <span class="text-blue-400">• {{ func }}</span>
          </div>
        </div>
      </div>
      
      <div class="mt-4 space-x-2">
        <button 
          @click="testWasm" 
          :disabled="!wasm.wasmModule"
          class="px-3 py-1 bg-blue-600 text-white rounded disabled:opacity-50 disabled:cursor-not-allowed hover:bg-blue-700"
        >
          🧪 Test WASM
        </button>
        
        <button 
          @click="wasm.initWasm()" 
          class="px-3 py-1 bg-green-600 text-white rounded hover:bg-green-700"
        >
          🔄 Reload WASM
        </button>
      </div>
      
      <div v-if="testResult" class="mt-2">
        <span class="text-gray-400">Test Result:</span>
        <pre class="text-xs text-green-400 mt-1 overflow-x-auto">{{ JSON.stringify(testResult, null, 2) }}</pre>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const wasm = useWasm()
const isClient = process.client
const testResult = ref(null)

const wasmFunctions = computed(() => {
  if (!wasm.wasmModule) return []
  
  return Object.getOwnPropertyNames(wasm.wasmModule)
    .filter(name => typeof wasm.wasmModule[name] === 'function')
    .filter(name => !name.startsWith('_'))
})

const testWasm = async () => {
  try {
    console.log('🧪 Testing WASM functions...')
    testResult.value = null
    
    if (!wasm.wasmModule) {
      testResult.value = { error: 'WASM module not loaded' }
      return
    }
    
    // Test the get_emc_standard function
    const result = wasm.getStandard('CISPR22', 'B')
    testResult.value = { success: true, standard: result }
    console.log('✅ WASM test successful:', result)
  } catch (error: any) {
    testResult.value = { error: error.message, stack: error.stack }
    console.error('❌ WASM test failed:', error)
  }
}

// Auto-test when WASM loads
watch(() => wasm.wasmModule, (newValue) => {
  if (newValue) {
    console.log('🎉 WASM module loaded, auto-testing...')
    nextTick(() => testWasm())
  }
})
</script>
