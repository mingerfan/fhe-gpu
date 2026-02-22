// integration_difftest.rs — Differential testing against Python OpenFHE oracle.
//
// Run with: RUN_DIFFTESTS=1 cargo test --features difftest

#[cfg(feature = "difftest")]
mod difftest {
    use fhe_compiler::difftest::{DifftestHarness, OracleParams};
    use std::collections::HashMap;
    use std::env;

    fn oracle_params() -> OracleParams {
        OracleParams {
            poly_degree: 4096,
            scale_bits: 40,
            depth: 3,
        }
    }

    fn should_run() -> bool {
        env::var("RUN_DIFFTESTS").map(|v| v == "1").unwrap_or(false)
    }

    fn oracle_dir() -> std::path::PathBuf {
        // Relative to workspace root
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()  // tests/
            .parent().unwrap()  // workspace root
            .join("oracle")
    }

    #[test]
    fn difftest_add_ct_ct() {
        if !should_run() { return; }

        let harness = DifftestHarness::new(oracle_dir()).expect("oracle dir not found");

        // Reference computation in plain Rust
        let x = vec![[1.5f64, 0.0], [2.3, 0.0], [0.0, 1.0]];
        let y = vec![[0.5f64, 0.0], [1.0, 0.0], [0.0, -1.0]];
        let expected: Vec<[f64; 2]> = x.iter().zip(y.iter())
            .map(|(a, b)| [a[0] + b[0], a[1] + b[1]])
            .collect();

        let mut inputs = HashMap::new();
        inputs.insert("x".to_string(), x);
        inputs.insert("y".to_string(), y);

        let response = harness.run_operation("add_ct_ct", inputs, oracle_params()).unwrap();

        // Compare oracle result against expected
        DifftestHarness::assert_close(&expected, &response.result, 1e-3).unwrap();
    }

    #[test]
    fn difftest_mul_ct_ct() {
        if !should_run() { return; }

        let harness = DifftestHarness::new(oracle_dir()).expect("oracle dir not found");

        let x = vec![[2.0f64, 0.0], [3.0, 0.0]];
        let y = vec![[4.0f64, 0.0], [5.0, 0.0]];
        let expected = vec![[8.0f64, 0.0], [15.0, 0.0]];

        let mut inputs = HashMap::new();
        inputs.insert("x".to_string(), x);
        inputs.insert("y".to_string(), y);

        let response = harness.run_operation("mul_ct_ct", inputs, oracle_params()).unwrap();
        DifftestHarness::assert_close(&expected, &response.result, 1e-3).unwrap();
    }

    // TODO: add difftest_add_vs_rust, difftest_mul_vs_rust after Rust impl is complete
}

#[cfg(not(feature = "difftest"))]
#[test]
fn difftest_disabled() {
    // Run with RUN_DIFFTESTS=1 cargo test --features difftest to enable
}
