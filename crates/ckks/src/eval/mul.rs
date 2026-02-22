//! Homomorphic multiplication.
//!
//! Ciphertext multiplication is the most expensive operation in CKKS.
//! A fresh degree-1 ciphertext `(c0, c1)` multiplied with `(d0, d1)` produces
//! a degree-2 ciphertext `(e0, e1, e2)` that must be relinearized.
//!
//! # Mathematical Specification
//! ```text
//! Tensor product:
//!   e0 = c0 * d0
//!   e1 = c0 * d1 + c1 * d0
//!   e2 = c1 * d1
//!
//! (e0, e1, e2) decrypts to (c0 + c1*s) * (d0 + d1*s) = m1*m2 (approximately)
//!
//! After multiplication, the scale doubles: Δ² instead of Δ.
//! Rescaling then divides by one prime factor to bring scale back to ≈ Δ.
//! ```
//!
//! # Learning Resources
//! - [EN] CKKS multiplication: https://eprint.iacr.org/2016/421.pdf §3
//! - [EN] Multiplication depth and noise: https://eprint.iacr.org/2016/421.pdf §4
//! - [CN] CKKS 同态乘法推导: N/A
//! - [CN] 张量积与密文度数: N/A

use crate::{
    core::{Ciphertext, Plaintext},
    CkksError,
};

/// Degree-2 ciphertext resulting from multiplication (before relinearization).
///
/// Decrypts as: `e0 + e1*s + e2*s^2 ≈ m1*m2 * Δ^2`.
pub struct Degree2Ciphertext {
    pub e0: fhe_math::rns::RnsPoly,
    pub e1: fhe_math::rns::RnsPoly,
    pub e2: fhe_math::rns::RnsPoly,
    pub level: usize,
    pub scale: f64,
}

/// Multiply two degree-1 ciphertexts, producing a degree-2 ciphertext.
///
/// **Must be followed by `relinearize` to return to degree-1 form.**
///
/// # Mathematical Specification
/// ```text
/// Tensor product (all operations in R_{Q_l}):
///   e0 = c0 * d0        mod Q_l
///   e1 = c0 * d1 + c1 * d0    mod Q_l
///   e2 = c1 * d1        mod Q_l
/// output_scale = ct1.scale * ct2.scale
/// ```
///
/// # Learning Resources
/// - [EN] CKKS tensor product: https://eprint.iacr.org/2016/421.pdf §3.2
/// - [CN] 密文乘法张量积: N/A
pub fn mul_ct_ct(ct1: &Ciphertext, ct2: &Ciphertext) -> Result<Degree2Ciphertext, CkksError> {
    if ct1.level != ct2.level {
        return Err(CkksError::Eval(format!(
            "mul_ct_ct: level mismatch {} vs {}", ct1.level, ct2.level
        )));
    }
    todo!(
        "compute e0 = ct1.c0 * ct2.c0, \
         e1 = ct1.c0 * ct2.c1 + ct1.c1 * ct2.c0, \
         e2 = ct1.c1 * ct2.c1; \
         all in NTT domain for efficiency"
    )
}

/// Multiply a ciphertext by a plaintext (much cheaper than ct*ct).
///
/// # Mathematical Specification
/// ```text
/// (c0 * pt.poly, c1 * pt.poly) mod Q_l
/// output_scale = ct.scale * pt.scale
/// ```
///
/// # Learning Resources
/// - [EN] CKKS plaintext multiplication: https://eprint.iacr.org/2016/421.pdf §3
/// - [CN] 明密文乘法（更高效）: N/A
pub fn mul_ct_pt(ct: &Ciphertext, pt: &Plaintext) -> Result<Ciphertext, CkksError> {
    if ct.level != pt.level {
        return Err(CkksError::Eval(format!(
            "mul_ct_pt: level mismatch ct={} pt={}", ct.level, pt.level
        )));
    }
    todo!(
        "c0 = ct.c0 * pt.poly mod Q; c1 = ct.c1 * pt.poly mod Q; \
         output scale = ct.scale * pt.scale"
    )
}
