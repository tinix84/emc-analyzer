import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export interface MeasurementPoint {
  frequency: number
  amplitude: number
}

export interface EMCStandard {
  id: string
  name: string
  description: string
  frequencyRanges: Array<{
    startFreq: number
    endFreq: number
    limit: number
  }>
}

export const useEMCStore = defineStore('emc', () => {
  // State
  const measurementData = ref<MeasurementPoint[]>([])
  const currentStandard = ref<EMCStandard | null>(null)
  
  // Load from localStorage on init
  const loadFromStorage = () => {
    try {
      const savedData = localStorage.getItem('emc-measurement-data')
      const savedStandard = localStorage.getItem('emc-current-standard')
      
      if (savedData) {
        const data = JSON.parse(savedData)
        measurementData.value = data
        console.log('📂 Loaded measurement data from storage:', data.length, 'points')
      }
      
      if (savedStandard) {
        const standardId = savedStandard
        const standard = standards.value.find(s => s.id === standardId)
        if (standard) {
          currentStandard.value = standard
          console.log('📂 Loaded standard from storage:', standard.name)
        }
      }
    } catch (error) {
      console.warn('⚠️ Could not load data from storage:', error)
    }
  }
  
  // Save to localStorage
  const saveToStorage = () => {
    try {
      localStorage.setItem('emc-measurement-data', JSON.stringify(measurementData.value))
      if (currentStandard.value) {
        localStorage.setItem('emc-current-standard', currentStandard.value.id)
      }
      console.log('💾 Data saved to localStorage')
    } catch (error) {
      console.warn('⚠️ Could not save to storage:', error)
    }
  }
  
  // Available EMC Standards
  const standards = ref<EMCStandard[]>([
    {
      id: 'EN55032_CLASS_A',
      name: 'EN 55032 Class A',
      description: 'Information technology equipment - Radio disturbance characteristics - Limits and methods of measurement - Class A',
      frequencyRanges: [
        { startFreq: 0.15, endFreq: 0.5, limit: 79 },
        { startFreq: 0.5, endFreq: 5, limit: 73 },
        { startFreq: 5, endFreq: 30, limit: 73 },
        { startFreq: 30, endFreq: 230, limit: 40 },
        { startFreq: 230, endFreq: 1000, limit: 47 }
      ]
    },
    {
      id: 'EN55032_CLASS_B',
      name: 'EN 55032 Class B',
      description: 'Information technology equipment - Radio disturbance characteristics - Limits and methods of measurement - Class B',
      frequencyRanges: [
        { startFreq: 0.15, endFreq: 0.5, limit: 66 },
        { startFreq: 0.5, endFreq: 5, limit: 56 },
        { startFreq: 5, endFreq: 30, limit: 56 },
        { startFreq: 30, endFreq: 230, limit: 30 },
        { startFreq: 230, endFreq: 1000, limit: 37 }
      ]
    },
    {
      id: 'CISPR32_CLASS_A',
      name: 'CISPR 32 Class A',
      description: 'Electromagnetic compatibility of multimedia equipment - Emission requirements - Class A',
      frequencyRanges: [
        { startFreq: 0.15, endFreq: 0.5, limit: 79 },
        { startFreq: 0.5, endFreq: 5, limit: 73 },
        { startFreq: 5, endFreq: 30, limit: 73 },
        { startFreq: 30, endFreq: 230, limit: 40 },
        { startFreq: 230, endFreq: 1000, limit: 47 }
      ]
    },
    {
      id: 'CISPR32_CLASS_B',
      name: 'CISPR 32 Class B',
      description: 'Electromagnetic compatibility of multimedia equipment - Emission requirements - Class B',
      frequencyRanges: [
        { startFreq: 0.15, endFreq: 0.5, limit: 66 },
        { startFreq: 0.5, endFreq: 5, limit: 56 },
        { startFreq: 5, endFreq: 30, limit: 56 },
        { startFreq: 30, endFreq: 230, limit: 30 },
        { startFreq: 230, endFreq: 1000, limit: 37 }
      ]
    }
  ])

  // Actions
  const setMeasurementData = (data: MeasurementPoint[]) => {
    measurementData.value = data
    saveToStorage()
    console.log('📊 Measurement data updated:', data.length, 'points')
  }

  const setCurrentStandard = (standardId: string) => {
    const standard = standards.value.find(s => s.id === standardId)
    currentStandard.value = standard || null
    saveToStorage()
    console.log('📋 Standard updated:', standard?.name || 'None')
  }

  const clearData = () => {
    measurementData.value = []
    currentStandard.value = null
    localStorage.removeItem('emc-measurement-data')
    localStorage.removeItem('emc-current-standard')
    console.log('🗑️ All data cleared')
  }

  // Initialize from storage
  loadFromStorage()

  const getStandardMask = (standardId: string) => {
    const standard = standards.value.find(s => s.id === standardId)
    if (!standard) return []

    const mask: MeasurementPoint[] = []
    standard.frequencyRanges.forEach(range => {
      // Generate points for smooth mask visualization
      const steps = 100
      const freqStep = (range.endFreq - range.startFreq) / steps
      
      for (let i = 0; i <= steps; i++) {
        const freq = range.startFreq + (i * freqStep)
        mask.push({
          frequency: freq,
          amplitude: range.limit
        })
      }
    })

    return mask
  }

  // Getters
  const complianceStatus = computed(() => {
    if (!currentStandard.value || measurementData.value.length === 0) {
      return null
    }

    const violations = measurementData.value.filter(point => {
      const applicableRange = currentStandard.value!.frequencyRanges.find(
        range => point.frequency >= range.startFreq && point.frequency <= range.endFreq
      )
      
      if (!applicableRange) return false
      
      return point.amplitude > applicableRange.limit
    })

    return {
      isCompliant: violations.length === 0,
      violations: violations.length,
      totalPoints: measurementData.value.length
    }
  })

  return {
    // State
    measurementData,
    currentStandard,
    standards,
    
    // Actions
    setMeasurementData,
    setCurrentStandard,
    getStandardMask,
    clearData,
    
    // Getters
    complianceStatus
  }
})
