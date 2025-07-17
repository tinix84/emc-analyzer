# EMC Validation Test Data

This directory contains sample data files for testing the EMC validation notebook and web components:

## Files Structure

### `../data/emc-standards/sample_emc_standard.csv`
- **Purpose**: EMC standard limits (CISPR 22 Class A conducted emissions)
- **Format**: CSV with columns: `frequency_hz`, `avg_limit_dbuv`, `qp_limit_dbuv`, `peak_limit_dbuv`
- **Frequency Range**: 150 kHz to 1 GHz
- **Usage**: Replace with your actual EMC standard data

### `../data/measurements/sample_measurements.txt`
- **Purpose**: Spectrum analyzer measurement data
- **Format**: Tab-separated frequency (Hz) and amplitude (dBμV)
- **Equipment**: Simulated Keysight E4440A data
- **Usage**: Replace with your actual measurement files

## Usage in Notebook

```python
# Load your actual data files (from notebooks directory)
emc_standard = load_emc_standard_csv("../data/emc-standards/sample_emc_standard.csv")
measurements = load_spectrum_analyzer_txt("../data/measurements/sample_measurements.txt")
```

## Usage in Vue Components

```javascript
// In Vue/Nuxt components
const standardPath = '/data/emc-standards/sample_emc_standard.csv'
const measurementPath = '/data/measurements/sample_measurements.txt'
```

## Data Characteristics

The sample data is designed to show:
- ✅ **Compliance** at most frequencies
- ❌ **Violations** at critical frequencies (around 1.5 MHz)
- 📊 **Realistic EMC behavior** with frequency-dependent noise floor
- 🎯 **Edge cases** near EMC standard discontinuities

This allows you to test all features of the EMC validation workflow!

## File Organization

The new organized structure:
```
data/
├── emc-standards/        # EMC standard CSV files
├── measurements/         # Spectrum analyzer TXT files  
└── emc_analysis_results/ # Generated analysis reports
```

This structure makes it easy to:
- 📁 **Organize** different types of data
- 🔍 **Find** files quickly
- 🌐 **Deploy** to websites
- 📊 **Scale** with more standards and measurements
