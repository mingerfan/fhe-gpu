//! CKKS evaluator: central struct for all homomorphic operations.

use crate::{
    core::{keys::{GaloisKey, RelinKey}, params::CkksParams},
    CkksError,
};
use std::{collections::HashMap, sync::Arc};

/// The main CKKS evaluator: holds parameters and evaluation keys.
pub struct CkksEvaluator {
    pub params: Arc<CkksParams>,
    pub rlk: RelinKey,
    pub galois_keys: HashMap<i32, GaloisKey>,
}

impl CkksEvaluator {
    pub fn new(
        params: Arc<CkksParams>,
        rlk: RelinKey,
        galois_keys: HashMap<i32, GaloisKey>,
    ) -> Self {
        Self { params, rlk, galois_keys }
    }

    /// Get the Galois key for a rotation step.
    pub fn get_galois_key(&self, step: i32) -> Result<&GaloisKey, CkksError> {
        self.galois_keys
            .get(&step)
            .ok_or_else(|| CkksError::KeyNotFound(format!("Galois key for step={step}")))
    }
}
