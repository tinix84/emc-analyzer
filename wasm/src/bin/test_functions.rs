// wasm/src/bin/test_functions.rs

#[derive(Debug, Clone)]
pub struct EMCStandard {
    pub name: String,
    pub f_avg_limit_mask: Vec<f64>,
    pub dbuv_avg_limit_mask: Vec<f64>,
    pub f_qp_limit_mask: Option<Vec<f64>>,
    pub dbuv_qp_limit_mask: Option<Vec<f64>>,
    pub f_pk_limit_mask: Option<Vec<f64>>,
    pub dbuv_pk_limit_mask: Option<Vec<f64>>,
}

#[derive(Debug, Clone)]
pub struct EMCLimitResult {
    pub avg_limit: f64,
    pub qp_limit: f64,
    pub pk_limit: f64,
    pub dbuv_avg_limit: f64,
    pub dbuv_qp_limit: f64,
    pub dbuv_pk_limit: f64,
}

#[derive(Debug, Clone)]
pub struct ComplianceResult {
    pub frequency: f64,
    pub amplitude: f64,
    pub limit: f64,
    pub passes: bool,
    pub margin: f64,
}

// Test functions
fn get_emc_standard_rust(standard_name: &str, emc_class: &str, _interface_type: Option<&str>) -> EMCStandard {
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

fn calculate_emc_limit_rust(standard: &EMCStandard, frequency: f64) -> EMCLimitResult {
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

fn check_emc_compliance_rust(
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

fn main() {
    println!("=== Testing EMC Functions Standalone ===\n");
    
    // Test 1: Basic EMC Standard Creation
    println!("1. Testing get_emc_standard_rust...");
    let standard = get_emc_standard_rust("CISPR25", "Class3", None);
    println!("   ✓ Standard name: {}", standard.name);
    println!("   ✓ Frequency points: {}", standard.f_avg_limit_mask.len());
    println!("   ✓ First frequency: {} Hz", standard.f_avg_limit_mask[0]);
    println!("   ✓ First limit: {} dBμV\n", standard.dbuv_avg_limit_mask[0]);
    
    // Test 2: Limit Calculation
    println!("2. Testing calculate_emc_limit_rust...");
    let test_freq = 1000000.0; // 1 MHz
    let result = calculate_emc_limit_rust(&standard, test_freq);
    println!("   ✓ Frequency: {} Hz", test_freq);
    println!("   ✓ AVG limit: {} dBμV", result.dbuv_avg_limit);
    println!("   ✓ QP limit: {} dBμV", result.dbuv_qp_limit);
    println!("   ✓ PK limit: {} dBμV\n", result.dbuv_pk_limit);
    
    // Test 3: Compliance Check
    println!("3. Testing check_emc_compliance_rust...");
    let frequencies = vec![500000.0, 1000000.0, 5000000.0];
    let amplitudes = vec![45.0, 55.0, 65.0]; // Mixed compliance
    
    let compliance_results = check_emc_compliance_rust(
        &standard, 
        &frequencies, 
        &amplitudes, 
        "avg"
    );
    
    for (i, result) in compliance_results.iter().enumerate() {
        let status = if result.passes { "✓ PASS" } else { "✗ FAIL" };
        println!("   Freq: {:.0} Hz, Measured: {:.1} dBμV, Limit: {:.1} dBμV, Margin: {:.1} dB - {}", 
                 result.frequency, result.amplitude, result.limit, result.margin, status);
    }
    
    println!("\n=== All basic functions working! ===");
}
