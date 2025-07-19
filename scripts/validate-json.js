#!/usr/bin/env node

/**
 * Standalone JSON Validation Script for EMC Standards
 * Usage: node scripts/validate-json.js
 */

import fs from 'fs'
import path from 'path'
import { fileURLToPath } from 'url'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

console.log('🔍 EMC Standards JSON Validation')
console.log('================================\n')

async function validateJSON() {
  const jsonPath = path.resolve(__dirname, '../public/emc-standards.json')
  
  try {
    // Check if file exists
    if (!fs.existsSync(jsonPath)) {
      console.error('❌ ERROR: emc-standards.json file not found!')
      process.exit(1)
    }
    
    console.log('📁 File found:', jsonPath)
    
    // Read and parse JSON
    const jsonContent = fs.readFileSync(jsonPath, 'utf-8')
    console.log('📊 File size:', (jsonContent.length / 1024).toFixed(1), 'KB')
    
    const startTime = Date.now()
    let standards
    
    try {
      standards = JSON.parse(jsonContent)
      const parseTime = Date.now() - startTime
      console.log('⚡ Parse time:', parseTime, 'ms')
    } catch (parseError) {
      console.error('❌ JSON PARSE ERROR:', parseError.message)
      console.error('Line:', parseError.message.match(/position (\d+)/)?.[1] || 'unknown')
      process.exit(1)
    }
    
    console.log('✅ JSON is valid!')
    
    // Structure validation
    if (!standards.standards) {
      console.error('❌ ERROR: Missing "standards" property')
      process.exit(1)
    }
    
    const standardsCount = Object.keys(standards.standards).length
    console.log('📈 Standards count:', standardsCount)
    
    if (standardsCount === 0) {
      console.error('❌ ERROR: No standards found!')
      process.exit(1)
    }
    
    // Validate each standard
    let errors = 0
    let warnings = 0
    
    const requiredFields = ['name', 'description', 'type', 'subtype', 'measurement_type', 'power_range', 'class', 'limits']
    const validTypes = ['conducted', 'radiated']
    const validSubtypes = {
      conducted: ['power_ports', 'mains_ports', 'ac_lines', 'dc_lines', 'telecom_lan_ports'],
      radiated: ['free_space']
    }
    const validMeasurementTypes = ['voltage', 'current', 'field_strength']
    const validLimitTypes = ['avg', 'qp', 'pk']
    
    console.log('\n🔍 Validating individual standards...')
    
    Object.entries(standards.standards).forEach(([key, standard], index) => {
      const stdNum = `[${index + 1}/${standardsCount}]`
      
      // Check required fields
      requiredFields.forEach(field => {
        if (!standard.hasOwnProperty(field)) {
          console.error(`❌ ${stdNum} "${key}": Missing field "${field}"`)
          errors++
        }
      })
      
      // Validate field types and values
      if (standard.type && !validTypes.includes(standard.type)) {
        console.error(`❌ ${stdNum} "${key}": Invalid type "${standard.type}"`)
        errors++
      }
      
      if (standard.subtype && standard.type) {
        const expectedSubtypes = validSubtypes[standard.type]
        if (!expectedSubtypes?.includes(standard.subtype)) {
          console.error(`❌ ${stdNum} "${key}": Invalid subtype "${standard.subtype}" for type "${standard.type}"`)
          errors++
        }
      }
      
      if (standard.measurement_type && !validMeasurementTypes.includes(standard.measurement_type)) {
        console.error(`❌ ${stdNum} "${key}": Invalid measurement_type "${standard.measurement_type}"`)
        errors++
      }
      
      // Validate limits structure
      if (standard.limits) {
        const limitTypes = Object.keys(standard.limits)
        
        if (limitTypes.length === 0) {
          console.error(`❌ ${stdNum} "${key}": No limit types found`)
          errors++
        }
        
        limitTypes.forEach(limitType => {
          if (!validLimitTypes.includes(limitType)) {
            console.error(`❌ ${stdNum} "${key}": Invalid limit type "${limitType}"`)
            errors++
          }
          
          const points = standard.limits[limitType]
          if (!Array.isArray(points) || points.length === 0) {
            console.error(`❌ ${stdNum} "${key}": Invalid or empty limit data for "${limitType}"`)
            errors++
            return
          }
          
          // Validate data points
          points.forEach((point, pointIndex) => {
            if (!Array.isArray(point) || point.length !== 2) {
              console.error(`❌ ${stdNum} "${key}": Invalid data point ${pointIndex} in "${limitType}"`)
              errors++
              return
            }
            
            const [frequency, level] = point
            if (typeof frequency !== 'number' || typeof level !== 'number') {
              console.error(`❌ ${stdNum} "${key}": Non-numeric values in "${limitType}" point ${pointIndex}`)
              errors++
            }
            
            if (frequency <= 0) {
              console.error(`❌ ${stdNum} "${key}": Invalid frequency ${frequency} in "${limitType}" point ${pointIndex}`)
              errors++
            }
            
            if (level <= 0 || level > 200) {
              console.warn(`⚠️  ${stdNum} "${key}": Unusual level ${level} dBμV in "${limitType}" point ${pointIndex}`)
              warnings++
            }
          })
          
          // Check frequency order
          for (let i = 1; i < points.length; i++) {
            if (points[i][0] <= points[i-1][0]) {
              console.error(`❌ ${stdNum} "${key}": Frequencies not in ascending order in "${limitType}"`)
              errors++
              break
            }
          }
        })
      }
      
      // Type-specific validations
      if (standard.type === 'radiated' && (!standard.distance || !['3m', '10m'].includes(standard.distance))) {
        console.warn(`⚠️  ${stdNum} "${key}": Radiated standard should have distance field (3m or 10m)`)
        warnings++
      }
      
      if (index < 5) { // Show first few standards
        console.log(`✅ ${stdNum} "${standard.name}" - ${standard.type}/${standard.subtype}`)
      }
    })
    
    // Summary statistics
    console.log('\n📊 Summary Statistics:')
    
    const typeCount = { conducted: 0, radiated: 0 }
    const classCount = {}
    const subtypeCount = {}
    
    Object.values(standards.standards).forEach(standard => {
      typeCount[standard.type] = (typeCount[standard.type] || 0) + 1
      classCount[standard.class] = (classCount[standard.class] || 0) + 1
      subtypeCount[standard.subtype] = (subtypeCount[standard.subtype] || 0) + 1
    })
    
    console.log('   Types:', typeCount)
    console.log('   Classes:', classCount)
    console.log('   Subtypes:', subtypeCount)
    
    // Final result
    console.log('\n🎯 Validation Results:')
    console.log(`   Standards processed: ${standardsCount}`)
    console.log(`   Errors: ${errors}`)
    console.log(`   Warnings: ${warnings}`)
    
    if (errors > 0) {
      console.log('\n❌ VALIDATION FAILED!')
      console.log('Please fix the errors above before using the JSON file.')
      process.exit(1)
    } else if (warnings > 0) {
      console.log('\n⚠️  VALIDATION PASSED WITH WARNINGS')
      console.log('Consider reviewing the warnings above.')
    } else {
      console.log('\n✅ VALIDATION PASSED!')
      console.log('JSON file is valid and ready to use.')
    }
    
  } catch (error) {
    console.error('❌ UNEXPECTED ERROR:', error.message)
    process.exit(1)
  }
}

// Run validation
validateJSON().catch(console.error)
