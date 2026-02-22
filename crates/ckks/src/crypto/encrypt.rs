//! CKKS encryption.
//!
//! # Public Key Encryption
//! To encrypt plaintext polynomial `m`:
//! ```text
//! u ← SmallRandom(R_Q)   (ephemeral key, ternary)
//! e0, e1 ← Error(σ)
//! c0 = pk_0 * u + e0 + m
//! c1 = pk_1 * u + e1
//! ct = (c0, c1)
//! ```
//!
//! # Symmetric Encryption
//! Encrypts under the secret key directly (used for batch operations):
//! ```text
//! a ← U(R_Q)
//! e ← Error(σ)
//! c0 = -a * s + e + m
//! c1 = a
//! ```
//!
//! # Learning Resources
//! - [EN] RLWE encryption: https://eprint.iacr.org/2012/230.pdf §3
//! - [EN] CKKS encrypt (§3.2): https://eprint.iacr.org/2016/421.pdf
//! - [CN] CKKS 加密推导: N/A
//! - [CN] RLWE 加密安全性分析: N/A

use crate::{
    core::{keys::{PublicKey, SecretKey}, params::CkksParams, Ciphertext, Plaintext},
    CkksError,
};
use rand::RngCore;
use std::sync::Arc;

/// Encrypt a plaintext using the public key.
///
/// # Mathematical Specification
/// ```text
/// u ← Ternary(R_Q)   (ephemeral randomness)
/// e0, e1 ← DiscreteGaussian(σ)
/// c0 = pk.b * u + e0 + pt.poly
/// c1 = pk.a * u + e1
/// return (c0, c1, level, scale)
/// ```
///
/// # Learning Resources
/// - [EN] RLWE public key encryption: https://eprint.iacr.org/2012/230.pdf §3
/// - [CN] 基于公钥的 RLWE 加密: N/A
pub fn encrypt(
    pt: &Plaintext,
    pk: &PublicKey,
    params: &Arc<CkksParams>,
    rng: &mut impl RngCore,
) -> Result<Ciphertext, CkksError> {
    todo!(
        "sample u (ternary), e0, e1 (Gaussian); \
         compute c0 = pk.b*u + e0 + pt.poly, c1 = pk.a*u + e1; \
         all operations in NTT domain at pt.level"
    )
}

/// Encrypt a plaintext using the secret key (symmetric encryption).
///
/// This is faster than public-key encryption and used when the encryptor
/// knows the secret key (e.g., in batch offline processing).
///
/// # Mathematical Specification
/// ```text
/// a ← U(R_Q)    (fresh uniform randomness)
/// e ← DiscreteGaussian(σ)
/// c0 = -a * s + e + pt.poly
/// c1 = a
/// return (c0, c1, level, scale)
/// ```
///
/// # Learning Resources
/// - [EN] RLWE symmetric encryption: https://eprint.iacr.org/2012/230.pdf §3
/// - [CN] 对称加密模式: N/A
pub fn encrypt_symmetric(
    pt: &Plaintext,
    sk: &SecretKey,
    params: &Arc<CkksParams>,
    rng: &mut impl RngCore,
) -> Result<Ciphertext, CkksError> {
    todo!(
        "sample a (uniform), e (Gaussian); \
         compute c0 = -a*s + e + pt.poly, c1 = a; \
         all in NTT domain"
    )
}
