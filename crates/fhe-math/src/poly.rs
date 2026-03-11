//! Single-modulus polynomial over Z_q[x] / (x^n + 1).
//!
//! This module represents polynomials with coefficients in Z_q, stored either
//! in coefficient form or NTT (evaluation) form. The quotient ring `Z_q[x]/(x^n+1)`
//! is the negacyclic convolution ring used in RLWE-based cryptography.
//!
//! # Negacyclic NTT
//! The standard NTT computes convolution mod `x^n - 1`. For RLWE we need
//! mod `x^n + 1` (negacyclic). This is achieved by pre-multiplying with
//! "twist" factors before a standard cyclic NTT.
//!
//! Important: the backend currently wrapped by `crate::ntt::NttPlan` is
//! `concrete_ntt::prime64::Plan`, which already implements a direct
//! negacyclic NTT. So the textbook "twist + standard NTT" derivation is the
//! math model, but the concrete call path should not apply an extra twist on
//! top of a direct negacyclic backend.
//!
//! # Learning Resources
//! - [EN] RLWE and the polynomial ring: https://eprint.iacr.org/2012/230.pdf §2
//! - [EN] Negacyclic NTT: https://eprint.iacr.org/2016/504.pdf §2
//! - [CN] 负循环 NTT（知乎）: N/A
//! - [CN] 多项式环基础（OI-Wiki）: https://oi-wiki.org/math/poly/intro/

use crate::{modular::mod_mul, ntt::NttPlan, MathError};
use rand::RngCore;
use serde::{Deserialize, Serialize};

/// Domain of polynomial representation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Domain {
    /// Coefficient form: `a[i]` is the coefficient of `x^i`.
    Coefficient,
    /// Negacyclic NTT/evaluation form.
    ///
    /// Under the current `concrete-ntt` backend this should be understood as
    /// the backend's negacyclic frequency-domain layout, not necessarily as
    /// "natural-order evaluations at points 0..n-1".
    Ntt,
}

/// A polynomial in Z_q[x]/(x^n+1), stored as `n` coefficients mod `q`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Poly {
    /// Coefficients (or NTT values), each in `[0, q)`.
    pub coeffs: Vec<u64>,
    /// The prime modulus `q`.
    pub modulus: u64,
    /// Current domain of `coeffs`.
    pub domain: Domain,
}

impl Poly {
    /// Create a zero polynomial of given degree and modulus.
    ///
    /// # Panics
    /// Panics if `degree` is 0 or not a power of 2.
    pub fn zero(degree: usize, modulus: u64) -> Self {
        assert!(degree > 0 && degree.is_power_of_two(), "degree must be a power of 2");
        Self { coeffs: vec![0u64; degree], modulus, domain: Domain::Coefficient }
    }

    /// Create a polynomial with random uniform coefficients in `[0, q)`.
    ///
    /// # Learning Resources
    /// - [EN] Uniform sampling in RLWE: https://eprint.iacr.org/2012/230.pdf
    /// - [CN] 均匀分布采样: N/A
    pub fn random(degree: usize, modulus: u64, rng: &mut impl RngCore) -> Self {
        todo!("sample each coefficient uniformly from [0, modulus) using rejection sampling")
    }

    /// Create a polynomial with ternary coefficients: each coeff ∈ {-1, 0, 1} mod q.
    ///
    /// Used for secret key generation in RLWE schemes.
    ///
    /// # Mathematical Specification
    /// ```text
    /// Each coefficient independently: -1 w.p. 1/3, 0 w.p. 1/3, 1 w.p. 1/3
    /// -1 is stored as q - 1
    /// ```
    ///
    /// # Learning Resources
    /// - [EN] Ternary secret keys in RLWE: https://eprint.iacr.org/2012/230.pdf §3
    /// - [CN] 三元分布密钥: N/A
    pub fn ternary(degree: usize, modulus: u64, rng: &mut impl RngCore) -> Self {
        todo!("sample each coeff as 0, 1, or q-1 with equal probability 1/3")
    }

    /// Number of coefficients (= polynomial degree as a ring element).
    pub fn len(&self) -> usize {
        self.coeffs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.coeffs.is_empty()
    }

    /// Add two polynomials coefficient-wise mod q.
    ///
    /// # Panics
    /// Panics if the polynomials have different lengths or moduli, or are in different domains.
    pub fn add(&self, other: &Self) -> Self {
        todo!("check preconditions, then coeffwise (a + b) mod q")
    }

    /// Subtract two polynomials coefficient-wise mod q.
    pub fn sub(&self, other: &Self) -> Self {
        todo!("check preconditions, then coeffwise (a - b + q) mod q")
    }

    /// Negate a polynomial: `-a[i] mod q = q - a[i]` (for nonzero coefficients).
    pub fn negate(&self) -> Self {
        todo!("coeffwise: if c == 0 then 0 else q - c")
    }

    /// Multiply two polynomials in the negacyclic ring Z_q[x]/(x^n+1).
    ///
    /// If both polynomials are in NTT domain, this is a pointwise multiply.
    /// If in coefficient domain, it converts to NTT, multiplies, then converts back.
    ///
    /// # Mathematical Specification
    /// ```text
    /// Product in Z_q[x]/(x^n+1):
    ///   c = a * b mod (x^n + 1, q)
    /// Negacyclic wrap: coefficient of x^k (k >= n) wraps with sign flip:
    ///   c[k mod n] -= contribution  (because x^n ≡ -1)
    /// ```
    ///
    /// # Learning Resources
    /// - [EN] Negacyclic convolution via twisted NTT: https://eprint.iacr.org/2016/504.pdf
    /// - [CN] 负循环卷积（知乎）: N/A
    pub fn mul(&self, other: &Self, plan: &NttPlan) -> Result<Self, MathError> {
        todo!("if both in NTT domain: pointwise mul; else convert to NTT, pointwise mul, convert back")
    }

    /// Convert this polynomial to NTT domain in-place.
    ///
    /// # Mathematical Specification
    /// ```text
    /// Textbook derivation:
    /// before forward NTT, multiply coeff[i] by ψ^i where ψ is a primitive
    /// 2n-th root of unity in Z_q, then apply a standard length-n NTT.
    ///
    /// Backend note:
    /// `NttPlan` currently wraps a direct negacyclic NTT implementation, so if
    /// we keep using that backend this method should *not* apply an additional
    /// manual twist.
    /// ```
    ///
    /// # Learning Resources
    /// - [EN] Twisted NTT for negacyclic convolution: https://eprint.iacr.org/2016/504.pdf §2
    /// - [CN] 扭转 NTT 推导: N/A
    pub fn ntt_forward(&mut self, plan: &NttPlan) -> Result<(), MathError> {
        if self.domain == Domain::Ntt {
            return Ok(());
        }
        todo!("for a direct negacyclic backend, call plan.forward() without an extra twist; if switching to a cyclic backend, apply ψ^i first")
    }

    /// Convert this polynomial from NTT domain back to coefficient form.
    pub fn ntt_inverse(&mut self, plan: &NttPlan) -> Result<(), MathError> {
        if self.domain == Domain::Coefficient {
            return Ok(());
        }
        todo!("for a direct negacyclic backend, call plan.inverse() without an extra untwist; if switching to a cyclic backend, multiply by ψ^(-i) afterwards")
    }
}

impl std::fmt::Display for Poly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let terms: Vec<String> = self.coeffs.iter().enumerate()
            .filter(|(_, &c)| c != 0)
            .map(|(i, &c)| if i == 0 { format!("{c}") } else { format!("{c}x^{i}") })
            .collect();
        if terms.is_empty() { write!(f, "0") } else { write!(f, "{}", terms.join(" + ")) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const P: u64 = 998_244_353;
    const N: usize = 8;

    #[test]
    fn test_zero_poly() {
        let p = Poly::zero(N, P);
        assert_eq!(p.coeffs, vec![0u64; N]);
        assert_eq!(p.domain, Domain::Coefficient);
    }

    #[test]
    #[ignore = "implement Poly::add first"]
    fn test_poly_add() {
        let a = Poly { coeffs: vec![1, 2, 3, 0, 0, 0, 0, 0], modulus: P, domain: Domain::Coefficient };
        let b = Poly { coeffs: vec![P - 1, 0, 1, 0, 0, 0, 0, 0], modulus: P, domain: Domain::Coefficient };
        let c = a.add(&b);
        assert_eq!(c.coeffs[0], 0); // 1 + (P-1) = P ≡ 0
        assert_eq!(c.coeffs[1], 2);
        assert_eq!(c.coeffs[2], 4);
    }

    #[test]
    #[ignore = "implement Poly::ntt_forward, ntt_inverse, mul first"]
    fn test_poly_mul_schoolbook_vs_ntt() {
        use crate::modular::mod_mul;
        let plan = NttPlan::new(P, N).unwrap();
        // a = [1, 1, 0, ...], b = [1, 1, 0, ...] -> a*b = [1, 2, 1, 0, ...] mod (x^8+1)
        let a = Poly { coeffs: vec![1, 1, 0, 0, 0, 0, 0, 0], modulus: P, domain: Domain::Coefficient };
        let b = Poly { coeffs: vec![1, 1, 0, 0, 0, 0, 0, 0], modulus: P, domain: Domain::Coefficient };
        let c = a.mul(&b, &plan).unwrap();
        assert_eq!(c.coeffs[0], 1);
        assert_eq!(c.coeffs[1], 2);
        assert_eq!(c.coeffs[2], 1);
    }
}
