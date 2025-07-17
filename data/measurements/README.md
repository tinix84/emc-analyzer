# Measurement Data

This directory contains spectrum analyzer measurement files in TXT format.

## File Formats Supported

### Keysight/Agilent Format
```
# Header comments (ignored)
150000	-45.2
155000	-47.1
160000	-46.8
```

### Rohde & Schwarz Format  
```
Frequency,Amplitude
150000,-45.2
155000,-47.1
160000,-46.8
```

### Generic Space-Separated
```
150000 -45.2
155000 -47.1  
160000 -46.8
```

## File Requirements

- **Frequency**: In Hz (first column)
- **Amplitude**: In dBμV (second column)
- **Separators**: Tab, comma, or space
- **Headers**: Lines starting with `#`, `%`, or containing "freq" are ignored
- **Order**: Frequencies should be in ascending order

## Usage

### In Jupyter Notebooks
```python
measurements = load_spectrum_analyzer_txt("../data/measurements/device_scan.txt")
```

### In Vue/Nuxt Components
```javascript
const response = await fetch('/data/measurements/device_scan.txt')
const text = await response.text()
// Parse the data...
```

## Measurement Types

Organize files by measurement type:
- `device_avg_*.txt` - Average detector measurements
- `device_qp_*.txt` - Quasi-peak detector measurements  
- `device_peak_*.txt` - Peak detector measurements

## Best Practices

1. **Naming**: Use descriptive filenames (e.g., `laptop_conducted_avg_20240717.txt`)
2. **Frequency Range**: Cover the full EMC standard range
3. **Resolution**: Use appropriate frequency steps (typically 1-10 kHz)
4. **Units**: Always use Hz for frequency, dBμV for amplitude
5. **Headers**: Include equipment info in comments

## Equipment Integration

### Export Settings
- **Keysight**: Format → ASCII, Delimiter → Tab
- **R&S**: File → Export → ASCII  
- **Tektronix**: Save → Data → ASCII

### Automation
You can automate data collection using SCPI commands and save directly to this directory.

## Files in This Directory

- `sample_measurements.txt` - Example measurement data with realistic EMC violations
- Add your measurement files here following the supported formats
