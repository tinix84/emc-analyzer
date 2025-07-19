/**
 * Test suite for EMC Standards JSON validation
 * Ensures the JSON file is valid and follows the expected schema
 */

import { describe, it, expect, beforeAll } from 'vitest'
import fs from 'fs'
import path from 'path'

let emcStandards = null

describe('EMC Standards JSON Tests', () => {
  beforeAll(async () => {
    // Load the JSON file
    const jsonPath = path.resolve('./public/emc-standards.json')
    const jsonContent = fs.readFileSync(jsonPath, 'utf-8')
    
    // Test basic JSON parsing
    try {
      emcStandards = JSON.parse(jsonContent)
    } catch (error) {
      throw new Error(`JSON parsing failed: ${error.message}`)
    }
  })

  it('should be valid JSON', () => {
    expect(emcStandards).toBeDefined()
    expect(emcStandards).toBeTypeOf('object')
  })

  it('should have a "standards" property', () => {
    expect(emcStandards).toHaveProperty('standards')
    expect(emcStandards.standards).toBeTypeOf('object')
  })

  it('should have at least 20 standards', () => {
    const standardsCount = Object.keys(emcStandards.standards).length
    expect(standardsCount).toBeGreaterThan(20)
    console.log(`Found ${standardsCount} EMC standards`)
  })

  describe('Individual Standard Validation', () => {
    it('should have valid structure for each standard', () => {
      const requiredFields = ['name', 'description', 'type', 'subtype', 'measurement_type', 'power_range', 'class', 'limits']
      
      Object.entries(emcStandards.standards).forEach(([key, standard]) => {
        // Check required fields
        requiredFields.forEach(field => {
          expect(standard).toHaveProperty(field, `Standard "${key}" missing field: ${field}`)
        })

        // Validate field types
        expect(standard.name).toBeTypeOf('string')
        expect(standard.description).toBeTypeOf('string')
        expect(standard.type).toBeTypeOf('string')
        expect(standard.subtype).toBeTypeOf('string')
        expect(standard.measurement_type).toBeTypeOf('string')
        expect(standard.power_range).toBeTypeOf('string')
        expect(standard.class).toBeTypeOf('string')
        expect(standard.limits).toBeTypeOf('object')
      })
    })

    it('should have valid emission types', () => {
      const validTypes = ['conducted', 'radiated']
      
      Object.entries(emcStandards.standards).forEach(([key, standard]) => {
        expect(validTypes).toContain(standard.type, 
          `Standard "${key}" has invalid type: ${standard.type}`)
      })
    })

    it('should have valid subtypes for each emission type', () => {
      const validSubtypes = {
        conducted: ['power_ports', 'mains_ports', 'ac_lines', 'dc_lines', 'telecom_lan_ports'],
        radiated: ['free_space']
      }
      
      Object.entries(emcStandards.standards).forEach(([key, standard]) => {
        const expectedSubtypes = validSubtypes[standard.type]
        expect(expectedSubtypes).toContain(standard.subtype, 
          `Standard "${key}" has invalid subtype "${standard.subtype}" for type "${standard.type}"`)
      })
    })

    it('should have valid measurement types', () => {
      const validMeasurementTypes = ['voltage', 'current', 'field_strength']
      
      Object.entries(emcStandards.standards).forEach(([key, standard]) => {
        expect(validMeasurementTypes).toContain(standard.measurement_type, 
          `Standard "${key}" has invalid measurement_type: ${standard.measurement_type}`)
      })
    })

    it('should have valid limit types', () => {
      const validLimitTypes = ['avg', 'qp', 'pk']
      
      Object.entries(emcStandards.standards).forEach(([key, standard]) => {
        const limitTypes = Object.keys(standard.limits)
        expect(limitTypes.length).toBeGreaterThan(0, 
          `Standard "${key}" has no limit types`)
        
        limitTypes.forEach(limitType => {
          expect(validLimitTypes).toContain(limitType, 
            `Standard "${key}" has invalid limit type: ${limitType}`)
        })
      })
    })

    it('should have valid limit data points', () => {
      Object.entries(emcStandards.standards).forEach(([key, standard]) => {
        Object.entries(standard.limits).forEach(([limitType, points]) => {
          expect(Array.isArray(points)).toBe(true, 
            `Standard "${key}" limit "${limitType}" should be an array`)
          
          expect(points.length).toBeGreaterThan(0, 
            `Standard "${key}" limit "${limitType}" should have data points`)
          
          points.forEach((point, index) => {
            expect(Array.isArray(point)).toBe(true, 
              `Standard "${key}" limit "${limitType}" point ${index} should be an array`)
            
            expect(point.length).toBe(2, 
              `Standard "${key}" limit "${limitType}" point ${index} should have exactly 2 values [frequency, level]`)
            
            expect(typeof point[0]).toBe('number', 
              `Standard "${key}" limit "${limitType}" point ${index} frequency should be a number`)
            
            expect(typeof point[1]).toBe('number', 
              `Standard "${key}" limit "${limitType}" point ${index} level should be a number`)
            
            expect(point[0]).toBeGreaterThan(0, 
              `Standard "${key}" limit "${limitType}" point ${index} frequency should be positive`)
          })
        })
      })
    })

    it('should have ascending frequency points in limits', () => {
      Object.entries(emcStandards.standards).forEach(([key, standard]) => {
        Object.entries(standard.limits).forEach(([limitType, points]) => {
          for (let i = 1; i < points.length; i++) {
            expect(points[i][0]).toBeGreaterThan(points[i-1][0], 
              `Standard "${key}" limit "${limitType}" frequencies should be in ascending order`)
          }
        })
      })
    })
  })

  describe('Data Consistency Tests', () => {
    it('should have distance field for radiated standards', () => {
      Object.entries(emcStandards.standards).forEach(([key, standard]) => {
        if (standard.type === 'radiated') {
          expect(standard).toHaveProperty('distance', 
            `Radiated standard "${key}" should have distance field`)
          expect(['3m', '10m']).toContain(standard.distance,
            `Standard "${key}" has invalid distance: ${standard.distance}`)
        }
      })
    })

    it('should have N/A distance for conducted standards', () => {
      Object.entries(emcStandards.standards).forEach(([key, standard]) => {
        if (standard.type === 'conducted') {
          if (standard.hasOwnProperty('distance')) {
            expect(standard.distance).toBe('N/A', 
              `Conducted standard "${key}" should have distance as "N/A" or no distance field`)
          }
        }
      })
    })

    it('should have consistent class values', () => {
      const validClasses = ['A', 'B', 'automotive', 'consumer', 'non_consumer', 'special']
      
      Object.entries(emcStandards.standards).forEach(([key, standard]) => {
        expect(validClasses).toContain(standard.class, 
          `Standard "${key}" has invalid class: ${standard.class}`)
      })
    })

    it('should have reasonable frequency ranges', () => {
      Object.entries(emcStandards.standards).forEach(([key, standard]) => {
        Object.entries(standard.limits).forEach(([limitType, points]) => {
          points.forEach((point, index) => {
            const frequency = point[0]
            expect(frequency).toBeGreaterThan(0.001, 
              `Standard "${key}" frequency ${frequency} MHz seems too low`)
            expect(frequency).toBeLessThan(10000, 
              `Standard "${key}" frequency ${frequency} MHz seems too high`)
          })
        })
      })
    })

    it('should have reasonable emission levels', () => {
      Object.entries(emcStandards.standards).forEach(([key, standard]) => {
        Object.entries(standard.limits).forEach(([limitType, points]) => {
          points.forEach((point, index) => {
            const level = point[1]
            expect(level).toBeGreaterThan(0, 
              `Standard "${key}" emission level ${level} should be positive`)
            expect(level).toBeLessThan(200, 
              `Standard "${key}" emission level ${level} dBμV seems unreasonably high`)
          })
        })
      })
    })
  })

  describe('Schema Completeness', () => {
    it('should count standards by type', () => {
      const typeCount = { conducted: 0, radiated: 0 }
      
      Object.values(emcStandards.standards).forEach(standard => {
        typeCount[standard.type]++
      })
      
      console.log('Standards by type:', typeCount)
      expect(typeCount.conducted).toBeGreaterThan(0)
      expect(typeCount.radiated).toBeGreaterThan(0)
    })

    it('should count standards by class', () => {
      const classCount = {}
      
      Object.values(emcStandards.standards).forEach(standard => {
        classCount[standard.class] = (classCount[standard.class] || 0) + 1
      })
      
      console.log('Standards by class:', classCount)
      expect(Object.keys(classCount).length).toBeGreaterThan(0)
    })

    it('should list all unique subtypes', () => {
      const subtypes = new Set()
      
      Object.values(emcStandards.standards).forEach(standard => {
        subtypes.add(standard.subtype)
      })
      
      console.log('All subtypes:', Array.from(subtypes).sort())
      expect(subtypes.size).toBeGreaterThan(0)
    })
  })

  describe('JSON Size and Performance', () => {
    it('should not be excessively large', () => {
      const jsonString = JSON.stringify(emcStandards)
      const sizeKB = jsonString.length / 1024
      
      console.log(`JSON file size: ${sizeKB.toFixed(1)} KB`)
      expect(sizeKB).toBeLessThan(500) // Should be under 500KB
    })

    it('should parse quickly', () => {
      const startTime = performance.now()
      
      // Parse the JSON multiple times to test performance
      for (let i = 0; i < 10; i++) {
        const jsonPath = path.resolve('./public/emc-standards.json')
        const jsonContent = fs.readFileSync(jsonPath, 'utf-8')
        JSON.parse(jsonContent)
      }
      
      const endTime = performance.now()
      const parseTime = endTime - startTime
      
      console.log(`10x parse time: ${parseTime.toFixed(2)}ms`)
      expect(parseTime).toBeLessThan(100) // Should parse quickly
    })
  })
})
