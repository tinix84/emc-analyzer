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
      
      // Dynamic import of WASM module
      const wasm = await import('~/public/wasm/emc_wasm.js')
      await wasm.default()
      wasm.init_panic_hook()
      
      wasmModule.value = wasm
      console.log('WASM module loaded successfully')
    } catch (err: any) {
      error.value = `Failed to load WASM: ${err.message}`
      console.error('WASM loading error:', err)
    } finally {
      isLoading.value = false
    }
  }

  const getStandard = (standardName: string, emcClass: string, interfaceType?: string): EMCStandard => {
    if (!wasmModule.value) {
      throw new Error('WASM module not loaded')
    }

    const result = wasmModule.value.get_emc_standard(standardName, emcClass, interfaceType)
    return result
  }

  const calculateLimit = (standard: EMCStandard, frequency: number): EMCLimitResult => {
    if (!wasmModule.value) {
      throw new Error('WASM module not loaded')
    }

    const standardJson = JSON.stringify(standard)
    return wasmModule.value.calculate_emc_limit(standardJson, frequency)
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

    const standardJson = JSON.stringify(standard)
    return wasmModule.value.check_emc_compliance(standardJson, frequencies, amplitudes, measurementType)
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

    const standardJson = JSON.stringify(standard)
    return wasmModule.value.generate_emc_mask(standardJson, fMin, fMax, pointsPerDecade)
  }

  // Auto-initialize when composable is used
  if (process.client && !wasmModule.value) {
    initWasm()
  }

  return {
    wasmModule: readonly(wasmModule),
    isLoading: readonly(isLoading),
    error: readonly(error),
    initWasm,
    getStandard,
    calculateLimit,
    checkCompliance,
    generateMask
  }
}

