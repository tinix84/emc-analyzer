# EMC Analysis Project Structure

This project provides a comprehensive EMC (Electromagnetic Compatibility) analysis toolkit using Rust WASM, Vue.js, and Nuxt.js.

## 📁 Project Structure

```
nuxtjs-boilerplate/
├── 📂 wasm/                    # Rust WASM EMC analysis engine
│   ├── src/                    # Rust source code
│   │   ├── lib.rs             # Main EMC analysis functions
│   │   └── frequency_helpers.rs # Frequency constants and utilities
│   ├── pkg/                   # Generated WASM package
│   ├── Cargo.toml            # Rust dependencies
│   └── emc_standards.json    # EMC standards database
│
├── 📂 data/                   # Data files and analysis results
│   ├── emc-standards/        # EMC standard definitions (CSV)
│   ├── measurements/         # Spectrum analyzer measurements (TXT)
│   └── emc_analysis_results/ # Generated analysis reports
│
├── 📂 notebooks/             # Jupyter notebooks for validation
│   └── emc_validation.ipynb # EMC compliance validation notebook
│
├── 📂 demos/                 # HTML demonstration files
│   ├── vue-emc-prototype.html     # Vue EMC analysis prototype
│   ├── vue-emc-wasm-demo.html     # Vue + WASM integration demo
│   ├── pure-vue-wasm-test.html    # Pure Vue WASM test
│   ├── simple-wasm-test.html      # Simple WASM functionality test
│   └── test.html                  # Basic test file
│
├── 📂 docs/                  # Documentation
│   └── EMC_TEST_DATA_README.md   # Test data documentation
│
├── 📂 pages/                 # Nuxt.js pages
├── 📂 components/            # Vue components
├── 📂 composables/           # Vue composables
├── 📂 stores/                # Pinia stores
├── 📂 assets/                # Static assets
├── 📂 public/                # Public files
└── 📂 types/                 # TypeScript type definitions
```

## 🚀 Getting Started

### 1. Build the Rust WASM Module
```bash
cd wasm
wasm-pack build --target web
```

### 2. Install Node.js Dependencies
```bash
npm install
```

### 3. Run the Development Server
```bash
npm run dev
```

### 4. Try the Demos
Open any file from the `demos/` directory in your browser.

### 5. Use the Jupyter Notebook
```bash
cd notebooks
jupyter notebook emc_validation.ipynb
```

## 📊 Data Organization

### EMC Standards (`data/emc-standards/`)
- **Format**: CSV files with columns: `frequency_hz`, `avg_limit_dbuv`, `qp_limit_dbuv`, `peak_limit_dbuv`
- **Standards Supported**: CISPR 22, EN 55032, ECE R10, IEC 61800-3
- **Example**: `sample_emc_standard.csv`

### Measurements (`data/measurements/`)
- **Format**: TXT files with frequency (Hz) and amplitude (dBμV)
- **Equipment Supported**: Keysight, Rohde & Schwarz, Tektronix, Generic
- **Example**: `sample_measurements.txt`

### Analysis Results (`data/emc_analysis_results/`)
- **Generated Files**:
  - `emc_detailed_analysis_*.csv` - Complete measurement vs limits
  - `emc_violations_*.csv` - Only violations with severity
  - `emc_test_summary_*.csv` - High-level results
  - `emc_compliance_report_*.txt` - Human-readable summary

## 🦀 Rust WASM Functions

The WASM module provides these key functions:
- `interpolate_at_frequency()` - Interpolate EMC limits
- `analyze_emc_statistics()` - Statistical analysis
- `generate_adaptive_emc_mask()` - Adaptive mask generation
- `check_compliance()` - Pass/fail analysis

## 🎯 Usage Examples

### Load Your Data
```python
# In Jupyter notebook
emc_standard = load_emc_standard_csv("../data/emc-standards/cispr22_class_a.csv")
measurements = load_spectrum_analyzer_txt("../data/measurements/my_device.txt")
```

### Vue.js Integration
```javascript
// In Vue components
import { useWasm } from '@/composables/useWasm'
const { checkCompliance } = useWasm()
```

## 🔧 Development

### File Paths in Code
- **Notebook**: Use relative paths `../data/...`
- **Vue Components**: Use `@/data/...` or `/data/...`
- **WASM**: Built files go to `wasm/pkg/`

### Adding New Standards
1. Add CSV file to `data/emc-standards/`
2. Update `wasm/emc_standards.json` if needed
3. Test with `notebooks/emc_validation.ipynb`

### Adding New Demos
1. Create HTML file in `demos/`
2. Reference WASM with `../wasm/pkg/`
3. Load data from `../data/`

## 📚 Documentation

- **API Documentation**: Check `wasm/src/lib.rs` for Rust functions
- **Test Data**: See `docs/EMC_TEST_DATA_README.md`
- **Frequency Expressions**: See `wasm/FREQUENCY_EXPRESSIONS.md`

## 🎉 Benefits of This Structure

- ✅ **Clean Separation**: Data, code, demos, and docs are separated
- ✅ **Website Ready**: Organized for web deployment
- ✅ **WASM Compliant**: Proper WASM module organization
- ✅ **Scalable**: Easy to add new standards and measurements
- ✅ **Professional**: Industry-standard directory structure
- ✅ **Version Control**: Clear organization for Git

This structure follows modern web development best practices and makes the project ready for production deployment! 🚀
