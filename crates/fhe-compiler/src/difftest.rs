//! Differential testing harness: compare Rust CKKS output against Python OpenFHE oracle.
//!
//! The harness spawns a Python subprocess running `fhe_oracle` (see `oracle/` directory),
//! sends operations as JSON over stdin, and reads JSON results from stdout.
//!
//! # Protocol
//! ```json
//! // Request (Rust → Python)
//! {
//!   "operation": "add_ct_ct",
//!   "inputs": { "x": [[1.5, 0.0], [2.3, 0.0]], "y": [[0.5, 0.0], [1.0, 0.0]] },
//!   "params": { "poly_degree": 4096, "scale_bits": 40, "depth": 3 }
//! }
//!
//! // Response (Python → Rust)
//! {
//!   "result": [[1.999, 0.0], [3.299, 0.0]],
//!   "scale": 1099511627776.0
//! }
//! ```
//!
//! # Usage
//! ```rust,ignore
//! let harness = DifftestHarness::new("oracle/")?;
//! let result = harness.run_operation("add_ct_ct", inputs, params)?;
//! ```

use crate::CompilerError;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, process::{Command, Stdio}};

/// Parameters sent to the oracle for each operation.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OracleParams {
    pub poly_degree: usize,
    pub scale_bits: u32,
    pub depth: usize,
}

/// A request to the Python oracle.
#[derive(Debug, Serialize)]
pub struct OracleRequest {
    pub operation: String,
    /// Each input is a list of [re, im] pairs.
    pub inputs: HashMap<String, Vec<[f64; 2]>>,
    pub params: OracleParams,
}

/// A response from the Python oracle.
#[derive(Debug, Deserialize)]
pub struct OracleResponse {
    /// The result as [re, im] pairs.
    pub result: Vec<[f64; 2]>,
    pub scale: f64,
}

/// Differential testing harness.
pub struct DifftestHarness {
    oracle_dir: PathBuf,
}

impl DifftestHarness {
    /// Create a new harness pointing to the oracle directory.
    pub fn new(oracle_dir: impl Into<PathBuf>) -> Result<Self, CompilerError> {
        let dir = oracle_dir.into();
        if !dir.exists() {
            return Err(CompilerError::Difftest(format!(
                "oracle directory not found: {}", dir.display()
            )));
        }
        Ok(Self { oracle_dir: dir })
    }

    /// Run a single FHE operation via the Python oracle and return the result.
    ///
    /// # Protocol
    /// Spawns `uv run python -m fhe_oracle` in `oracle_dir`,
    /// writes JSON to stdin, reads JSON from stdout.
    pub fn run_operation(
        &self,
        operation: &str,
        inputs: HashMap<String, Vec<[f64; 2]>>,
        params: OracleParams,
    ) -> Result<OracleResponse, CompilerError> {
        let request = OracleRequest {
            operation: operation.to_string(),
            inputs,
            params,
        };
        let json_input = serde_json::to_string(&request)?;

        let output = Command::new("uv")
            .args(["run", "python", "-m", "fhe_oracle"])
            .current_dir(&self.oracle_dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                use std::io::Write;
                child.stdin.as_mut().unwrap().write_all(json_input.as_bytes())?;
                child.wait_with_output()
            })
            .map_err(|e| CompilerError::Subprocess(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CompilerError::Difftest(format!("oracle failed: {stderr}")));
        }

        let response: OracleResponse = serde_json::from_slice(&output.stdout)?;
        Ok(response)
    }

    /// Compare Rust result against oracle result, failing if they differ by more than `tol`.
    pub fn assert_close(
        rust_result: &[[f64; 2]],
        oracle_result: &[[f64; 2]],
        tol: f64,
    ) -> Result<(), CompilerError> {
        if rust_result.len() != oracle_result.len() {
            return Err(CompilerError::Difftest(format!(
                "result length mismatch: rust={} oracle={}", rust_result.len(), oracle_result.len()
            )));
        }
        for (i, (r, o)) in rust_result.iter().zip(oracle_result.iter()).enumerate() {
            let re_err = (r[0] - o[0]).abs();
            let im_err = (r[1] - o[1]).abs();
            if re_err > tol || im_err > tol {
                return Err(CompilerError::Difftest(format!(
                    "slot {i}: rust=[{}, {}] oracle=[{}, {}] re_err={re_err:.2e} im_err={im_err:.2e} tol={tol:.2e}",
                    r[0], r[1], o[0], o[1]
                )));
            }
        }
        Ok(())
    }
}
