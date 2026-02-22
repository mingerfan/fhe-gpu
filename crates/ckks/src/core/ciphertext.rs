//! CKKS ciphertext type.
//!
//! A CKKS ciphertext is a pair of RNS polynomials `(c0, c1)` in R_Q_l,
//! where Q_l = q_0 * q_1 * ... * q_l is the current ciphertext modulus
//! at level `l`.
//!
//! # Decryption Formula
//! ```text
//! [c0 + c1 * s]_{Q_l} ≈ Δ * m   (approximately, with small noise)
//! ```
//!
//! # Level Semantics
//! - Fresh ciphertext: level = max_depth
//! - After each rescale: level decreases by 1
//! - At level 0: no more multiplications possible (out of budget)
//!
//! # Learning Resources
//! - [EN] CKKS ciphertext structure: https://eprint.iacr.org/2016/421.pdf §2
//! - [CN] CKKS 密文格式: N/A

use crate::core::params::CkksParams;
use fhe_math::rns::RnsPoly;
use serde::{Deserialize, Serialize};

/// A degree-1 CKKS ciphertext `(c0, c1)`.
///
/// Decrypts to `m ≈ (c0 + c1*s) / Δ` where `s` is the secret key.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ciphertext {
    /// `c0 = b*u + e0 + Δ*m` (public key encryption) or `-a*s + e + Δ*m` (symmetric)
    pub c0: RnsPoly,
    /// `c1 = a*u + e1` (public key encryption) or `a` (symmetric)
    pub c1: RnsPoly,
    /// Current level (= number of RNS limbs - 1).
    pub level: usize,
    /// Current scale factor Δ^k after k multiplications.
    pub scale: f64,
    /// Reference to the shared parameters.
    #[serde(skip)]
    pub params: Option<std::sync::Arc<CkksParams>>,
}

impl Ciphertext {
    /// Create a new ciphertext at the given level.
    pub fn new(c0: RnsPoly, c1: RnsPoly, level: usize, scale: f64) -> Self {
        Self { c0, c1, level, scale, params: None }
    }

    /// Number of RNS limbs at the current level (= level + 1).
    pub fn num_limbs(&self) -> usize {
        self.level + 1
    }
}
