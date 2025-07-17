// wasm/src/lib.rs
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    
    // Improved logarithmic interpolation 
    fn interpolate_log(frequencies: &[f64], amplitudes: &[f64], target_freq: f64) -> f64 {
        if frequencies.is_empty() || amplitudes.is_empty() || frequencies.len() != amplitudes.len() {
            return 0.0;
        }
        
        // Boundary conditions
        if target_freq <= frequencies[0] {
            return amplitudes[0];
        }
        if target_freq >= frequencies[frequencies.len() - 1] {
            return amplitudes[amplitudes.len() - 1];
        }
        
        // Find interpolation interval
        for i in 0..frequencies.len() - 1 {
            if target_freq >= frequencies[i] && target_freq <= frequencies[i + 1] {
                let log_f1 = frequencies[i].ln();
                let log_f2 = frequencies[i + 1].ln();
                let log_target = target_freq.ln();
                
                let ratio = (log_target - log_f1) / (log_f2 - log_f1);
                return amplitudes[i] + ratio * (amplitudes[i + 1] - amplitudes[i]);
            }
        }
        
        amplitudes[0]
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
    
    // Calculate standard deviation manually
    fn calculate_std(data: &[f64]) -> f64 {
        if data.len() < 2 {
            return 0.0;
        }
        
        let mean = data.iter().sum::<f64>() / data.len() as f64;
        let variance = data.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / (data.len() - 1) as f64;
        
        variance.sqrt()
    }
    
    // Advanced EMC analysis using statistical methods
    pub fn analyze_measurement_statistics(&self, frequencies: &[f64], amplitudes: &[f64]) -> EMCStatistics {
        // Calculate basic statistics
        let mean_amplitude = amplitudes.iter().sum::<f64>() / amplitudes.len() as f64;
        let std_amplitude = Self::calculate_std(amplitudes);
        let max_amplitude = amplitudes.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let min_amplitude = amplitudes.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        
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
    
    // Improved mask generation with logarithmic spacing
    pub fn generate_adaptive_mask(&self, f_min: f64, f_max: f64, target_points: usize) -> EMCMask {
        let log_min = f_min.log10();
        let log_max = f_max.log10();
        
        let mut avg_points = Vec::new();
        let mut qp_points = Vec::new();
        let mut pk_points = Vec::new();
        
        // Create logarithmically spaced frequency points
        for i in 0..target_points {
            let log_f = log_min + (i as f64 / (target_points - 1) as f64) * (log_max - log_min);
            let freq = 10.0_f64.powf(log_f);
            
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

// Legacy interpolation function for compatibility
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
    
    let mask = standard.generate_adaptive_mask(f_min, f_max, total_points);
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
mod tests {
    use super::*;

    #[test]
    fn test_interpolate_log() {
        let frequencies = vec![150000.0, 500000.0, 30000000.0];
        let amplitudes = vec![66.0, 60.0, 60.0];
        let target_freq = 300000.0;
        
        let result = EMCStandard::interpolate_log(&frequencies, &amplitudes, target_freq);
        
        // Should interpolate between 66.0 and 60.0
        assert!(result > 60.0 && result < 66.0);
    }
    
    #[test]
    fn test_calculate_percentile() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let p50 = EMCStandard::calculate_percentile(&data, 50.0);
        assert_eq!(p50, 3.0);
        
        let p95 = EMCStandard::calculate_percentile(&data, 95.0);
        assert_eq!(p95, 4.8);
    }
    
    #[test]
    fn test_calculate_std() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let std = EMCStandard::calculate_std(&data);
        assert!((std - 1.5811388300841898).abs() < 1e-10);
    }
}
