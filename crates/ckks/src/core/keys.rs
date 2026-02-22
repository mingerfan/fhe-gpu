//! CKKS key types.
//!
//! # Key Hierarchy
//! - `SecretKey` sk: a ternary polynomial s in R = Z[x]/(x^n+1)
//! - `PublicKey` pk = (pk_0, pk_1): an RLWE encryption of 0 under sk
//! - `RelinKey` rlk: enables relinearization after multiplication
//! - `GaloisKey` gk_k: enables rotation by `k` slots (automorphism φ_k)
//!
//! # Mathematical Specification
//! ```text
//! SecretKey:  s ← Ternary distribution (coefficients in {-1, 0, 1})
//!
//! PublicKey:  a ← U(R_Q),  e ← Error distribution
//!             pk = (-a*s + e, a) in R_Q^2
//!
//! RelinKey:   For digit decomposition key switching:
//!             rlk[i] = (-a_i * s + e_i + P^i * s^2, a_i) in R_{P*Q}^2
//! ```
//!
//! # Learning Resources
//! - [EN] RLWE key generation: https://eprint.iacr.org/2012/230.pdf §3
//! - [EN] Key switching in CKKS: https://eprint.iacr.org/2016/421.pdf §3
//! - [CN] RLWE 密钥生成（知乎）: N/A
//! - [CN] 重线性化密钥（CSDN）: N/A

use fhe_math::rns::RnsPoly;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// CKKS secret key: a ternary polynomial in R_Q.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SecretKey {
    /// The secret polynomial s (coefficients in {0, 1, q-1} ≡ {0, 1, -1}).
    pub poly: RnsPoly,
}

/// CKKS public key: an RLWE encryption of 0 under the secret key.
///
/// `pk = (b, a)` where `b = -a*s + e` (a is uniform, e is small).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicKey {
    /// `b = -a*s + e`
    pub b: RnsPoly,
    /// `a` (uniform random)
    pub a: RnsPoly,
}

/// Relinearization key for reducing degree-2 ciphertext back to degree 1.
///
/// Contains the key switching hints for `s^2`.
///
/// # Mathematical Specification
/// ```text
/// Using the "modulus raising" (P-gadget) technique:
/// For each decomposition digit d:
///   rlk[d] = (-a_d * s + e_d + P^d * s^2, a_d) in R_{P*Q}^2
/// ```
///
/// # Learning Resources
/// - [EN] Relinearization key (Brakerski et al.): https://eprint.iacr.org/2011/277.pdf
/// - [CN] 重线性化密钥推导: N/A
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelinKey {
    /// Digit decomposition key switching elements.
    pub digits: Vec<(RnsPoly, RnsPoly)>,
}

/// Galois/rotation key for rotating plaintext slots by a given step.
///
/// `GaloisKey` for automorphism index `k` enables rotation by `k` positions.
/// For CKKS with n slots, the Galois group has generators 5 and -1.
///
/// # Mathematical Specification
/// ```text
/// Rotation by k positions uses the automorphism φ_{5^k}:
///   φ_k(f(x)) = f(x^k) mod (x^n + 1)
///
/// The Galois key is a key switching key from s(x^k) to s(x):
///   gk = (-a * s + e + P * s(x^k), a) in R_{P*Q}^2
/// ```
///
/// # Learning Resources
/// - [EN] Galois automorphisms in CKKS: https://eprint.iacr.org/2016/421.pdf §4
/// - [CN] Galois 自同构与旋转（知乎）: N/A
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GaloisKey {
    /// The Galois automorphism index (5^step mod 2n for rotation by step).
    pub galois_elt: u64,
    /// The key switching pair.
    pub ksk: (RnsPoly, RnsPoly),
}

/// Collection of all keys for a CKKS scheme instance.
#[derive(Clone, Debug)]
pub struct KeySet {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
    pub relin_key: RelinKey,
    /// Map from rotation step to Galois key.
    pub galois_keys: HashMap<i32, GaloisKey>,
}
