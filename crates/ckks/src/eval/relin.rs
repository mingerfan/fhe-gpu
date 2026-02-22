//! Relinearization: reduce a degree-2 ciphertext back to degree 1.
//!
//! After ciphertext multiplication, we have `(e0, e1, e2)` where `e2` is the
//! "extra" component involving `s^2`. Relinearization uses the relinearization
//! key (an encryption of `s^2`) to absorb `e2` back into `(c0, c1)`.
//!
//! # Mathematical Specification (Digit Decomposition)
//! ```text
//! Decompose e2 into digits: e2 = Σ_d e2_d * P^d  (base-P decomposition)
//!
//! For each digit d:
//!   c0 += e2_d * rlk[d].0     (where rlk[d].0 ≈ P^d * s^2 * something)
//!   c1 += e2_d * rlk[d].1
//!
//! Result: (e0 + Σ corrections to c0, e1 + Σ corrections to c1)
//! ```
//!
//! # Noise Cost
//! Relinearization adds a small noise term bounded by σ√n * B_gadget.
//!
//! # Learning Resources
//! - [EN] Relinearization procedure: https://eprint.iacr.org/2011/277.pdf §3
//! - [EN] BV key switching: https://eprint.iacr.org/2012/144.pdf
//! - [EN] Hybrid key switching (modern): N/A
//! - [CN] 重线性化原理详解: N/A
//! - [CN] 密钥切换的数字分解法: N/A

use crate::{
    core::{keys::RelinKey, Ciphertext},
    eval::mul::Degree2Ciphertext,
    CkksError,
};

/// Relinearize a degree-2 ciphertext into a degree-1 ciphertext.
///
/// Consumes a `Degree2Ciphertext` and applies the relinearization key
/// to produce a standard `Ciphertext` with the same decryption semantics.
///
/// # Mathematical Specification
/// ```text
/// Input: (e0, e1, e2) with e2 the "excess" term from s^2
///
/// Digit decomposition of e2:
///   Write e2 = Σ_{d=0}^{D-1} e2_d * B^d  (B = decomposition base)
///
/// Apply rlk:
///   c0 = e0 + Σ_d e2_d * rlk[d].b
///   c1 = e1 + Σ_d e2_d * rlk[d].a
///
/// This is correct because rlk[d].b ≈ P^d * s^2 (encrypted),
/// so the sum approximates adding e2 * s^2 to the decryption.
/// ```
///
/// # Learning Resources
/// - [EN] Relinearization correctness proof: https://eprint.iacr.org/2011/277.pdf §3
/// - [CN] 重线性化正确性证明: N/A
pub fn relinearize(
    ct2: Degree2Ciphertext,
    rlk: &RelinKey,
) -> Result<Ciphertext, CkksError> {
    todo!(
        "decompose ct2.e2 into digits (base-P); \
         for each digit d: accumulate e2_d * rlk.digits[d].0 into c0, \
         e2_d * rlk.digits[d].1 into c1; \
         return (ct2.e0 + c0_acc, ct2.e1 + c1_acc)"
    )
}
