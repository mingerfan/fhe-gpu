//! RNS (Residue Number System) polynomial representation.
//!
//! An RNS polynomial is a polynomial in Z_Q[x]/(x^n+1) represented as multiple
//! polynomials, one per small prime modulus q_i, where Q = q_0 * q_1 * ... * q_{L-1}.
//!
//! This is the core data representation for CKKS (and most modern FHE schemes).
//! Each "limb" is a `Poly` modulo a small NTT-friendly prime.
//!
//! # Why RNS?
//! Q can be hundreds of bits (e.g., 1024 bits for depth-20 CKKS), too large for
//! native arithmetic. By splitting Q into ~30 small primes q_i (each ~60 bits),
//! all arithmetic stays in 64-bit words and the CRT guarantees correctness.
//!
//! # Learning Resources
//! - [EN] RNS in FHE (Bajard et al.): https://eprint.iacr.org/2016/510.pdf
//! - [EN] CKKS with full RNS: https://eprint.iacr.org/2018/931.pdf
//! - [CN] RNS 表示与 CRT（知乎）: N/A
//! - [CN] 剩余数系统（OI-Wiki）: https://oi-wiki.org/math/crt/

use crate::{modular::mod_mul, ntt::NttPlan, poly::Poly, MathError};
use serde::{Deserialize, Serialize};

/// An RNS basis: a sequence of distinct small prime moduli.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RnsBasis {
    /// The prime moduli q_0, q_1, ..., q_{L-1}.
    pub moduli: Vec<u64>,
}

impl RnsBasis {
    /// Create a new RNS basis from the given moduli (all must be distinct primes).
    pub fn new(moduli: Vec<u64>) -> Self {
        Self { moduli }
    }

    pub fn len(&self) -> usize {
        self.moduli.len()
    }

    pub fn is_empty(&self) -> bool {
        self.moduli.is_empty()
    }
}

/// A polynomial in Z_Q[x]/(x^n+1) represented in RNS form.
///
/// `limbs[i]` is the polynomial reduced modulo `basis.moduli[i]`.
/// All limbs have the same degree and are in the same domain (coefficient or NTT).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RnsPoly {
    /// One `Poly` per RNS limb.
    pub limbs: Vec<Poly>,
    /// The shared RNS basis.
    pub basis: RnsBasis,
}

impl RnsPoly {
    /// Create a zero RNS polynomial.
    pub fn zero(degree: usize, basis: RnsBasis) -> Self {
        let limbs = basis.moduli.iter().map(|&q| Poly::zero(degree, q)).collect();
        Self { limbs, basis }
    }

    /// Degree of the polynomial (number of coefficients per limb).
    pub fn degree(&self) -> usize {
        self.limbs.first().map(|p| p.len()).unwrap_or(0)
    }

    /// Number of RNS limbs (= number of prime moduli in the basis).
    pub fn num_limbs(&self) -> usize {
        self.limbs.len()
    }

    /// Construct an RNS polynomial from a single-modulus polynomial by reducing mod each q_i.
    ///
    /// # Mathematical Specification
    /// ```text
    /// For each limb i:
    ///   limbs[i].coeffs[j] = poly.coeffs[j] mod q_i
    /// ```
    /// Note: `poly.coeffs[j]` may be large (arbitrary precision); this function
    /// currently takes a `&[u64]` for simplicity, assuming coefficients fit in u64.
    ///
    /// # Learning Resources
    /// - [EN] RNS conversion: https://eprint.iacr.org/2016/510.pdf §2
    /// - [CN] RNS 正向变换: N/A
    pub fn from_coeffs(coeffs: &[u64], basis: RnsBasis) -> Self {
        todo!("for each limb i: reduce each coefficient mod basis.moduli[i]")
    }

    /// Add two RNS polynomials limb-wise.
    ///
    /// # Panics
    /// Panics if the bases or degrees don't match.
    pub fn add(&self, other: &Self) -> Self {
        todo!("check basis equality, then zip limbs and call Poly::add for each")
    }

    /// Subtract two RNS polynomials limb-wise.
    pub fn sub(&self, other: &Self) -> Self {
        todo!("check basis equality, then zip limbs and call Poly::sub for each")
    }

    /// Multiply two RNS polynomials limb-wise using NTT.
    ///
    /// # Mathematical Specification
    /// ```text
    /// For each limb i:
    ///   result.limbs[i] = self.limbs[i] * other.limbs[i] mod (x^n+1, q_i)
    /// ```
    /// This is elementwise — not the full CRT reconstruction — because we stay in RNS form.
    ///
    /// # Learning Resources
    /// - [EN] RNS polynomial multiplication: https://eprint.iacr.org/2018/931.pdf §2
    /// - [CN] RNS 多项式乘法: N/A
    pub fn mul(&self, other: &Self, plans: &[NttPlan]) -> Result<Self, MathError> {
        todo!("zip self.limbs, other.limbs, plans; for each triple call Poly::mul")
    }

    /// Scalar-multiply: multiply every coefficient by `scalar` (a single u64).
    ///
    /// # Mathematical Specification
    /// ```text
    /// result.limbs[i].coeffs[j] = (self.limbs[i].coeffs[j] * scalar) mod q_i
    /// ```
    pub fn scalar_mul(&self, scalar: u64) -> Self {
        todo!("for each limb i: coeffwise (c * scalar) mod q_i")
    }

    /// Convert all limbs to NTT domain in-place.
    pub fn ntt_forward(&mut self, plans: &[NttPlan]) -> Result<(), MathError> {
        todo!("zip self.limbs with plans, call Poly::ntt_forward on each")
    }

    /// Convert all limbs from NTT domain back to coefficient form.
    pub fn ntt_inverse(&mut self, plans: &[NttPlan]) -> Result<(), MathError> {
        todo!("zip self.limbs with plans, call Poly::ntt_inverse on each")
    }

    /// ModDown (rescale): drop the last limb and divide coefficients by q_{L-1}.
    ///
    /// This is the core operation for CKKS rescaling (noise management after multiplication).
    ///
    /// # Mathematical Specification
    /// ```text
    /// Given poly at level L (with L limbs), produce poly at level L-1:
    ///
    /// For each remaining limb i (0 ≤ i < L-1):
    ///   For each coefficient j:
    ///     delta_j = last_limb.coeffs[j]   (the value mod q_{L-1})
    ///     // Lift delta to a centered representative in (-q_{L-1}/2, q_{L-1}/2]
    ///     // Then subtract delta from limb[i].coeffs[j], divide by q_{L-1} mod q_i
    ///     result.limbs[i].coeffs[j] = (limbs[i].coeffs[j] - delta_j) * q_{L-1}^{-1} mod q_i
    /// ```
    ///
    /// # Learning Resources
    /// - [EN] CKKS rescaling (Cheon et al. 2017): https://eprint.iacr.org/2016/421.pdf §3
    /// - [EN] Full RNS CKKS ModDown: https://eprint.iacr.org/2018/931.pdf §2.4
    /// - [CN] CKKS Rescale 操作解析（知乎）: N/A
    /// - [CN] ModDown 推导: N/A
    pub fn mod_down(&self) -> Result<Self, MathError> {
        todo!("drop last limb, for each remaining limb i: subtract then multiply by q_last^{{-1}} mod q_i")
    }

    /// CRT reconstruction: recover a big-integer polynomial from RNS form.
    ///
    /// This is the Garner algorithm for CRT reconstruction. Used primarily
    /// for printing/debugging, not in hot paths.
    ///
    /// # Mathematical Specification
    /// ```text
    /// Given x[i] = a mod q_i for i=0..L-1, recover a mod Q = ∏q_i.
    ///
    /// Garner's method (mixed-radix):
    ///   a = x[0] + q_0 * (x[1]' + q_1 * (x[2]'' + ...))
    /// where x[i]' are the mixed-radix digits computed iteratively.
    /// ```
    ///
    /// # Learning Resources
    /// - [EN] Garner's algorithm: https://cp-algorithms.com/algebra/chinese-remainder-theorem.html
    /// - [CN] 中国剩余定理与 Garner 算法（OI-Wiki）: https://oi-wiki.org/math/crt/
    pub fn crt_reconstruct(&self) -> Vec<u128> {
        todo!("Garner's algorithm: compute mixed-radix coefficients, reconstruct as big integers (use u128 for small examples)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Two small NTT-friendly primes
    const Q0: u64 = 998_244_353; // 119 * 2^23 + 1
    const Q1: u64 = 985_661_441; // 235 * 2^22 + 1

    fn two_limb_basis() -> RnsBasis {
        RnsBasis::new(vec![Q0, Q1])
    }

    #[test]
    fn test_zero_rns() {
        let basis = two_limb_basis();
        let p = RnsPoly::zero(8, basis);
        assert_eq!(p.num_limbs(), 2);
        assert!(p.limbs.iter().all(|l| l.coeffs.iter().all(|&c| c == 0)));
    }

    #[test]
    #[ignore = "implement RnsPoly::from_coeffs first"]
    fn test_from_coeffs_roundtrip() {
        let basis = two_limb_basis();
        let coeffs = vec![1u64, 2, 3, 4, 5, 6, 7, 8];
        let rns = RnsPoly::from_coeffs(&coeffs, basis);
        // Check that each limb has correct reduction
        assert_eq!(rns.limbs[0].coeffs[0] % Q0, 1);
        assert_eq!(rns.limbs[1].coeffs[0] % Q1, 1);
    }

    #[test]
    #[ignore = "implement RnsPoly::add first"]
    fn test_rns_add() {
        let basis = two_limb_basis();
        let a = RnsPoly::from_coeffs(&[1, 2, 3, 4, 5, 6, 7, 8], basis.clone());
        let b = RnsPoly::from_coeffs(&[8, 7, 6, 5, 4, 3, 2, 1], basis);
        let c = a.add(&b);
        // Each coefficient should be 9
        for limb in &c.limbs {
            assert!(limb.coeffs.iter().all(|&x| x == 9), "expected all 9s");
        }
    }
}
