# EMC Standards JSON Validation

This directory contains validation tools and tests for the EMC standards JSON database.

## Quick Validation

To quickly validate the JSON file:

```bash
npm run test:json
# or
npm run validate
# or
node scripts/validate-json.js
```

## Files Overview

### 📁 `/public/emc-standards.json`
The main EMC standards database containing 29+ standards with structured data including:
- **Type classification**: `conducted` vs `radiated` emissions
- **Subtype specification**: `power_ports`, `mains_ports`, `ac_lines`, etc.
- **Measurement details**: voltage, current, or field_strength
- **Limit data**: avg/qp/pk emission limits with frequency points

### 🧪 `/tests/emc-standards.test.js`
Comprehensive test suite using Vitest:
- JSON parsing validation
- Structure schema checking
- Data consistency tests
- Performance benchmarks
- Field validation (types, ranges, format)

### 🔍 `/scripts/validate-json.js`
Standalone validation script that checks:
- ✅ JSON syntax validity
- ✅ Required field presence  
- ✅ Data type validation
- ✅ Frequency point ordering
- ✅ Reasonable value ranges
- ✅ Type-specific requirements
- 📊 Statistics and summary

### 📋 `/schemas/emc-standards.schema.json` 
JSON Schema definition for formal validation:
- Field requirements and types
- Enum value constraints  
- Conditional validation rules
- Array structure requirements

### 🚀 `/.github/workflows/validate-json.yml`
GitHub Actions workflow that runs automatically:
- On every push to main/dev branches
- When JSON or validation files change
- Validates syntax, structure, and size limits

## JSON Structure

```json
{
  "standards": {
    "STANDARD_ID": {
      "name": "Human Readable Name",
      "description": "Detailed description",
      "type": "conducted|radiated",
      "subtype": "power_ports|mains_ports|ac_lines|dc_lines|telecom_lan_ports|free_space",
      "measurement_type": "voltage|current|field_strength",
      "distance": "3m|10m|N/A",
      "power_range": "Power specification",
      "class": "A|B|automotive|consumer|non_consumer|special",
      "limits": {
        "avg|qp|pk": [
          [frequency_mhz, level_dbuv],
          [frequency_mhz, level_dbuv]
        ]
      }
    }
  }
}
```

## Validation Rules

### Required Fields
All standards must have: `name`, `description`, `type`, `subtype`, `measurement_type`, `power_range`, `class`, `limits`

### Type-Specific Rules
- **Radiated standards** must have `distance` field (`3m` or `10m`)
- **Conducted standards** should have `distance: "N/A"` or no distance field
- **Limits** must contain at least one of: `avg`, `qp`, `pk`

### Data Constraints
- Frequencies must be positive numbers in ascending order
- Emission levels should be 0-200 dBμV (warnings for unusual values)
- Standard IDs should use alphanumeric characters and underscores

### Performance Limits  
- JSON file size should be under 500KB
- Parsing should complete in under 100ms for 10 iterations

## Running Tests

### Full Test Suite
```bash
npm test
```

### JSON-Only Validation
```bash
npm run test:json
```

### Integration Tests
```bash
npm test -- tests/integration.test.js
```

## Error Examples

### Common JSON Errors
```bash
❌ JSON PARSE ERROR: Unexpected token '}' at position 1234
❌ Missing field "type"
❌ Invalid type "radiated_conducted" 
❌ Frequencies not in ascending order
⚠️  Unusual level 250 dBμV (too high)
```

### Fixing Errors
1. **JSON Syntax**: Use online JSON validators or editor with JSON linting
2. **Missing Fields**: Refer to schema and add required properties
3. **Invalid Values**: Check enum constraints in schema
4. **Frequency Order**: Sort frequency points in ascending order
5. **Data Ranges**: Verify frequency (>0) and level (0-200) ranges

## Current Statistics

As of latest validation:
- **29 standards** total
- **22 conducted**, **7 radiated** emission standards  
- **6 classes**: A, B, automotive, consumer, non_consumer, special
- **6 subtypes**: power_ports, mains_ports, ac_lines, dc_lines, telecom_lan_ports, free_space
- **File size**: ~15KB
- **Standards coverage**: CISPR 11, CISPR 32, EN 55032, ECE R-10, FCC Part 18

## Integration with Application

The JSON validation ensures:
1. **Application stability** - No runtime JSON parsing errors
2. **Data integrity** - All required fields present with correct types
3. **UI consistency** - Standards display predictably in selectors
4. **Chart rendering** - Frequency/level data is properly formatted
5. **Compliance analysis** - Limit types (avg/qp/pk) are available

## Continuous Integration

Every commit automatically validates:
- JSON syntax and structure
- Data consistency and completeness  
- File size and performance constraints
- Standard count and coverage

Failed validation blocks deployment until issues are resolved.
