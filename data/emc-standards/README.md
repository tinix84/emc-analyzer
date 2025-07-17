# EMC Standards Data

This directory contains EMC standard definitions in CSV format.

## File Format

EMC standard CSV files should have these columns:
- `frequency_hz`: Frequency in Hz  
- `avg_limit_dbuv`: Average limit in dBμV
- `qp_limit_dbuv`: Quasi-peak limit in dBμV
- `peak_limit_dbuv`: Peak limit in dBμV

## Example File Structure

```csv
frequency_hz,avg_limit_dbuv,qp_limit_dbuv,peak_limit_dbuv
150000,79,84,90
500000,73,78,84
30000000,40,46,56
100000000,40,46,56
1000000000,47,53,63
```

## Supported Standards

- **CISPR 22**: Information technology equipment
- **EN 55032**: Multimedia equipment  
- **ECE R10**: Automotive electromagnetic compatibility
- **IEC 61800-3**: Power drive systems
- **CISPR 25**: Automotive radio disturbance

## Usage

### In Jupyter Notebooks
```python
emc_standard = load_emc_standard_csv("../data/emc-standards/cispr22_class_a.csv")
```

### In Vue/Nuxt Components
```javascript
const response = await fetch('/data/emc-standards/cispr22_class_a.csv')
```

## Adding New Standards

1. Create CSV file with required columns
2. Use exponential notation for large frequencies (e.g., `1e6` instead of `1000000`)
3. Ensure frequencies are in ascending order
4. Include discontinuity points where standards change
5. Test with the validation notebook

## Files in This Directory

- `sample_emc_standard.csv` - Example CISPR 22 Class A standard
- Add your own standard files here following the same format
