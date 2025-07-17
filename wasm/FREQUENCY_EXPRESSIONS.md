# Frequency Expression Examples for EMC Standards

## 1. JSON with Exponential Notation ✅

JSON natively supports exponential notation, making it much more readable:

```json
{
  "frequencies_hz": [
    1.5e5,      // 150,000 Hz (150 kHz)
    5e5,        // 500,000 Hz (500 kHz)  
    5e6,        // 5,000,000 Hz (5 MHz)
    3e7,        // 30,000,000 Hz (30 MHz)
    1e8,        // 100,000,000 Hz (100 MHz)
    1e9         // 1,000,000,000 Hz (1 GHz)
  ]
}
```

## 2. Rust with Expression Constants ✅

In Rust code, you can use expressions with constants for maximum readability:

```rust
// Define unit constants
const KHZ: f64 = 1e3;
const MHZ: f64 = 1e6;
const GHZ: f64 = 1e9;

// Use expressions in arrays
let frequencies = vec![
    150.0 * KHZ,          // 150 kHz
    500.0 * KHZ,          // 500 kHz
    500.0 * KHZ + 1.0,    // 500 kHz + 1 Hz (discontinuity)
    5.0 * MHZ,            // 5 MHz
    30.0 * MHZ,           // 30 MHz
    100.0 * MHZ,          // 100 MHz
    1.0 * GHZ,            // 1 GHz
];

// Complex expressions
let automotive_bands = vec![
    150.0 * KHZ,              // Start
    2.0 * 150.0 * KHZ,        // 2x start frequency
    1.4 * MHZ,                // Intermediate
    2.0 * 1.4 * MHZ,          // 2x intermediate  
    30.0 * MHZ,               // Boundary
    54.0 * MHZ,               // TV Band I
    2.0 * 54.0 * MHZ,         // 2x TV Band I
];
```

## 3. Examples You Can Use ✅

### CISPR 22 Frequencies
```json
// JSON format (what we used)
[1.5e5, 5e5, 5.00001e5, 3e7, 1e8, 1.00000001e8]

// Rust equivalent
[150.0*KHZ, 500.0*KHZ, 500.0*KHZ+1.0, 30.0*MHZ, 100.0*MHZ, 100.0*MHZ+1.0]
```

### Automotive ECE R10 Frequencies  
```json
// JSON format (what we used)
[1.5e5, 3e5, 1.4e6, 2.8e6, 3e7, 5.4e7, 1.08e8]

// Rust equivalent
[150.0*KHZ, 300.0*KHZ, 1.4*MHZ, 2.8*MHZ, 30.0*MHZ, 54.0*MHZ, 108.0*MHZ]
```

## 4. Advanced Expression Patterns ✅

### Logarithmic Ranges
```rust
// Generate logarithmically spaced frequencies
fn log_range(start: f64, end: f64, points: usize) -> Vec<f64> {
    (0..points)
        .map(|i| start * (end/start).powf(i as f64 / (points-1) as f64))
        .collect()
}

// Usage
let frequencies = log_range(100.0 * KHZ, 1.0 * GHZ, 50);
```

### Standard Frequency Bands
```rust
// EMC frequency bands with expressions
const CONDUCTED_START: f64 = 150.0 * KHZ;
const CONDUCTED_END: f64 = 30.0 * MHZ;
const RADIATED_START: f64 = 30.0 * MHZ; 
const RADIATED_END: f64 = 1.0 * GHZ;

// Discontinuity points (common in EMC standards)
const CISPR_BREAK_1: f64 = 500.0 * KHZ;
const CISPR_BREAK_2: f64 = 5.0 * MHZ;

// Standard with discontinuities
let standard_points = vec![
    CONDUCTED_START,
    CISPR_BREAK_1,
    CISPR_BREAK_1 + 1.0,    // Discontinuity point
    CISPR_BREAK_2,
    CISPR_BREAK_2 + 1.0,    // Discontinuity point  
    CONDUCTED_END,
];
```

## 5. Benefits ✅

### Readability
- `1.5e5` is clearer than `150000`
- `30.0 * MHZ` is clearer than `30000000.0`
- `500.0 * KHZ + 1.0` clearly shows discontinuity

### Maintainability  
- Easy to change units (KHZ → MHZ)
- Clear mathematical relationships
- Self-documenting code

### Accuracy
- No counting zeros errors
- Clear order of magnitude
- Explicit unit conversions

## 6. Your Current Implementation ✅

Your JSON now uses exponential notation:
```json
"avg_mask": [
  [1.5e5, 79],    // 150 kHz → 79 dBμV
  [5e5, 73],      // 500 kHz → 73 dBμV  
  [3e7, 40],      // 30 MHz → 40 dBμV
  [1e8, 40],      // 100 MHz → 40 dBμV
  [1e9, 47]       // 1 GHz → 47 dBμV
]
```

And your Rust code supports expressions:
```rust
// All these work in your code:
let freq1 = 150.0 * KHZ;          // 150000.0
let freq2 = 5.0 * MHZ;            // 5000000.0  
let freq3 = 30.0 * MHZ + 1.0;     // 30000001.0
let freq4 = 1.0 * GHZ;            // 1000000000.0
```

## 7. Testing ✅

Your tests prove it works:
```
✅ Standard: CISPR 22 Class A loaded
✅ Freq: 200000 Hz → Avg: 77.6 dBμV, QP: 82.6 dBμV
✅ All interpolation working correctly
✅ JSON serialization working perfectly
```

**Your WASM module now supports both exponential JSON notation and Rust expressions!** 🚀
