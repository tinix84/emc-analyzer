// Helper module for frequency constants with expressions
// This demonstrates how to use expressions in Rust code for better readability

// Frequency units as constants
pub const KHZ: f64 = 1e3;
pub const MHZ: f64 = 1e6;
pub const GHZ: f64 = 1e9;

// EMC frequency bands
pub mod frequency_bands {
    use super::*;
    
    // Standard EMC frequency ranges
    pub const LF_START: f64 = 150.0 * KHZ;           // 150 kHz
    pub const LF_END: f64 = 30.0 * MHZ;              // 30 MHz
    pub const VHF_START: f64 = 30.0 * MHZ;           // 30 MHz
    pub const VHF_END: f64 = 300.0 * MHZ;            // 300 MHz
    pub const UHF_START: f64 = 300.0 * MHZ;          // 300 MHz
    pub const UHF_END: f64 = 1.0 * GHZ;              // 1 GHz
    
    // Common EMC test frequencies
    pub const CISPR_BREAK_1: f64 = 500.0 * KHZ;      // 500 kHz
    pub const CISPR_BREAK_2: f64 = 5.0 * MHZ;        // 5 MHz
    pub const CISPR_BREAK_3: f64 = 30.0 * MHZ;       // 30 MHz
}

// Example of how to define standards with expressions in Rust
pub mod standard_definitions {
    use super::*;
    use super::frequency_bands::*;
    
    // CISPR 22 Class A with expression-based frequencies
    pub fn cispr22_class_a_frequencies() -> Vec<f64> {
        vec![
            150.0 * KHZ,                    // 1.5e5
            500.0 * KHZ,                    // 5e5  
            500.0 * KHZ + 1.0,              // 5e5 + 1 (discontinuity point)
            30.0 * MHZ,                     // 3e7
            100.0 * MHZ,                    // 1e8
            100.0 * MHZ + 1.0,              // 1e8 + 1 (discontinuity point)
        ]
    }
    
    pub fn cispr22_class_a_limits() -> Vec<f64> {
        vec![79.0, 73.0, 73.0, 40.0, 40.0, 47.0]
    }
    
    // Automotive ECE R10 with more complex expressions
    pub fn ece_r10_ac_frequencies() -> Vec<f64> {
        vec![
            150.0 * KHZ,                    // Start frequency
            300.0 * KHZ,                    // 2x start frequency
            1.4 * MHZ,                      // 1.4 MHz
            2.8 * MHZ,                      // 2x 1.4 MHz
            30.0 * MHZ,                     // Standard boundary
            54.0 * MHZ,                     // TV Band I
            108.0 * MHZ,                    // FM Band / TV Band III
        ]
    }
    
    // Function to create frequency ranges with expressions
    pub fn create_log_range(start: f64, end: f64, points_per_decade: usize) -> Vec<f64> {
        let log_start = start.log10();
        let log_end = end.log10();
        let decades = log_end - log_start;
        let total_points = (decades * points_per_decade as f64).ceil() as usize;
        
        (0..=total_points)
            .map(|i| {
                let log_f = log_start + (i as f64 / total_points as f64) * decades;
                10.0_f64.powf(log_f)
            })
            .collect()
    }
}

// Example usage and tests
#[cfg(test)]
mod tests {
    use super::*;
    use super::standard_definitions::*;
    
    #[test]
    fn test_frequency_constants() {
        assert_eq!(150.0 * KHZ, 150_000.0);
        assert_eq!(30.0 * MHZ, 30_000_000.0);
        assert_eq!(1.0 * GHZ, 1_000_000_000.0);
    }
    
    #[test]
    fn test_expression_frequencies() {
        let frequencies = cispr22_class_a_frequencies();
        
        // Test that expressions work correctly
        assert_eq!(frequencies[0], 150_000.0);           // 150 kHz
        assert_eq!(frequencies[1], 500_000.0);           // 500 kHz
        assert_eq!(frequencies[2], 500_001.0);           // 500 kHz + 1
        assert_eq!(frequencies[3], 30_000_000.0);        // 30 MHz
        assert_eq!(frequencies[4], 100_000_000.0);       // 100 MHz
        assert_eq!(frequencies[5], 100_000_001.0);       // 100 MHz + 1
    }
    
    #[test]
    fn test_log_range_generation() {
        let frequencies = create_log_range(100.0 * KHZ, 1.0 * GHZ, 10);
        
        // Should have logarithmic spacing
        assert!(frequencies.len() > 10);
        assert!(frequencies[0] >= 100_000.0);
        assert!(frequencies.last().unwrap() <= &1_000_000_000.0);
        
        // Should be monotonically increasing
        for i in 1..frequencies.len() {
            assert!(frequencies[i] > frequencies[i-1]);
        }
    }
}
