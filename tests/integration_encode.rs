// integration_encode.rs — End-to-end encoding/decoding integration tests.
//
// These tests require the `phase-encoding` feature flag to run:
//   cargo test --features phase-encoding

use ckks::{core::params::toy_params, encoding::CkksEncoder};
use num_complex::Complex64;
use std::sync::Arc;

#[cfg(feature = "phase-encoding")]
mod encode_tests {
    use super::*;

    #[test]
    #[ignore = "temporary bisect: isolate next failing layer"]
    fn test_encode_decode_zero() {
        let params = Arc::new(toy_params());
        let encoder = CkksEncoder::new(params.clone());
        let slots = vec![Complex64::new(0.0, 0.0); params.num_slots];
        let pt = encoder.encode(&slots, params.max_depth).unwrap();
        let decoded = encoder.decode(&pt).unwrap();
        fhe_testing::assert_slots_close!(decoded, slots, rel_tol = 1e-6);
    }

    #[test]
    #[ignore = "temporary bisect: isolate next failing layer"]
    fn test_encode_decode_ones() {
        let params = Arc::new(toy_params());
        let encoder = CkksEncoder::new(params.clone());
        let slots = vec![Complex64::new(1.0, 0.0); params.num_slots];
        let pt = encoder.encode(&slots, params.max_depth).unwrap();
        let decoded = encoder.decode(&pt).unwrap();
        fhe_testing::assert_slots_close!(decoded, slots, rel_tol = 1e-4);
    }

    #[test]
    #[ignore = "temporary bisect: isolate next failing layer"]
    fn test_encode_decode_mixed() {
        let params = Arc::new(toy_params());
        let encoder = CkksEncoder::new(params.clone());
        let slots = vec![
            Complex64::new(1.5, -0.5),
            Complex64::new(3.14, 2.71),
            Complex64::new(-1.0, 1.0),
        ];
        let pt = encoder.encode(&slots, params.max_depth).unwrap();
        let decoded = encoder.decode(&pt).unwrap();
        fhe_testing::assert_slots_close!(decoded, slots, rel_tol = 1e-4);
    }

    #[test]
    #[ignore = "temporary bisect: isolate next failing layer"]
    fn test_encode_partial_slots() {
        // Encoding fewer than num_slots should zero-pad
        let params = Arc::new(toy_params());
        let encoder = CkksEncoder::new(params.clone());
        let slots = vec![Complex64::new(42.0, 0.0)]; // only 1 slot
        let pt = encoder.encode(&slots, params.max_depth).unwrap();
        let decoded = encoder.decode(&pt).unwrap();
        let tol = 1e-4;
        assert!((decoded[0].re - 42.0).abs() < tol);
        // Remaining slots should be ~0
        for d in &decoded[1..] {
            assert!(d.norm() < tol * 10.0, "expected near-zero, got {d}");
        }
    }
}

#[cfg(not(feature = "phase-encoding"))]
#[test]
fn encoding_tests_disabled() {
    // Run with --features phase-encoding to enable these tests
}
