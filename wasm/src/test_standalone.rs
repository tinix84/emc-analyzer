// wasm/src/test_standalone.rs
use crate::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_emc_standard_cispr25_class3() {
        // Test basic functionality without WASM
        let standard = get_emc_standard_rust("CISPR25", "Class3", None);
        
        assert!(!standard.name.is_empty());
        assert!(!standard.f_avg_limit_mask.is_empty());
        assert!(!standard.dbuv_avg_limit_mask.is_empty());
        
        println!("Standard name: {}", standard.name);
        println!("Frequency points: {}", standard.f_avg_limit_mask.len());
        println!("First frequency: {}", standard.f_avg_limit_mask[0]);
        println!("First limit: {}", standard.dbuv_avg_limit_mask[0]);
    }

    #[test]
    fn test_calculate_emc_limit_basic() {
        // Create a simple test standard
        let standard = EMCStandard {
            name: "Test Standard".to_string(),
            f_avg_limit_mask: vec![150000.0, 500000.0, 5000000.0, 30000000.0],
            dbuv_avg_limit_mask: vec![60.0, 50.0, 50.0, 70.0],
            f_qp_limit_mask: None,
            dbuv_qp_limit_mask: None,
            f_pk_limit_mask: None,
            dbuv_pk_limit_mask: None,
        };

        // Test frequency interpolation
        let result = calculate_emc_limit_rust(&standard, 1000000.0);
        
        println!("Limit at 1 MHz: {}", result.dbuv_avg_limit);
        assert!(result.dbuv_avg_limit > 0.0);
        assert!(result.dbuv_avg_limit < 100.0);
    }

    #[test]
    fn test_check_compliance_basic() {
        let standard = EMCStandard {
            name: "Test Standard".to_string(),
            f_avg_limit_mask: vec![150000.0, 30000000.0],
            dbuv_avg_limit_mask: vec![60.0, 70.0],
            f_qp_limit_mask: None,
            dbuv_qp_limit_mask: None,
            f_pk_limit_mask: None,
            dbuv_pk_limit_mask: None,
        };

        let frequencies = vec![1000000.0, 2000000.0];
        let amplitudes = vec![50.0, 80.0]; // First should pass, second should fail

        let results = check_emc_compliance_rust(&standard, &frequencies, &amplitudes, "avg");
        
        assert_eq!(results.len(), 2);
        assert!(results[0].passes); // 50 dBμV should pass
        assert!(!results[1].passes); // 80 dBμV should fail
        
        println!("First result: passes={}, margin={}", results[0].passes, results[0].margin);
        println!("Second result: passes={}, margin={}", results[1].passes, results[1].margin);
    }
}

// Pure Rust functions without WASM bindings for testing
pub fn get_emc_standard_rust(standard_name: &str, emc_class: &str, interface_type: Option<&str>) -> EMCStandard {
    // Implementation logic from the main function but without WASM
    match (standard_name, emc_class) {
        ("CISPR25", "Class3") => EMCStandard {
            name: format!("{} {}", standard_name, emc_class),
            f_avg_limit_mask: vec![150000.0, 500000.0, 5000000.0, 30000000.0],
            dbuv_avg_limit_mask: vec![60.0, 50.0, 50.0, 70.0],
            f_qp_limit_mask: Some(vec![150000.0, 500000.0, 5000000.0, 30000000.0]),
            dbuv_qp_limit_mask: Some(vec![66.0, 56.0, 56.0, 76.0]),
            f_pk_limit_mask: Some(vec![150000.0, 500000.0, 5000000.0, 30000000.0]),
            dbuv_pk_limit_mask: Some(vec![80.0, 70.0, 70.0, 90.0]),
        },
        ("CISPR25", "Class5") => EMCStandard {
            name: format!("{} {}", standard_name, emc_class),
            f_avg_limit_mask: vec![150000.0, 500000.0, 5000000.0, 30000000.0],
            dbuv_avg_limit_mask: vec![50.0, 40.0, 40.0, 60.0],
            f_qp_limit_mask: Some(vec![150000.0, 500000.0, 5000000.0, 30000000.0]),
            dbuv_qp_limit_mask: Some(vec![56.0, 46.0, 46.0, 66.0]),
            f_pk_limit_mask: Some(vec![150000.0, 500000.0, 5000000.0, 30000000.0]),
            dbuv_pk_limit_mask: Some(vec![70.0, 60.0, 60.0, 80.0]),
        },
        _ => EMCStandard {
            name: "Unknown Standard".to_string(),
            f_avg_limit_mask: vec![150000.0, 30000000.0],
            dbuv_avg_limit_mask: vec![60.0, 70.0],
            f_qp_limit_mask: None,
            dbuv_qp_limit_mask: None,
            f_pk_limit_mask: None,
            dbuv_pk_limit_mask: None,
        },
    }
}

pub fn calculate_emc_limit_rust(standard: &EMCStandard, frequency: f64) -> EMCLimitResult {
    let avg_limit = interpolate(&standard.f_avg_limit_mask, &standard.dbuv_avg_limit_mask, frequency);
    
    let qp_limit = if let (Some(f_qp), Some(qp)) = (&standard.f_qp_limit_mask, &standard.dbuv_qp_limit_mask) {
        interpolate(f_qp, qp, frequency)
    } else {
        avg_limit + 6.0
    };
    
    let pk_limit = if let (Some(f_pk), Some(pk)) = (&standard.f_pk_limit_mask, &standard.dbuv_pk_limit_mask) {
        interpolate(f_pk, pk, frequency)
    } else {
        qp_limit + 14.0
    };

    EMCLimitResult {
        avg_limit: frequency,
        qp_limit: frequency,
        pk_limit: frequency,
        dbuv_avg_limit: avg_limit,
        dbuv_qp_limit: qp_limit,
        dbuv_pk_limit: pk_limit,
    }
}

pub fn check_emc_compliance_rust(
    standard: &EMCStandard,
    frequencies: &[f64],
    amplitudes: &[f64],
    measurement_type: &str,
) -> Vec<ComplianceResult> {
    frequencies
        .iter()
        .zip(amplitudes.iter())
        .map(|(&freq, &amp)| {
            let limit_result = calculate_emc_limit_rust(standard, freq);
            
            let limit = match measurement_type {
                "qp" => limit_result.dbuv_qp_limit,
                "pk" => limit_result.dbuv_pk_limit,
                _ => limit_result.dbuv_avg_limit,
            };
            
            let margin = limit - amp;
            
            ComplianceResult {
                frequency: freq,
                amplitude: amp,
                limit,
                passes: amp <= limit,
                margin,
            }
        })
        .collect()
}

fn interpolate(x_values: &[f64], y_values: &[f64], x: f64) -> f64 {
    if x_values.len() != y_values.len() || x_values.is_empty() {
        return 0.0;
    }

    if x <= x_values[0] {
        return y_values[0];
    }

    if x >= x_values[x_values.len() - 1] {
        return y_values[y_values.len() - 1];
    }

    for i in 0..x_values.len() - 1 {
        if x >= x_values[i] && x <= x_values[i + 1] {
            let x1 = x_values[i];
            let x2 = x_values[i + 1];
            let y1 = y_values[i];
            let y2 = y_values[i + 1];

            return y1 + (y2 - y1) * (x - x1) / (x2 - x1);
        }
    }

    0.0
}
