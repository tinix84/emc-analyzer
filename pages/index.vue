<template>
  <div class="min-h-screen bg-gradient-to-br from-blue-50 via-purple-50 to-pink-50">
    <div class="container mx-auto px-4 py-8">
      <!-- Header -->
      <div class="text-center mb-8">
        <h1 class="text-5xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent mb-4">
          🔬 EMC Spectrum Analyzer
        </h1>
        <p class="text-xl text-gray-600 max-w-2xl mx-auto">
          Analyze EMC compliance with international standards using WebAssembly for lightning-fast performance
        </p>
      </div>

      <!-- WASM Loading Status -->
      <div v-if="wasm.isLoading" class="text-center mb-8">
        <div class="inline-flex items-center px-6 py-3 bg-blue-100 rounded-lg text-blue-700">
          <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-blue-700" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          Loading WASM module...
        </div>
      </div>

      <!-- WASM Success Status -->
      <div v-if="!wasm.isLoading && !wasm.error && wasm.wasmModule" class="text-center mb-8">
        <div class="inline-flex items-center px-6 py-3 bg-green-100 rounded-lg text-green-700">
          ✅ WASM module loaded successfully!
        </div>
      </div>

      <!-- Error State -->
      <div v-if="wasm.error" class="text-center mb-8">
        <div class="inline-flex items-center px-6 py-3 bg-red-100 rounded-lg text-red-700">
          ❌ {{ wasm.error }}
        </div>
        <div class="mt-4 text-center">
          <button 
            @click="wasm.initWasm()" 
            class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700"
          >
            🔄 Retry WASM Loading
          </button>
        </div>
      </div>

      <!-- Demo Links Section -->
      <div class="mb-8">
        <h2 class="text-3xl font-bold text-center mb-6 text-gray-800">
          🎯 Demo Pages
        </h2>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
          <DemoCard 
            title="Vue EMC Prototype"
            description="Complete EMC analysis interface with Vue components"
            url="/vue-emc-prototype.html"
            icon="🔬"
          />
          <DemoCard 
            title="Vue WASM Demo"
            description="Vue + WebAssembly integration demonstration"
            url="/vue-emc-wasm-demo.html"
            icon="🦀"
          />
          <DemoCard 
            title="Pure Vue WASM Test"
            description="Minimal Vue + WASM test implementation"
            url="/pure-vue-wasm-test.html"
            icon="⚡"
          />
          <DemoCard 
            title="Simple WASM Test"
            description="Basic WebAssembly functionality test"
            url="/simple-wasm-test.html"
            icon="🧪"
          />
          <DemoCard 
            title="Basic Test"
            description="Fundamental test page for debugging"
            url="/test.html"
            icon="🔧"
          />
          <DemoCard 
            title="Jupyter Notebook"
            description="Complete validation workflow with Python"
            url="/notebooks"
            icon="📓"
            external="true"
          />
        </div>
      </div>

      <!-- Main Content -->
      <div v-if="!wasm.isLoading && !wasm.error">
        <!-- Control Panel -->
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 mb-8">
          <EMCStandardLoader />
          <EMCFileUpload />
        </div>

        <!-- Chart -->
        <div class="mb-8">
          <EMCChart 
            :standard="emcStore.currentStandard" 
            :measurement-data="emcStore.measurementData"
          />
        </div>

        <!-- Results -->
        <ResultsTable />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const emcStore = useEMCStore()
const wasm = useWasm()

// Set page metadata
useHead({
  title: 'EMC Spectrum Analyzer - WASM Powered',
  meta: [
    { name: 'description', content: 'Fast EMC compliance analysis using WebAssembly' }
  ]
})

// Client-side only initialization
onMounted(() => {
  if (process.client) {
    console.log('🚀 EMC Analyzer initialized in client')
  }
})
</script>