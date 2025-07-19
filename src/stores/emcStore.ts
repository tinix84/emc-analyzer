import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export interface MeasurementPoint {
  frequency: number
  amplitude: number
  peak?: number
  avg?: number
  qp?: number
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
        
        // Use the first available limit type (avg, qp, or pk) for frequency ranges
        let frequencyRanges: Array<{startFreq: number, endFreq: number, limit: number}> = []
        
        if (stdData.limits.avg && stdData.limits.avg.length > 0) {
          frequencyRanges = stdData.limits.avg.map((point: [number, number]) => ({
            startFreq: point[0],
            endFreq: point[0], // For now, we'll use the same frequency as start and end
            limit: point[1]
          }))
        } else if (stdData.limits.qp && stdData.limits.qp.length > 0) {
          frequencyRanges = stdData.limits.qp.map((point: [number, number]) => ({
            startFreq: point[0],
            endFreq: point[0],
            limit: point[1]
          }))
        } else if (stdData.limits.pk && stdData.limits.pk.length > 0) {
          frequencyRanges = stdData.limits.pk
            .filter((point: [number, number | null]) => point[1] !== null)
            .map((point: [number, number]) => ({
              startFreq: point[0],
              endFreq: point[0],
              limit: point[1]
            }))
        }
        
        if (frequencyRanges.length > 0) {
          loadedStandards.push({
            id: key,
            name: stdData.name,
            description: stdData.description,
            frequencyRanges
          })
        } else {
          console.warn('⚠️ Standard', key, 'has no valid limits, skipping')
        }
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
      console.log('🔄 Using fallback standard')
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
        
        // Create logarithmically interpolated points between the defined limit points
        for (let i = 0; i < validPoints.length - 1; i++) {
          const [freq1, limit1] = validPoints[i]
          const [freq2, limit2] = validPoints[i + 1]
          
          // Add start point
          mask.push({ frequency: freq1, amplitude: limit1 })
          
          // Logarithmic interpolation for frequency, linear for amplitude
          const steps = 100 // More steps for smoother curves on log scale
          const logFreq1 = Math.log10(freq1)
          const logFreq2 = Math.log10(freq2)
          
          for (let j = 1; j < steps; j++) {
            const ratio = j / steps
            
            // Logarithmic interpolation for frequency
            const logFreq = logFreq1 + (logFreq2 - logFreq1) * ratio
            const freq = Math.pow(10, logFreq)
            
            // Linear interpolation for amplitude (dB values)
            const limit = limit1 + (limit2 - limit1) * ratio
            
            mask.push({ frequency: freq, amplitude: limit })
          }
        }
        
        // Add final point
        const lastPoint = validPoints[validPoints.length - 1]
        mask.push({ frequency: lastPoint[0], amplitude: lastPoint[1] })
        
        // Sort by frequency to ensure proper ordering
        mask.sort((a, b) => a.frequency - b.frequency)
        
        // Add additional logarithmically spaced points for better comparison
        const firstFreq = validPoints[0][0]
        const lastFreq = validPoints[validPoints.length - 1][0]
        const additionalPoints = generateLogSpacedPoints(firstFreq, lastFreq, mask, 50)
        mask.push(...additionalPoints)
        
        // Final sort and remove duplicates
        mask.sort((a, b) => a.frequency - b.frequency)
        const uniqueMask = removeDuplicateFrequencies(mask)
        
        masks[limitType] = uniqueMask
        console.log(`✅ Generated ${limitType.toUpperCase()} mask with ${uniqueMask.length} points`)
      })

      return masks
    } catch (error) {
      console.error('❌ Error loading standard masks from JSON:', error)
      return {}
    }
  }

  // Helper function to generate logarithmically spaced points
  const generateLogSpacedPoints = (minFreq: number, maxFreq: number, existingMask: MeasurementPoint[], numPoints: number): MeasurementPoint[] => {
    const logMin = Math.log10(minFreq)
    const logMax = Math.log10(maxFreq)
    const logStep = (logMax - logMin) / (numPoints - 1)
    const additionalPoints: MeasurementPoint[] = []
    
    for (let i = 0; i < numPoints; i++) {
      const logFreq = logMin + i * logStep
      const freq = Math.pow(10, logFreq)
      
      // Interpolate amplitude from existing mask points
      const amplitude = interpolateAmplitude(freq, existingMask)
      if (amplitude !== null) {
        additionalPoints.push({ frequency: freq, amplitude })
      }
    }
    
    return additionalPoints
  }

  // Helper function to interpolate amplitude at a given frequency
  const interpolateAmplitude = (targetFreq: number, maskPoints: MeasurementPoint[]): number | null => {
    if (maskPoints.length === 0) return null
    
    // Find the two closest points for interpolation
    let lowerPoint = maskPoints[0]
    let upperPoint = maskPoints[maskPoints.length - 1]
    
    for (let i = 0; i < maskPoints.length - 1; i++) {
      if (maskPoints[i].frequency <= targetFreq && maskPoints[i + 1].frequency >= targetFreq) {
        lowerPoint = maskPoints[i]
        upperPoint = maskPoints[i + 1]
        break
      }
    }
    
    if (lowerPoint.frequency === upperPoint.frequency) {
      return lowerPoint.amplitude
    }
    
    // Linear interpolation in amplitude (dB values)
    const ratio = (targetFreq - lowerPoint.frequency) / (upperPoint.frequency - lowerPoint.frequency)
    return lowerPoint.amplitude + (upperPoint.amplitude - lowerPoint.amplitude) * ratio
  }

  // Helper function to remove duplicate frequencies (keep the first occurrence)
  const removeDuplicateFrequencies = (maskPoints: MeasurementPoint[]): MeasurementPoint[] => {
    const seen = new Set<number>()
    const unique: MeasurementPoint[] = []
    
    for (const point of maskPoints) {
      const roundedFreq = Math.round(point.frequency * 1000) / 1000 // Round to 3 decimal places
      if (!seen.has(roundedFreq)) {
        seen.add(roundedFreq)
        unique.push(point)
      }
    }
    
    return unique
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
      // Generate points for smooth mask visualization with logarithmic interpolation
      const steps = 100
      const logStart = Math.log10(range.startFreq)
      const logEnd = Math.log10(range.endFreq)
      const logStep = (logEnd - logStart) / steps
      
      for (let i = 0; i <= steps; i++) {
        const logFreq = logStart + (i * logStep)
        const freq = Math.pow(10, logFreq)
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

    // Check if we have multi-column data
    const hasMultiColumns = measurementData.value.some(point => 
      point.peak !== undefined || point.avg !== undefined || point.qp !== undefined
    )

    if (hasMultiColumns) {
      // Separate compliance for Peak, QP, and Average
      const peakViolations = measurementData.value.filter(point => {
        if (point.peak === undefined) return false
        const applicableRange = currentStandard.value!.frequencyRanges.find(
          range => point.frequency >= range.startFreq && point.frequency <= range.endFreq
        )
        
        if (!applicableRange) return false
        
        return point.peak > applicableRange.limit
      })

      const qpViolations = measurementData.value.filter(point => {
        if (point.qp === undefined) return false
        const applicableRange = currentStandard.value!.frequencyRanges.find(
          range => point.frequency >= range.startFreq && point.frequency <= range.endFreq
        )
        
        if (!applicableRange) return false
        
        return point.qp > applicableRange.limit
      })

      const avgViolations = measurementData.value.filter(point => {
        if (point.avg === undefined) return false
        const applicableRange = currentStandard.value!.frequencyRanges.find(
          range => point.frequency >= range.startFreq && point.frequency <= range.endFreq
        )
        
        if (!applicableRange) return false
        
        return point.avg > applicableRange.limit
      })

      return {
        isCompliant: peakViolations.length === 0 && qpViolations.length === 0 && avgViolations.length === 0,
        violations: peakViolations.length + qpViolations.length + avgViolations.length,
        totalPoints: [
          measurementData.value.filter(p => p.peak !== undefined).length,
          measurementData.value.filter(p => p.qp !== undefined).length,
          measurementData.value.filter(p => p.avg !== undefined).length
        ].reduce((sum, count) => sum + count, 0),
        peakViolations: peakViolations.length,
        qpViolations: qpViolations.length,
        avgViolations: avgViolations.length,
        hasMultiColumns: true
      }
    } else {
      // Standard compliance for 2-column data
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
        totalPoints: measurementData.value.length,
        hasMultiColumns: false
      }
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
