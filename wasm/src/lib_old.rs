// wasm/src/lib.rs
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ndarray::{Array1, Axis};
use num_traits::Float;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Point structure for frequency-amplitude pairs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaskPoint {
    pub frequency: f64,  // Hz
    pub amplitude: f64,  // dBµV
}

impl MaskPoint {
    pub fn new(frequency: f64, amplitude: f64) -> Self {
        Self { frequency, amplitude }
    }
}

// Standard class definition from JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardClass {
    pub name: String,
    pub description: String,
    pub avg_mask: Vec<(f64, f64)>,  // (frequency_hz, amplitude_dbuv)
    pub qp_mask: Option<Vec<(f64, f64)>>,
    pub pk_mask: Option<Vec<(f64, f64)>>,
}

// Complete standard definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardDefinition {
    #[serde(flatten)]
    pub classes: HashMap<String, StandardClass>,
}

// Root structure for the JSON file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmcStandardsData {
    pub standards: HashMap<String, HashMap<String, StandardClass>>,
}

// The EMC Standard structure used by the API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EMCStandard {
    pub name: String,
    pub description: String,
    pub f_avg_limit_mask: Vec<f64>,
    pub dbuv_avg_limit_mask: Vec<f64>,
    pub f_qp_limit_mask: Option<Vec<f64>>,
    pub dbuv_qp_limit_mask: Option<Vec<f64>>,
    pub f_pk_limit_mask: Option<Vec<f64>>,
    pub dbuv_pk_limit_mask: Option<Vec<f64>>,
}

impl EMCStandard {
    // Convert from StandardClass to EMCStandard
    pub fn from_standard_class(class: &StandardClass) -> Self {
        let (f_avg, dbuv_avg): (Vec<f64>, Vec<f64>) = class.avg_mask.iter().cloned().unzip();
        
        let (f_qp, dbuv_qp) = if let Some(qp_mask) = &class.qp_mask {
            let (f, dbuv): (Vec<f64>, Vec<f64>) = qp_mask.iter().cloned().unzip();
            (Some(f), Some(dbuv))
        } else {
            (None, None)
        };
        
        let (f_pk, dbuv_pk) = if let Some(pk_mask) = &class.pk_mask {
            let (f, dbuv): (Vec<f64>, Vec<f64>) = pk_mask.iter().cloned().unzip();
            (Some(f), Some(dbuv))
        } else {
            (None, None)
        };
        
        Self {
            name: class.name.clone(),
            description: class.description.clone(),
            f_avg_limit_mask: f_avg,
            dbuv_avg_limit_mask: dbuv_avg,
            f_qp_limit_mask: f_qp,
            dbuv_qp_limit_mask: dbuv_qp,
            f_pk_limit_mask: f_pk,
            dbuv_pk_limit_mask: dbuv_pk,
        }
    }
    
    // Load standards from embedded JSON
    fn load_standards_data() -> Result<EmcStandardsData, String> {
        let json_data = include_str!("../emc_standards.json");
        serde_json::from_str(json_data).map_err(|e| format!("Failed to parse standards JSON: {}", e))
    }
    
    // Get a specific standard by name and class
    pub fn get_standard(standard_name: &str, class_name: &str) -> Result<EMCStandard, String> {
        let data = Self::load_standards_data()?;
        
        let standard = data.standards.get(standard_name)
            .ok_or_else(|| format!("Standard '{}' not found", standard_name))?;
            
        let class = standard.get(class_name)
            .ok_or_else(|| format!("Class '{}' not found for standard '{}'", class_name, standard_name))?;
            
        Ok(Self::from_standard_class(class))
    }
    
    // List available standards
    pub fn list_standards() -> Result<Vec<String>, String> {
        let data = Self::load_standards_data()?;
        Ok(data.standards.keys().cloned().collect())
    }
    
    // List available classes for a standard
    pub fn list_classes(standard_name: &str) -> Result<Vec<String>, String> {
        let data = Self::load_standards_data()?;
        let standard = data.standards.get(standard_name)
            .ok_or_else(|| format!("Standard '{}' not found", standard_name))?;
        Ok(standard.keys().cloned().collect())
    }
    
    // Interpolate limit at a specific frequency
    pub fn interp_log(&self, frequency: f64) -> EMCLimitResult {
        let avg_limit = Self::interpolate_log(&self.f_avg_limit_mask, &self.dbuv_avg_limit_mask, frequency);
        
        let qp_limit = if let (Some(f_qp), Some(dbuv_qp)) = (&self.f_qp_limit_mask, &self.dbuv_qp_limit_mask) {
            Self::interpolate_log(f_qp, dbuv_qp, frequency)
        } else {
            avg_limit + 6.0 // Typical QP offset
        };
        
        let pk_limit = if let (Some(f_pk), Some(dbuv_pk)) = (&self.f_pk_limit_mask, &self.dbuv_pk_limit_mask) {
            Self::interpolate_log(f_pk, dbuv_pk, frequency)
        } else {
            qp_limit + 10.0 // Typical PK offset
        };
        
        EMCLimitResult {
            avg_limit,
            qp_limit,
            pk_limit,
            dbuv_avg_limit: avg_limit,
            dbuv_qp_limit: qp_limit,
            dbuv_pk_limit: pk_limit,
        }
    }
    
    // Improved logarithmic interpolation using ndarray
    fn interpolate_log(frequencies: &[f64], amplitudes: &[f64], target_freq: f64) -> f64 {
        if frequencies.is_empty() || amplitudes.is_empty() || frequencies.len() != amplitudes.len() {
            return 0.0;
        }
        
        let freq_array = Array1::from(frequencies.to_vec());
        let amp_array = Array1::from(amplitudes.to_vec());
        
        // Convert to log scale for interpolation
        let log_freq_array = freq_array.mapv(|f| f.ln());
        let log_target = target_freq.ln();
        
        // Boundary conditions
        if log_target <= log_freq_array[0] {
            return amp_array[0];
        }
        if log_target >= log_freq_array[log_freq_array.len() - 1] {
            return amp_array[amp_array.len() - 1];
        }
        
        // Find interpolation interval using binary search-like approach
        for i in 0..log_freq_array.len() - 1 {
            if log_target >= log_freq_array[i] && log_target <= log_freq_array[i + 1] {
                let ratio = (log_target - log_freq_array[i]) / (log_freq_array[i + 1] - log_freq_array[i]);
                return amp_array[i] + ratio * (amp_array[i + 1] - amp_array[i]);
            }
        }
        
        amp_array[0]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EMCLimitResult {
    pub avg_limit: f64,
    pub qp_limit: f64,
    pub pk_limit: f64,
    pub dbuv_avg_limit: f64,
    pub dbuv_qp_limit: f64,
    pub dbuv_pk_limit: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceResult {
    pub frequency: f64,
    pub amplitude: f64,
    pub limit: f64,
    pub passes: bool,
    pub margin: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EMCMask {
    pub avg: Vec<MaskPoint>,
    pub qp: Vec<MaskPoint>,
    pub pk: Vec<MaskPoint>,
}

impl EMCStandard {
    fn new(name: &str) -> Self {
        EMCStandard {
            name: name.to_string(),
            description: String::new(),
            f_avg_limit_mask: vec![],
            dbuv_avg_limit_mask: vec![],
            f_qp_limit_mask: None,
            dbuv_qp_limit_mask: None,
            f_pk_limit_mask: None,
            dbuv_pk_limit_mask: None,
        }
    }



    // Standard implementations
    fn cispr22(emc_class: &str) -> Result<Self, String> {
        let mut standard = EMCStandard::new(&format!("CISPR22_Class_{}", emc_class.to_uppercase()));
        
        match emc_class.to_lowercase().as_str() {
            "a" => {
                standard.f_avg_limit_mask = vec![0.15e6, 0.5e6, 0.5e6 + 1.0, 30e6];
                standard.dbuv_avg_limit_mask = vec![66.0, 66.0, 60.0, 60.0];
                standard.f_qp_limit_mask = Some(standard.f_avg_limit_mask.clone());
                standard.dbuv_qp_limit_mask = Some(vec![79.0, 79.0, 73.0, 73.0]);
            }
            "b" => {
                standard.f_avg_limit_mask = vec![0.15e6, 0.5e6, 0.5e6 + 1.0, 5e6, 5e6 + 1.0, 30e6];
                standard.dbuv_avg_limit_mask = vec![56.0, 46.0, 46.0, 46.0, 50.0, 50.0];
                standard.f_qp_limit_mask = Some(standard.f_avg_limit_mask.clone());
                standard.dbuv_qp_limit_mask = Some(vec![66.0, 56.0, 56.0, 56.0, 60.0, 60.0]);
            }
            _ => return Err("emc_class must be 'A' or 'B'".to_string()),
        }
        
        Ok(standard)
    }

    fn en55032(emc_class: &str) -> Result<Self, String> {
        let mut standard = Self::cispr22(emc_class)?;
        standard.name = format!("EN55032_Class_{}", emc_class.to_uppercase());
        Ok(standard)
    }

    fn ece_r10_conducted_ac_lines() -> Self {
        let mut standard = EMCStandard::new("ECE_R_10_2012_AC");
        standard.f_avg_limit_mask = vec![0.15e6, 0.5e6, 0.5e6 + 1.0, 5e6, 5e6 + 1.0, 30e6];
        standard.dbuv_avg_limit_mask = vec![56.0, 46.0, 46.0, 46.0, 50.0, 50.0];
        standard.f_qp_limit_mask = Some(standard.f_avg_limit_mask.clone());
        standard.dbuv_qp_limit_mask = Some(vec![66.0, 56.0, 56.0, 56.0, 60.0, 60.0]);
        standard
    }

    fn ece_r10_conducted_dc_lines() -> Self {
        let mut standard = EMCStandard::new("ECE_R_10_2012_DC");
        standard.f_avg_limit_mask = vec![0.15e6, 0.5e6, 0.5e6 + 1.0, 30e6];
        standard.dbuv_avg_limit_mask = vec![66.0, 66.0, 60.0, 60.0];
        standard.f_qp_limit_mask = Some(standard.f_avg_limit_mask.clone());
        standard.dbuv_qp_limit_mask = Some(vec![79.0, 79.0, 66.0, 66.0]);
        standard
    }

    fn iec61800_3(emc_class: &str, interface: &str) -> Result<Self, String> {
        let mut standard = EMCStandard::new(&format!("IEC61800_3_Class_{}", emc_class.to_uppercase()));

        match (interface.to_lowercase().as_str(), emc_class.to_lowercase().as_str()) {
            ("ac", "c1") => {
                standard.f_avg_limit_mask = vec![0.15e6, 0.5e6, 0.5e6 + 1.0, 30e6];
                standard.dbuv_avg_limit_mask = vec![66.0, 66.0, 60.0, 60.0];
                standard.f_qp_limit_mask = Some(standard.f_avg_limit_mask.clone());
                standard.dbuv_qp_limit_mask = Some(vec![79.0, 79.0, 73.0, 73.0]);
            }
            ("ac", "c2") => {
                standard.f_avg_limit_mask = vec![0.15e6, 0.5e6, 0.5e6 + 1.0, 5e6, 5e6 + 1.0, 30e6];
                standard.dbuv_avg_limit_mask = vec![56.0, 46.0, 46.0, 46.0, 50.0, 50.0];
                standard.f_qp_limit_mask = Some(standard.f_avg_limit_mask.clone());
                standard.dbuv_qp_limit_mask = Some(vec![66.0, 56.0, 56.0, 56.0, 60.0, 60.0]);
            }
            ("dc", "c1") => {
                standard.f_avg_limit_mask = vec![0.15e6, 0.5e6, 0.5e6 + 1.0, 30e6];
                standard.dbuv_avg_limit_mask = vec![80.0, 80.0, 74.0, 74.0];
                standard.f_qp_limit_mask = Some(standard.f_avg_limit_mask.clone());
                standard.dbuv_qp_limit_mask = Some(vec![80.0, 80.0, 74.0, 74.0]);
            }
            _ => return Err("Invalid interface/class combination".to_string()),
        }

        Ok(standard)
    }

    // Advanced EMC analysis using statistical methods
    pub fn analyze_measurement_statistics(&self, frequencies: &[f64], amplitudes: &[f64]) -> EMCStatistics {
        let freq_array = Array1::from(frequencies.to_vec());
        let amp_array = Array1::from(amplitudes.to_vec());
        
        // Calculate statistical measures
        let mean_amplitude = amp_array.mean().unwrap_or(0.0);
        let std_amplitude = amp_array.std(0.0);
        let max_amplitude = amp_array.fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let min_amplitude = amp_array.fold(f64::INFINITY, |a, &b| a.min(b));
        
        // Calculate percentiles
        let mut sorted_amps = amplitudes.to_vec();
        sorted_amps.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let p95 = Self::calculate_percentile(&sorted_amps, 95.0);
        let p99 = Self::calculate_percentile(&sorted_amps, 99.0);
        
        // Find worst-case violations
        let mut violations = Vec::new();
        let mut max_violation = 0.0;
        let mut max_violation_freq = 0.0;
        
        for (&freq, &amp) in frequencies.iter().zip(amplitudes.iter()) {
            let limit = self.interp_log(freq);
            let margin = limit.dbuv_avg_limit - amp;
            
            if margin < 0.0 {
                violations.push(ComplianceResult {
                    frequency: freq,
                    amplitude: amp,
                    limit: limit.dbuv_avg_limit,
                    passes: false,
                    margin,
                });
                
                if margin.abs() > max_violation {
                    max_violation = margin.abs();
                    max_violation_freq = freq;
                }
            }
        }
        
        EMCStatistics {
            mean_amplitude,
            std_amplitude,
            max_amplitude,
            min_amplitude,
            percentile_95: p95,
            percentile_99: p99,
            violation_count: violations.len(),
            max_violation,
            max_violation_frequency: max_violation_freq,
            compliance_rate: ((frequencies.len() - violations.len()) as f64 / frequencies.len() as f64) * 100.0,
        }
    }
    
    // Improved mask generation with adaptive sampling
    pub fn generate_adaptive_mask(&self, f_min: f64, f_max: f64, target_points: usize) -> EMCMask {
        let log_min = f_min.log10();
        let log_max = f_max.log10();
        
        // Create logarithmically spaced frequency points
        let log_freqs = Array1::linspace(log_min, log_max, target_points);
        let frequencies: Vec<f64> = log_freqs.mapv(|log_f| 10.0_f64.powf(log_f)).to_vec();
        
        let mut avg_points = Vec::new();
        let mut qp_points = Vec::new();
        let mut pk_points = Vec::new();
        
        for freq in frequencies {
            let limit = self.interp_log(freq);
            
            avg_points.push(MaskPoint { 
                frequency: freq, 
                amplitude: limit.dbuv_avg_limit 
            });
            
            if limit.dbuv_qp_limit > 0.0 {
                qp_points.push(MaskPoint { 
                    frequency: freq, 
                    amplitude: limit.dbuv_qp_limit 
                });
            }
            
            if limit.dbuv_pk_limit > 0.0 {
                pk_points.push(MaskPoint { 
                    frequency: freq, 
                    amplitude: limit.dbuv_pk_limit 
                });
            }
        }
        
        EMCMask {
            avg: avg_points,
            qp: qp_points,
            pk: pk_points,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EMCStatistics {
    pub mean_amplitude: f64,
    pub std_amplitude: f64,
    pub max_amplitude: f64,
    pub min_amplitude: f64,
    pub percentile_95: f64,
    pub percentile_99: f64,
    pub violation_count: usize,
    pub max_violation: f64,
    pub max_violation_frequency: f64,
    pub compliance_rate: f64,
}

// Manual percentile calculation for WASM compatibility
fn calculate_percentile(sorted_data: &[f64], percentile: f64) -> f64 {
    if sorted_data.is_empty() {
        return 0.0;
    }
    
    let n = sorted_data.len();
    let index = (percentile / 100.0) * (n - 1) as f64;
    let lower = index.floor() as usize;
    let upper = index.ceil() as usize;
    
    if lower == upper || upper >= n {
        return sorted_data[lower.min(n - 1)];
    }
    
    let weight = index - lower as f64;
    sorted_data[lower] * (1.0 - weight) + sorted_data[upper] * weight
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EMCMask {
    pub avg: Vec<MaskPoint>,
    pub qp: Vec<MaskPoint>,
    pub pk: Vec<MaskPoint>,
}

impl EMCStandard {
    fn new(name: &str) -> Self {
        EMCStandard {
            name: name.to_string(),
            description: String::new(),
            f_avg_limit_mask: vec![],
            dbuv_avg_limit_mask: vec![],
            f_qp_limit_mask: None,
            dbuv_qp_limit_mask: None,
            f_pk_limit_mask: None,
            dbuv_pk_limit_mask: None,
        }
    }



    // Standard implementations
    fn cispr22(emc_class: &str) -> Result<Self, String> {
        let mut standard = EMCStandard::new(&format!("CISPR22_Class_{}", emc_class.to_uppercase()));
        
        match emc_class.to_lowercase().as_str() {
            "a" => {
                standard.f_avg_limit_mask = vec![0.15e6, 0.5e6, 0.5e6 + 1.0, 30e6];
                standard.dbuv_avg_limit_mask = vec![66.0, 66.0, 60.0, 60.0];
                standard.f_qp_limit_mask = Some(standard.f_avg_limit_mask.clone());
                standard.dbuv_qp_limit_mask = Some(vec![79.0, 79.0, 73.0, 73.0]);
            }
            "b" => {
                standard.f_avg_limit_mask = vec![0.15e6, 0.5e6, 0.5e6 + 1.0, 5e6, 5e6 + 1.0, 30e6];
                standard.dbuv_avg_limit_mask = vec![56.0, 46.0, 46.0, 46.0, 50.0, 50.0];
                standard.f_qp_limit_mask = Some(standard.f_avg_limit_mask.clone());
                standard.dbuv_qp_limit_mask = Some(vec![66.0, 56.0, 56.0, 56.0, 60.0, 60.0]);
            }
            _ => return Err("emc_class must be 'A' or 'B'".to_string()),
        }
        
        Ok(standard)
    }

    fn en55032(emc_class: &str) -> Result<Self, String> {
        let mut standard = Self::cispr22(emc_class)?;
        standard.name = format!("EN55032_Class_{}", emc_class.to_uppercase());
        Ok(standard)
    }

    fn ece_r10_conducted_ac_lines() -> Self {
        let mut standard = EMCStandard::new("ECE_R_10_2012_AC");
        standard.f_avg_limit_mask = vec![0.15e6, 0.5e6, 0.5e6 + 1.0, 5e6, 5e6 + 1.0, 30e6];
        standard.dbuv_avg_limit_mask = vec![56.0, 46.0, 46.0, 46.0, 50.0, 50.0];
        standard.f_qp_limit_mask = Some(standard.f_avg_limit_mask.clone());
        standard.dbuv_qp_limit_mask = Some(vec![66.0, 56.0, 56.0, 56.0, 60.0, 60.0]);
        standard
    }

    fn ece_r10_conducted_dc_lines() -> Self {
        let mut standard = EMCStandard::new("ECE_R_10_2012_DC");
        standard.f_avg_limit_mask = vec![0.15e6, 0.5e6, 0.5e6 + 1.0, 30e6];
        standard.dbuv_avg_limit_mask = vec![66.0, 66.0, 60.0, 60.0];
        standard.f_qp_limit_mask = Some(standard.f_avg_limit_mask.clone());
        standard.dbuv_qp_limit_mask = Some(vec![79.0, 79.0, 66.0, 66.0]);
        standard
    }

    fn iec61800_3(emc_class: &str, interface: &str) -> Result<Self, String> {
        let mut standard = EMCStandard::new(&format!("IEC61800_3_Class_{}", emc_class.to_uppercase()));

        match (interface.to_lowercase().as_str(), emc_class.to_lowercase().as_str()) {
            ("ac", "c1") => {
                standard.f_avg_limit_mask = vec![0.15e6, 0.5e6, 0.5e6 + 1.0, 30e6];
                standard.dbuv_avg_limit_mask = vec![66.0, 66.0, 60.0, 60.0];
                standard.f_qp_limit_mask = Some(standard.f_avg_limit_mask.clone());
                standard.dbuv_qp_limit_mask = Some(vec![79.0, 79.0, 73.0, 73.0]);
            }
            ("ac", "c2") => {
                standard.f_avg_limit_mask = vec![0.15e6, 0.5e6, 0.5e6 + 1.0, 5e6, 5e6 + 1.0, 30e6];
                standard.dbuv_avg_limit_mask = vec![56.0, 46.0, 46.0, 46.0, 50.0, 50.0];
                standard.f_qp_limit_mask = Some(standard.f_avg_limit_mask.clone());
                standard.dbuv_qp_limit_mask = Some(vec![66.0, 56.0, 56.0, 56.0, 60.0, 60.0]);
            }
            ("dc", "c1") => {
                standard.f_avg_limit_mask = vec![0.15e6, 0.5e6, 0.5e6 + 1.0, 30e6];
                standard.dbuv_avg_limit_mask = vec![80.0, 80.0, 74.0, 74.0];
                standard.f_qp_limit_mask = Some(standard.f_avg_limit_mask.clone());
                standard.dbuv_qp_limit_mask = Some(vec![80.0, 80.0, 74.0, 74.0]);
            }
            _ => return Err("Invalid interface/class combination".to_string()),
        }

        Ok(standard)
    }
}

// Improved logarithmic interpolation using ndarray (legacy function for compatibility)
fn log_interp(x_points: &[f64], y_points: &[f64], x: f64) -> f64 {
    EMCStandard::interpolate_log(x_points, y_points, x)
}

// WASM bindings
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn get_emc_standard(standard_name: &str, emc_class: &str, _interface: Option<String>) -> Result<JsValue, JsValue> {
    console_log!("Getting EMC standard: {} {}", standard_name, emc_class);
    
    match EMCStandard::get_standard(standard_name, emc_class) {
        Ok(standard) => {
            console_log!("Standard loaded successfully: {}", standard.name);
            serde_wasm_bindgen::to_value(&standard).map_err(|e| JsValue::from_str(&e.to_string()))
        },
        Err(e) => {
            console_log!("Error loading standard: {}", e);
            Err(JsValue::from_str(&e))
        }
    }
}

#[wasm_bindgen]
pub fn list_available_standards() -> Result<JsValue, JsValue> {
    match EMCStandard::list_standards() {
        Ok(standards) => serde_wasm_bindgen::to_value(&standards).map_err(|e| JsValue::from_str(&e.to_string())),
        Err(e) => Err(JsValue::from_str(&e))
    }
}

#[wasm_bindgen]
pub fn list_standard_classes(standard_name: &str) -> Result<JsValue, JsValue> {
    match EMCStandard::list_classes(standard_name) {
        Ok(classes) => serde_wasm_bindgen::to_value(&classes).map_err(|e| JsValue::from_str(&e.to_string())),
        Err(e) => Err(JsValue::from_str(&e))
    }
}

#[wasm_bindgen]
pub fn calculate_emc_limit(standard_json: &str, frequency: f64) -> Result<JsValue, JsValue> {
    let standard: EMCStandard = serde_json::from_str(standard_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let result = standard.interp_log(frequency);
    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn check_emc_compliance(
    standard_json: &str,
    frequencies: &[f64],
    amplitudes: &[f64],
    measurement_type: &str
) -> Result<JsValue, JsValue> {
    let standard: EMCStandard = serde_json::from_str(standard_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let mut results = Vec::new();
    
    for (&freq, &amp) in frequencies.iter().zip(amplitudes.iter()) {
        let limit = standard.interp_log(freq);
        
        let limit_value = match measurement_type.to_lowercase().as_str() {
            "avg" => limit.dbuv_avg_limit,
            "qp" => limit.dbuv_qp_limit,
            "pk" => limit.dbuv_pk_limit,
            _ => return Err(JsValue::from_str("measurement_type must be 'avg', 'qp', or 'pk'")),
        };
        
        results.push(ComplianceResult {
            frequency: freq,
            amplitude: amp,
            limit: limit_value,
            passes: amp <= limit_value,
            margin: limit_value - amp,
        });
    }

    serde_wasm_bindgen::to_value(&results).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn generate_emc_mask(
    standard_json: &str,
    f_min: f64,
    f_max: f64,
    points_per_decade: usize
) -> Result<JsValue, JsValue> {
    let standard: EMCStandard = serde_json::from_str(standard_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let log_min = f_min.log10();
    let log_max = f_max.log10();
    let total_points = ((log_max - log_min) * points_per_decade as f64).ceil() as usize;
    
    let mut avg_points = Vec::new();
    let mut qp_points = Vec::new();
    let mut pk_points = Vec::new();
    
    for i in 0..=total_points {
        let log_f = log_min + (i as f64 / total_points as f64) * (log_max - log_min);
        let freq = 10.0_f64.powf(log_f);
        
        let limit = standard.interp_log(freq);
        
        avg_points.push(MaskPoint { frequency: freq, amplitude: limit.dbuv_avg_limit });
        
        if limit.dbuv_qp_limit > 0.0 {
            qp_points.push(MaskPoint { frequency: freq, amplitude: limit.dbuv_qp_limit });
        }
        
        if limit.dbuv_pk_limit > 0.0 {
            pk_points.push(MaskPoint { frequency: freq, amplitude: limit.dbuv_pk_limit });
        }
    }
    
    let mask = EMCMask {
        avg: avg_points,
        qp: qp_points,
        pk: pk_points,
    };
    
    serde_wasm_bindgen::to_value(&mask).map_err(|e| JsValue::from_str(&e.to_string()))
}

// Enhanced WASM bindings for statistical analysis
#[wasm_bindgen]
pub fn analyze_emc_statistics(
    standard_json: &str,
    frequencies: &[f64],
    amplitudes: &[f64]
) -> Result<JsValue, JsValue> {
    let standard: EMCStandard = serde_json::from_str(standard_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let stats = standard.analyze_measurement_statistics(frequencies, amplitudes);
    serde_wasm_bindgen::to_value(&stats).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn generate_adaptive_emc_mask(
    standard_json: &str,
    f_min: f64,
    f_max: f64,
    target_points: usize
) -> Result<JsValue, JsValue> {
    let standard: EMCStandard = serde_json::from_str(standard_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let mask = standard.generate_adaptive_mask(f_min, f_max, target_points);
    serde_wasm_bindgen::to_value(&mask).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(test)]
pub mod test_standalone {
    use super::*;

    #[test]
    fn test_log_interp() {
        let x_points = vec![1.0, 10.0, 100.0, 1000.0];
        let y_points = vec![0.0, 1.0, 2.0, 3.0];
        let x = 50.0;
        let expected = 1.69897; // log10(50) ≈ 1.69897
        let result = log_interp(&x_points, &y_points, x);
        assert!((result - expected).abs() < 1e-5);
    }

    #[test]
    fn test_interp_log() {
        let standard = EMCStandard {
            name: "Test_Standard".to_string(),
            f_avg_limit_mask: vec![0.15e6, 0.5e6, 30e6],
            dbuv_avg_limit_mask: vec![66.0, 60.0, 60.0],
            f_qp_limit_mask: None,
            dbuv_qp_limit_mask: None,
            f_pk_limit_mask: None,
            dbuv_pk_limit_mask: None,
        };

        let frequency = 0.3e6;
        let result = standard.interp_log(frequency);
        
        assert!((result.avg_limit - 10.0).abs() < 1e-5);
        assert!((result.qp_limit - 10.0).abs() < 1e-5);
        assert!((result.pk_limit - 10.0).abs() < 1e-5);
    }

    #[test]
    fn test_cispr22() {
        let standard = EMCStandard::cispr22("A").unwrap();
        
        assert_eq!(standard.name, "CISPR22_Class_A");
        assert_eq!(standard.f_avg_limit_mask.len(), 4);
        assert_eq!(standard.dbuv_avg_limit_mask.len(), 4);
    }

    #[test]
    fn test_en55032() {
        let standard = EMCStandard::en55032("B").unwrap();
        
        assert_eq!(standard.name, "EN55032_Class_B");
        assert_eq!(standard.f_avg_limit_mask.len(), 6);
        assert_eq!(standard.dbuv_avg_limit_mask.len(), 6);
    }

    #[test]
    fn test_ece_r10_conducted_ac_lines() {
        let standard = EMCStandard::ece_r10_conducted_ac_lines();
        
        assert_eq!(standard.name, "ECE_R_10_2012_AC");
        assert_eq!(standard.f_avg_limit_mask.len(), 6);
        assert_eq!(standard.dbuv_avg_limit_mask.len(), 6);
    }

    #[test]
    fn test_ece_r10_conducted_dc_lines() {
        let standard = EMCStandard::ece_r10_conducted_dc_lines();
        
        assert_eq!(standard.name, "ECE_R_10_2012_DC");
        assert_eq!(standard.f_avg_limit_mask.len(), 4);
        assert_eq!(standard.dbuv_avg_limit_mask.len(), 4);
    }

    #[test]
    fn test_iec61800_3() {
        let standard = EMCStandard::iec61800_3("C1", "AC").unwrap();
        
        assert_eq!(standard.name, "IEC61800_3_Class_C1");
        assert_eq!(standard.f_avg_limit_mask.len(), 4);
        assert_eq!(standard.dbuv_avg_limit_mask.len(), 4);
    }
}
