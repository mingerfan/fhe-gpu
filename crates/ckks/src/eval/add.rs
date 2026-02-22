//! Homomorphic addition, subtraction, and negation.
//!
//! These are the cheapest homomorphic operations — they cost only
//! coefficient-wise addition modulo Q_l.
//!
//! # Mathematical Specification
//! ```text
//! Add: (c0, c1) + (d0, d1) = (c0 + d0, c1 + d1) mod Q_l
//! Sub: (c0, c1) - (d0, d1) = (c0 - d0, c1 - d1) mod Q_l
//! Neg: -(c0, c1)            = (-c0, -c1) mod Q_l
//! ```
//!
//! # Scale Compatibility
//! Addition requires both ciphertexts to have the same level AND scale.
//! If scales differ, one must be rescaled first.
//!
//! # Learning Resources
//! - [EN] CKKS homomorphic addition: https://eprint.iacr.org/2016/421.pdf §3
//! - [CN] CKKS 同态加法: N/A

use crate::{
    core::{params::CkksParams, Ciphertext, Plaintext},
    CkksError,
};
use fhe_math::rns::RnsPoly;

/// Homomorphic addition of two ciphertexts.
///
/// Both ciphertexts must be at the same level. The output scale equals
/// the input scale (no change).
///
/// # Errors
/// Returns `CkksError::Eval` if levels or scales don't match.
pub fn add_ct_ct(ct1: &Ciphertext, ct2: &Ciphertext) -> Result<Ciphertext, CkksError> {
    if ct1.level != ct2.level {
        return Err(CkksError::Eval(format!(
            "add_ct_ct: level mismatch {} vs {}", ct1.level, ct2.level
        )));
    }
    todo!("c0 = ct1.c0 + ct2.c0; c1 = ct1.c1 + ct2.c1; return Ciphertext with same level/scale")
}

/// Homomorphic addition of a ciphertext and a plaintext.
///
/// # Mathematical Specification
/// ```text
/// (c0 + pt.poly, c1)
/// ```
pub fn add_ct_pt(ct: &Ciphertext, pt: &Plaintext) -> Result<Ciphertext, CkksError> {
    if ct.level != pt.level {
        return Err(CkksError::Eval(format!(
            "add_ct_pt: level mismatch ct={} pt={}", ct.level, pt.level
        )));
    }
    todo!("c0 = ct.c0 + pt.poly; c1 = ct.c1 unchanged")
}

/// Homomorphic subtraction: `ct1 - ct2`.
pub fn sub_ct_ct(ct1: &Ciphertext, ct2: &Ciphertext) -> Result<Ciphertext, CkksError> {
    if ct1.level != ct2.level {
        return Err(CkksError::Eval(format!(
            "sub_ct_ct: level mismatch {} vs {}", ct1.level, ct2.level
        )));
    }
    todo!("c0 = ct1.c0 - ct2.c0; c1 = ct1.c1 - ct2.c1")
}

/// Homomorphic negation: `-ct`.
pub fn negate(ct: &Ciphertext) -> Ciphertext {
    todo!("c0 = -ct.c0 mod Q; c1 = -ct.c1 mod Q; keep level/scale")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_level_mismatch() {
        // Build two dummy ciphertexts at different levels
        use fhe_math::rns::{RnsBasis, RnsPoly};
        let basis = RnsBasis::new(vec![7681]);
        let zero = RnsPoly::zero(4, basis);
        let ct1 = Ciphertext::new(zero.clone(), zero.clone(), 2, 1.0);
        let ct2 = Ciphertext::new(zero.clone(), zero.clone(), 1, 1.0);
        assert!(add_ct_ct(&ct1, &ct2).is_err());
    }
}
