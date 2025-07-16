// composables/useWasm.ts
import type { EMCStandard, EMCLimitResult, ComplianceResult, EMCMask } from '~/types/emc'

export const useWasm = () => {
  const wasmModule = ref<any>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  const initWasm = async () => {
    try {
      isLoading.value = true
      error.value = null
      
      console.log('🔧 Starting WASM initialization...')
      
      // Check if we're on client side
      if (!process.client) {
        console.log('❌ Not on client side, skipping WASM init')
        return
      }
      
      // Dynamic import of WASM module
      console.log('📦 Importing WASM module...')
      const wasmImport = await import('/wasm/emc_wasm.js')
      console.log('✅ WASM module imported:', wasmImport)
      
      // Initialize WASM module using the default export function
      console.log('🔧 Calling WASM initialization function...')
      await wasmImport.default('/wasm/emc_wasm_bg.wasm')
      console.log('✅ WASM initialized successfully')
      
      // After initialization, the functions are available on the wasmImport object
      console.log('✅ Module validation passed, storing module...')
      wasmModule.value = wasmImport
      console.log('✅ wasmModule.value set to:', wasmModule.value)
      console.log('✅ Available functions:', Object.keys(wasmImport).filter(k => typeof wasmImport[k] === 'function'))
      
      // Verify that the essential functions exist
      if (!wasmImport.get_emc_standard || typeof wasmImport.get_emc_standard !== 'function') {
        throw new Error('get_emc_standard function not available')
      }
      
      console.log('🎉 WASM module stored and ready!')
    } catch (err: any) {
      error.value = `Failed to load WASM: ${err.message}`
      console.error('❌ WASM loading error:', err)
    } finally {
      isLoading.value = false
    }
  }

  const getStandard = (standardName: string, emcClass: string, interfaceType?: string): EMCStandard => {
    console.log('🔍 Getting standard called with:', { standardName, emcClass, interfaceType })
    
    if (!wasmModule.value) {
      console.error('❌ WASM module is null or undefined')
      throw new Error('WASM module not loaded')
    }

    if (typeof wasmModule.value.get_emc_standard !== 'function') {
      console.error('❌ get_emc_standard is not a function. Available functions:', 
        Object.keys(wasmModule.value).filter(k => typeof wasmModule.value[k] === 'function'))
      throw new Error('get_emc_standard function not available')
    }

    console.log('🔍 Calling WASM get_emc_standard...')
    try {
      const result = wasmModule.value.get_emc_standard(standardName, emcClass, interfaceType || null)
      console.log('✅ Standard result:', result)
      return result
    } catch (error) {
      console.error('❌ Error getting standard:', error)
      throw error
    }
  }

  const calculateLimit = (standard: EMCStandard, frequency: number): EMCLimitResult => {
    if (!wasmModule.value) {
      throw new Error('WASM module not loaded')
    }

    console.log('🧮 Calculating limit for frequency:', frequency)
    try {
      const standardJson = JSON.stringify(standard)
      const result = wasmModule.value.calculate_emc_limit(standardJson, frequency)
      console.log('✅ Limit calculation result:', result)
      return result
    } catch (error) {
      console.error('❌ Error calculating limit:', error)
      throw error
    }
  }

  const checkCompliance = (
    standard: EMCStandard,
    frequencies: number[],
    amplitudes: number[],
    measurementType: string
  ): ComplianceResult[] => {
    if (!wasmModule.value) {
      throw new Error('WASM module not loaded')
    }

    console.log('✅ Checking compliance with:', { frequencies: frequencies.length, amplitudes: amplitudes.length, measurementType })
    try {
      const standardJson = JSON.stringify(standard)
      const freqArray = new Float64Array(frequencies)
      const ampArray = new Float64Array(amplitudes)
      const result = wasmModule.value.check_emc_compliance(standardJson, freqArray, ampArray, measurementType)
      console.log('✅ Compliance check result:', result)
      return result
    } catch (error) {
      console.error('❌ Error checking compliance:', error)
      throw error
    }
  }

  const generateMask = (
    standard: EMCStandard,
    fMin: number = 150000,
    fMax: number = 30000000,
    pointsPerDecade: number = 20
  ): EMCMask => {
    if (!wasmModule.value) {
      throw new Error('WASM module not loaded')
    }

    console.log('🎭 Generating mask:', { fMin, fMax, pointsPerDecade })
    try {
      const standardJson = JSON.stringify(standard)
      const result = wasmModule.value.generate_emc_mask(standardJson, fMin, fMax, pointsPerDecade)
      console.log('✅ Mask generation result:', result)
      return result
    } catch (error) {
      console.error('❌ Error generating mask:', error)
      throw error
    }
  }

  // Auto-initialize when composable is used
  if (process.client && !wasmModule.value) {
    console.log('🚀 Auto-initializing WASM from composable...')
    initWasm()
  }

  return {
    wasmModule,
    isLoading,
    error,
    initWasm,
    getStandard,
    calculateLimit,
    checkCompliance,
    generateMask
  }
}

