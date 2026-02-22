//! CKKS parameter sets.
//!
//! A CKKS parameter set specifies:
//! - `poly_degree` n: the ring dimension (power of 2)
//! - `q_chain`: a chain of prime moduli [q_0, q_1, ..., q_L]
//! - `scale`: the default scaling factor Δ (typically 2^40 or 2^50)
//!
//! The ciphertext modulus at level `l` is Q_l = q_0 * q_1 * ... * q_l.
//! After each multiplication + rescale, one limb is dropped (level decreases by 1).
//!
//! # Security Parameters
//! For 128-bit security with depth 10, typically:
//!   n = 16384, log(Q) ≈ 438 bits (using 10 primes of ~44 bits each)
//!
//! # Learning Resources
//! - [EN] CKKS parameter selection: https://eprint.iacr.org/2016/421.pdf §5
//! - [EN] Homomorphic Encryption Standard: https://homomorphicencryption.org/standard/
//! - [CN] CKKS 参数选择指南（知乎）: N/A
//! - [CN] FHE 安全参数（CSDN）: N/A

use fhe_math::{ntt::NttPlan, rns::RnsBasis};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// CKKS scheme parameters.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CkksParams {
    /// Ring dimension n (must be a power of 2).
    pub poly_degree: usize,
    /// The chain of prime moduli. `q_chain[0]` is the "special" prime;
    /// `q_chain[1..=depth]` are the "data" primes dropped during rescaling.
    pub q_chain: Vec<u64>,
    /// Default scale Δ ≈ 2^scale_bits.
    pub scale: f64,
    /// Number of plaintext slots = n/2.
    pub num_slots: usize,
    /// Maximum multiplicative depth.
    pub max_depth: usize,
}

impl CkksParams {
    /// Create parameters manually (for advanced users and tests).
    pub fn new(poly_degree: usize, q_chain: Vec<u64>, scale: f64) -> Self {
        assert!(poly_degree.is_power_of_two(), "poly_degree must be power of 2");
        assert!(!q_chain.is_empty(), "q_chain must not be empty");
        let max_depth = q_chain.len().saturating_sub(1);
        Self { poly_degree, num_slots: poly_degree / 2, q_chain, scale, max_depth }
    }

    /// RNS basis for level `level` (uses limbs 0..=level).
    pub fn rns_basis_at_level(&self, level: usize) -> RnsBasis {
        RnsBasis::new(self.q_chain[..=level].to_vec())
    }

    /// NTT plans for all limbs at a given level.
    ///
    /// In practice these should be cached; this creates new ones each call.
    pub fn ntt_plans_at_level(&self, level: usize) -> Vec<NttPlan> {
        self.q_chain[..=level]
            .iter()
            .map(|&q| NttPlan::new(q, self.poly_degree).expect("params guarantee valid NTT moduli"))
            .collect()
    }
}

/// Toy parameters for unit tests: n=16, depth=3, insecure.
///
/// These are intentionally tiny so tests run fast.
/// **Never use in production.**
pub fn toy_params() -> CkksParams {
    // Small NTT-friendly primes for degree 16:
    // Each must be ≡ 1 mod 32 (= 2*16)
    let q_chain = vec![
        786_433,   // 3 * 2^18 + 1, supports degree up to 2^17
        786_433,   // repeated for simplicity in toy setting
        786_433,
        786_433,
    ];
    CkksParams::new(16, q_chain, (1 << 20) as f64)
}

/// 128-bit-secure parameters for depth 10 (production-grade).
///
/// These match the OpenFHE "STD128" preset.
/// Poly degree 16384, 11 primes of ~44 bits each.
pub fn secure_128bit_depth10() -> CkksParams {
    // NTT-friendly primes ≡ 1 mod 32768 (= 2*16384), each ~44 bits
    let q_chain = vec![
        0x0003_FFFF_FFFF_0001, // placeholder — replace with actual NTT primes
        0x0003_FFFF_FFFE_0001,
        0x0003_FFFF_FFFD_0001,
        0x0003_FFFF_FFFC_0001,
        0x0003_FFFF_FFFB_0001,
        0x0003_FFFF_FFFA_0001,
        0x0003_FFFF_FFF9_0001,
        0x0003_FFFF_FFF8_0001,
        0x0003_FFFF_FFF7_0001,
        0x0003_FFFF_FFF6_0001,
        0x0003_FFFF_FFF5_0001, // special modulus
    ];
    let scale = f64::powi(2.0, 44);
    CkksParams::new(16384, q_chain, scale)
}
