// stores/emc.ts
export const useEMCStore = defineStore('emc', () => {
  const { getStandard, checkCompliance, generateMask } = useWasm()
  
  const currentStandard = ref<EMCStandard | null>(null)
  const measurementData = ref<SpectrumData | null>(null)
  const complianceResults = ref<ComplianceResult[]>([])
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  const loadStandard = async (standardName: string, emcClass: string, interfaceType?: string) => {
    try {
      isLoading.value = true
      error.value = null
      
      currentStandard.value = getStandard(standardName, emcClass, interfaceType)
      
      console.log(`Standard "${currentStandard.value.name}" loaded successfully`)
    } catch (err: any) {
      error.value = err.message || 'Failed to load standard'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  const setMeasurementData = (data: SpectrumData) => {
    measurementData.value = data
  }

  const analyzeCompliance = async (measurementType: string) => {
    if (!currentStandard.value || !measurementData.value) {
      throw new Error('Standard and measurement data required')
    }

    try {
      isLoading.value = true
      error.value = null
      
      complianceResults.value = checkCompliance(
        currentStandard.value,
        measurementData.value.frequencies,
        measurementData.value.amplitudes,
        measurementType
      )
      
      console.log(`Compliance analysis completed: ${complianceResults.value.length} points analyzed`)
    } catch (err: any) {
      error.value = err.message || 'Failed to analyze compliance'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  const generateMaskData = async (fMin = 150000, fMax = 30000000, pointsPerDecade = 20) => {
    if (!currentStandard.value) {
      throw new Error('No standard loaded')
    }

    try {
      return generateMask(currentStandard.value, fMin, fMax, pointsPerDecade)
    } catch (err: any) {
      error.value = err.message || 'Failed to generate mask'
      throw err
    }
  }

  const clearResults = () => {
    complianceResults.value = []
    error.value = null
  }

  return {
    currentStandard: readonly(currentStandard),
    measurementData: readonly(measurementData),
    complianceResults: readonly(complianceResults),
    isLoading: readonly(isLoading),
    error: readonly(error),
    loadStandard,
    setMeasurementData,
    analyzeCompliance,
    generateMaskData,
    clearResults
  }
})

