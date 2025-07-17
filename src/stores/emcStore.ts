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
  
  // Available EMC Standards - loaded from JSON
  const standards = ref<EMCStandard[]>([])
  
  // Load standards from JSON file
  const loadStandards = async () => {
    try {
      const response = await fetch('/emc-standards.json')
      const data = await response.json()
      
      console.log('📋 Loading standards from JSON:', Object.keys(data.standards))
      
      const loadedStandards: EMCStandard[] = []
      
      for (const [key, standard] of Object.entries(data.standards)) {
        const stdData = standard as any
        
        // Convert the avg limits to frequency ranges for backward compatibility
        const frequencyRanges = stdData.limits.avg.map((point: [number, number]) => ({
          startFreq: point[0],
          endFreq: point[0], // For now, we'll use the same frequency as start and end
          limit: point[1]
        }))
        
        loadedStandards.push({
          id: key,
          name: stdData.name,
          description: stdData.description,
          frequencyRanges
        })
      }
      
      standards.value = loadedStandards
      console.log('✅ Loaded', loadedStandards.length, 'standards from JSON')
      
    } catch (error) {
      console.error('❌ Failed to load standards from JSON:', error)
      // Fallback to hardcoded standards if JSON loading fails
      standards.value = [
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
        }
      ]
    }
  }

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

  // Initialize from storage and load standards
  loadFromStorage()
  loadStandards()

  const getStandardMasks = async (standardId: string) => {
    console.log('🏪 Store: Getting masks for standard:', standardId)
    
    try {
      const response = await fetch('/emc-standards.json')
      const data = await response.json()
      const standardData = data.standards[standardId]
      
      if (!standardData) {
        console.log('🏪 Store: Standard not found in JSON!')
        return {}
      }

      console.log('🏪 Store: Standard found in JSON:', standardData.name)
      const masks: { [key: string]: MeasurementPoint[] } = {}

      // Generate masks for each limit type (avg, qp, pk)
      Object.entries(standardData.limits).forEach(([limitType, points]) => {
        if (!points || !Array.isArray(points) || points.length === 0) {
          console.log(`⚠️ No ${limitType.toUpperCase()} limits found for ${standardId}`)
          return
        }
        
        const mask: MeasurementPoint[] = []
        const pointsArray = points as [number, number][]
        
        // Filter out null values and invalid points
        const validPoints = pointsArray.filter(([freq, limit]) => 
          freq !== null && limit !== null && 
          typeof freq === 'number' && typeof limit === 'number'
        )
        
        if (validPoints.length === 0) {
          console.log(`⚠️ No valid ${limitType.toUpperCase()} points for ${standardId}`)
          return
        }
        
        // Create interpolated points between the defined limit points
        for (let i = 0; i < validPoints.length - 1; i++) {
          const [freq1, limit1] = validPoints[i]
          const [freq2, limit2] = validPoints[i + 1]
          
          // Add start point
          mask.push({ frequency: freq1, amplitude: limit1 })
          
          // Add interpolated points for smooth visualization
          const steps = 50
          for (let j = 1; j < steps; j++) {
            const ratio = j / steps
            const freq = freq1 + (freq2 - freq1) * ratio
            const limit = limit1 + (limit2 - limit1) * ratio
            mask.push({ frequency: freq, amplitude: limit })
          }
        }
        
        // Add final point
        const lastPoint = validPoints[validPoints.length - 1]
        mask.push({ frequency: lastPoint[0], amplitude: lastPoint[1] })
        
        masks[limitType] = mask
        console.log(`✅ Generated ${limitType.toUpperCase()} mask with ${mask.length} points`)
      })

      return masks
    } catch (error) {
      console.error('❌ Error loading standard masks from JSON:', error)
      return {}
    }
  }

  // Legacy function for backward compatibility
  const getStandardMask = (standardId: string) => {
    console.log('🏪 Store: Getting legacy mask for standard:', standardId)
    const standard = standards.value.find(s => s.id === standardId)
    if (!standard) {
      console.log('🏪 Store: Standard not found!')
      return []
    }

    console.log('🏪 Store: Standard found:', standard.name, 'with', standard.frequencyRanges.length, 'ranges')
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

    console.log('🏪 Store: Generated legacy mask with', mask.length, 'points')
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
    getStandardMasks,
    clearData,
    loadStandards,
    
    // Getters
    complianceStatus
  }
})
