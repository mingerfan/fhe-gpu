//! Rescaling: reduce ciphertext scale after multiplication.
//!
//! After multiplying two ciphertexts with scale Δ each, the result has scale Δ².
//! Rescaling divides all coefficients by q_l (the last RNS prime), dropping
//! one limb and reducing the scale back to approximately Δ.
//!
//! # Mathematical Specification
//! ```text
//! Input: (c0, c1) at level l with scale Δ²
//!
//! For each limb i in 0..l-1:
//!   Δ_l  = c0.limbs[l] (the last limb, scalar at each coeff position)
//!   c0_new.limbs[i] = (c0.limbs[i] - Δ_l) * q_l^{-1} mod q_i
//!   (same for c1)
//!
//! Output: (c0_new, c1_new) at level l-1 with scale Δ² / q_l ≈ Δ
//! ```
//!
//! # Why This Works
//! The "rounding error" from integer division is bounded by q_l, which is
//! small relative to Q_{l-1}, so it contributes only a tiny additive noise.
//!
//! # Learning Resources
//! - [EN] CKKS Rescale operation: https://eprint.iacr.org/2016/421.pdf §3
//! - [EN] Full RNS rescale: https://eprint.iacr.org/2018/931.pdf §2.4
//! - [CN] CKKS Rescale 详解（知乎）: N/A
//! - [CN] ModDown 与 Rescale 对比: N/A

use crate::{core::{params::CkksParams, Ciphertext}, CkksError};
use std::sync::Arc;

/// Rescale a ciphertext: drop one RNS limb and adjust scale.
///
/// This is the operation that keeps the scale from growing unboundedly
/// after repeated multiplications.
///
/// # Errors
/// Returns error if `ct.level == 0` (no limbs to drop).
///
/// # Learning Resources
/// - [EN] RNS ModDown (the underlying primitive): https://eprint.iacr.org/2018/931.pdf
/// - [CN] Rescale 实现步骤: N/A
pub fn rescale(ct: &Ciphertext, params: &CkksParams) -> Result<Ciphertext, CkksError> {
    if ct.level == 0 {
        return Err(CkksError::Eval("rescale: ciphertext is at level 0, cannot drop further".into()));
    }
    todo!(
        "call RnsPoly::mod_down on both c0 and c1; \
         new_scale = ct.scale / params.q_chain[ct.level]; \
         new_level = ct.level - 1"
    )
}
