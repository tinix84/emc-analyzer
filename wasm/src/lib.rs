// wasm/src/lib.rs
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EMCStandard {
    pub name: String,
    pub f_avg_limit_mask: Vec<f64>,
    pub dbuv_avg_limit_mask: Vec<f64>,
    pub f_qp_limit_mask: Option<Vec<f64>>,
    pub dbuv_qp_limit_mask: Option<Vec<f64>>,
    pub f_pk_limit_mask: Option<Vec<f64>>,
    pub dbuv_pk_limit_mask: Option<Vec<f64>>,
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
pub struct MaskPoint {
    pub x: f64,
    pub y: f64,
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
            f_avg_limit_mask: vec![],
            dbuv_avg_limit_mask: vec![],
            f_qp_limit_mask: None,
            dbuv_qp_limit_mask: None,
            f_pk_limit_mask: None,
            dbuv_pk_limit_mask: None,
        }
    }

    fn interp_log(&self, new_frequency: f64) -> EMCLimitResult {
        let dbuv_avg_limit = log_interp(
            &self.f_avg_limit_mask,
            &self.dbuv_avg_limit_mask,
            new_frequency,
        );

        let dbuv_qp_limit = if let (Some(f_qp), Some(dbuv_qp)) = 
            (&self.f_qp_limit_mask, &self.dbuv_qp_limit_mask) {
            log_interp(f_qp, dbuv_qp, new_frequency)
        } else {
            0.0
        };

        let dbuv_pk_limit = if let (Some(f_pk), Some(dbuv_pk)) = 
            (&self.f_pk_limit_mask, &self.dbuv_pk_limit_mask) {
            log_interp(f_pk, dbuv_pk, new_frequency)
        } else {
            0.0
        };

        let avg_limit = 10.0_f64.powf((dbuv_avg_limit - 120.0) / 20.0);
        let qp_limit = 10.0_f64.powf((dbuv_qp_limit - 120.0) / 20.0);
        let pk_limit = 10.0_f64.powf((dbuv_pk_limit - 120.0) / 20.0);

        EMCLimitResult {
            avg_limit,
            qp_limit,
            pk_limit,
            dbuv_avg_limit,
            dbuv_qp_limit,
            dbuv_pk_limit,
        }
    }

    // Standard implementations (same as before)
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

fn log_interp(x_points: &[f64], y_points: &[f64], x: f64) -> f64 {
    if x_points.len() != y_points.len() || x_points.is_empty() {
        return 0.0;
    }

    let log_x = x.log10();
    let log_x_points: Vec<f64> = x_points.iter().map(|&val| val.log10()).collect();

    for i in 0..log_x_points.len() - 1 {
        if log_x >= log_x_points[i] && log_x <= log_x_points[i + 1] {
            let slope = (y_points[i + 1] - y_points[i]) / (log_x_points[i + 1] - log_x_points[i]);
            return y_points[i] + slope * (log_x - log_x_points[i]);
        }
    }

    if log_x < log_x_points[0] {
        y_points[0]
    } else {
        y_points[y_points.len() - 1]
    }
}

// WASM bindings
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn get_emc_standard(standard_name: &str, emc_class: &str, interface: Option<String>) -> Result<JsValue, JsValue> {
    let standard = match standard_name {
        "CISPR22" => EMCStandard::cispr22(emc_class),
        "EN55032" => EMCStandard::en55032(emc_class),
        "IEC61800_3" => {
            let intf = interface.unwrap_or_else(|| "AC".to_string());
            EMCStandard::iec61800_3(emc_class, &intf)
        }
        _ => Err("Unknown standard".to_string()),
    };

    match standard {
        Ok(std) => serde_wasm_bindgen::to_value(&std).map_err(|e| JsValue::from_str(&e.to_string())),
        Err(e) => Err(JsValue::from_str(&e)),
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
        
        avg_points.push(MaskPoint { x: freq, y: limit.dbuv_avg_limit });
        
        if limit.dbuv_qp_limit > 0.0 {
            qp_points.push(MaskPoint { x: freq, y: limit.dbuv_qp_limit });
        }
        
        if limit.dbuv_pk_limit > 0.0 {
            pk_points.push(MaskPoint { x: freq, y: limit.dbuv_pk_limit });
        }
    }
    
    let mask = EMCMask {
        avg: avg_points,
        qp: qp_points,
        pk: pk_points,
    };
    
    serde_wasm_bindgen::to_value(&mask).map_err(|e| JsValue::from_str(&e.to_string()))
}

