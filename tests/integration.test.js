/**
 * Integration test for EMC Standards loading in the application
 */

import { describe, it, expect, beforeEach } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useEmcStore } from '../src/stores/emc.ts'

describe('EMC Store JSON Integration', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('should load EMC standards from JSON file', async () => {
    const emcStore = useEmcStore()
    
    // Initially no standards should be loaded
    expect(emcStore.availableStandards).toEqual([])
    
    // Mock fetch for the JSON file
    global.fetch = vi.fn(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({
          standards: {
            "TEST_STANDARD": {
              "name": "Test Standard",
              "description": "Test Description",
              "type": "conducted",
              "subtype": "mains_ports",
              "measurement_type": "voltage",
              "power_range": "any",
              "class": "A",
              "limits": {
                "avg": [[0.15, 66], [30, 60]],
                "qp": [[0.15, 79], [30, 73]]
              }
            }
          }
        })
      })
    )
    
    // Load standards
    await emcStore.loadStandards()
    
    // Check that standards were loaded
    expect(emcStore.availableStandards.length).toBeGreaterThan(0)
    expect(emcStore.availableStandards[0]).toHaveProperty('name')
    expect(emcStore.availableStandards[0]).toHaveProperty('limits')
  })

  it('should handle JSON loading errors gracefully', async () => {
    const emcStore = useEmcStore()
    
    // Mock failed fetch
    global.fetch = vi.fn(() =>
      Promise.reject(new Error('Network error'))
    )
    
    // Attempt to load standards
    await emcStore.loadStandards()
    
    // Should handle error gracefully
    expect(emcStore.availableStandards).toEqual([])
  })

  it('should validate standard structure', async () => {
    const emcStore = useEmcStore()
    
    // Mock fetch with malformed data
    global.fetch = vi.fn(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({
          standards: {
            "INVALID_STANDARD": {
              "name": "Invalid Standard",
              // Missing required fields
              "limits": "invalid_format"
            }
          }
        })
      })
    )
    
    await emcStore.loadStandards()
    
    // Should filter out invalid standards
    const validStandards = emcStore.availableStandards.filter(std => 
      std.limits && typeof std.limits === 'object'
    )
    
    expect(validStandards.length).toBeGreaterThan(0)
  })
})
