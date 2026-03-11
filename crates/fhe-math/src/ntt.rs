//! Negacyclic NTT (Number Theoretic Transform) planning and execution.
//!
//! The NTT is the modular-arithmetic analogue of the DFT, used to speed up
//! polynomial multiplication from O(n²) to O(n log n).
//!
//! # Mathematical Background
//! This module is about the RLWE/CKKS-style transform over `Z_q[x] / (x^n + 1)`,
//! not the textbook cyclic transform over `Z_q[x] / (x^n - 1)`.
//!
//! For a prime `q ≡ 1 (mod 2n)`, choose a primitive `2n`-th root `ψ` with
//! `ψ^n = -1`, and define `ω = ψ^2`. A textbook twisted negacyclic NTT can
//! then be written as:
//! ```text
//! A[k] = Σ_{j=0}^{n-1} a[j] * ψ^j * ω^{jk} mod q
//! ```
//! The wrapped backend `concrete_ntt::prime64::Plan` packages these negacyclic
//! semantics directly.
//!
//! # Learning Resources
//! - [EN] NTT tutorial (CP-Algorithms): https://cp-algorithms.com/algebra/fft.html#number-theoretic-transform
//! - [EN] Cooley-Tukey algorithm: https://en.wikipedia.org/wiki/Cooley%E2%80%93Tukey_FFT_algorithm
//! - [CN] NTT（OI-Wiki）: https://oi-wiki.org/math/poly/ntt/
//! - [CN] NTT 原理与实现（知乎）: N/A

use crate::MathError;

/// A precomputed negacyclic NTT plan for a given (modulus, degree) pair.
///
/// Wraps `concrete_ntt::prime64::Plan`, which is a direct negacyclic NTT plan
/// for `Z_q[x] / (x^n + 1)`.
/// This type caches the twiddle factors so they are computed only once.
pub struct NttPlan {
    /// The prime modulus `q` (must satisfy `q ≡ 1 mod 2n`).
    pub modulus: u64,
    /// The transform length (number of coefficients, must be a power of 2).
    pub degree: usize,
    /// The underlying concrete-ntt plan.
    inner: concrete_ntt::prime64::Plan,
}

impl NttPlan {
    /// Create a new negacyclic NTT plan for the given `(modulus, degree)` pair.
    ///
    /// # Mathematical Specification
    /// ```text
    /// Preconditions:
    ///   - degree must be a power of 2
    ///   - modulus must be prime and satisfy modulus ≡ 1 (mod 2*degree)
    ///   - the backend may impose extra constraints; for the current
    ///     concrete-ntt backend, the modulus must admit a primitive 2*degree-th
    ///     root of unity and very small degrees may be rejected
    /// ```
    ///
    /// # Errors
    /// Returns `MathError::DegreeNotPowerOfTwo` or `MathError::NttUnsupported`.
    ///
    /// # Learning Resources
    /// - [EN] NTT-friendly prime requirements: https://cgyurgyik.github.io/posts/2021/04/brief-introduction-to-ntt/
    /// - [CN] NTT 模数选取（知乎）: N/A
    pub fn new(modulus: u64, degree: usize) -> Result<Self, MathError> {
        if degree == 0 || degree & (degree - 1) != 0 {
            return Err(MathError::DegreeNotPowerOfTwo(degree));
        }
        let inner = concrete_ntt::prime64::Plan::try_new(degree, modulus)
            .ok_or(MathError::NttUnsupported { modulus, degree })?;
        Ok(Self { modulus, degree, inner })
    }

    /// Perform an in-place forward negacyclic NTT on `data`.
    ///
    /// After this call, `data` contains negacyclic NTT coefficients.
    ///
    /// With the current backend, the input is in standard coefficient order and
    /// the output is in the backend's frequency-domain layout (bit-reversed for
    /// `concrete-ntt`).
    ///
    /// Callers should not apply an extra manual twist factor around this call,
    /// because the backend already implements the negacyclic transform itself.
    /// ```text
    /// textbook twisted form:
    /// data[k] = Σ_{j=0}^{n-1} data_in[j] * ψ^j * ω^{jk} mod q
    /// ```
    ///
    /// # Panics
    /// Panics if `data.len() != self.degree`.
    ///
    /// # Learning Resources
    /// - [EN] concrete-ntt docs: https://docs.rs/concrete-ntt
    /// - [EN] Cooley-Tukey butterfly: https://cp-algorithms.com/algebra/fft.html
    /// - [CN] NTT 蝴蝶操作（OI-Wiki）: https://oi-wiki.org/math/poly/ntt/
    pub fn forward(&self, data: &mut [u64]) {
        assert_eq!(data.len(), self.degree, "NTT forward: data length mismatch");
        todo!("call self.inner.fwd(data) — concrete-ntt prime64::Plan does in-place forward negacyclic NTT")
    }

    /// Perform an in-place inverse negacyclic NTT on `data`.
    ///
    /// After this call, `data` contains the original polynomial coefficients
    /// in standard order.
    ///
    /// The input should use the same backend frequency-domain layout produced by
    /// `forward` (bit-reversed for `concrete-ntt`).
    ///
    /// # Panics
    /// Panics if `data.len() != self.degree`.
    ///
    /// # Learning Resources
    /// - [EN] Inverse NTT and the normalization factor: https://cp-algorithms.com/algebra/fft.html
    /// - [CN] INTT 归一化（OI-Wiki）: https://oi-wiki.org/math/poly/ntt/
    pub fn inverse(&self, data: &mut [u64]) {
        assert_eq!(data.len(), self.degree, "NTT inverse: data length mismatch");
        todo!("call self.inner.inv(data) — concrete-ntt prime64::Plan does in-place inverse negacyclic NTT")
    }

    /// Elementwise multiply two NTT-domain vectors in-place: `a[i] = a[i] * b[i] mod q`.
    ///
    /// This is the O(n) "convolution in frequency domain" step.
    ///
    /// Important: both vectors must be in the same backend frequency layout.
    /// Under `concrete-ntt`, that means negacyclic frequency coefficients stored
    /// in bit-reversed order.
    pub fn pointwise_mul(&self, a: &mut [u64], b: &[u64]) {
        assert_eq!(a.len(), b.len());
        assert_eq!(a.len(), self.degree);
        for (ai, bi) in a.iter_mut().zip(b.iter()) {
            *ai = crate::modular::mod_mul(*ai, *bi, self.modulus);
        }
    }
}

/// Compute a primitive `2n`-th root of unity `ψ` in `Z_q`.
///
/// # Mathematical Specification
/// ```text
/// g = primitive root of Z_q^*  (generator of the full group of order q-1)
/// ψ = g^{(q-1)/(2n)} mod q
/// ```
/// Verify: `ψ^n = -1 mod q` (primitive, not just a square root of 1).
///
/// In negacyclic NTT, this is the natural root to talk about. The corresponding
/// cyclic `n`-th root is `ω = ψ^2`.
///
/// # Learning Resources
/// - [EN] Roots of unity in NTT: https://cgyurgyik.github.io/posts/2021/04/brief-introduction-to-ntt/
/// - [CN] NTT 单位根（知乎）: N/A
pub fn primitive_root_of_unity(q: u64, n: usize) -> Result<u64, MathError> {
    use crate::modular::{find_primitive_root, mod_pow};
    if (q - 1) % (2 * n as u64) != 0 {
        return Err(MathError::NttUnsupported { modulus: q, degree: n });
    }
    let g = find_primitive_root(q)?;
    Ok(mod_pow(g, (q - 1) / (2 * n as u64), q))
}

#[cfg(test)]
mod tests {
    use super::*;

    // A small NTT-friendly prime: 7681 = 15 * 2^9 + 1, degree up to 512
    const P_SMALL: u64 = 7681;
    // The standard concrete-ntt prime: 998244353 = 119 * 2^23 + 1
    const P_NTT: u64 = 998_244_353;

    #[test]
    #[ignore = "concrete-ntt Plan::try_new API mismatch — fix when implementing NttPlan"]
    fn test_plan_creation() {
        assert!(NttPlan::new(P_NTT, 8).is_ok());
        assert!(NttPlan::new(P_NTT, 3).is_err()); // not power of 2
    }

    #[test]
    #[ignore = "implement NttPlan::forward and ::inverse first"]
    fn test_ntt_roundtrip() {
        let plan = NttPlan::new(P_NTT, 8).unwrap();
        let original = vec![1u64, 2, 3, 4, 5, 6, 7, 8];
        let mut data = original.clone();
        plan.forward(&mut data);
        plan.inverse(&mut data);
        assert_eq!(data, original, "INTT(NTT(x)) should equal x");
    }

    #[test]
    #[ignore = "implement NttPlan::forward and ::inverse first"]
    fn test_ntt_convolution() {
        // Multiply polynomial [1, 2] * [3, 4] = [3, 10, 8] (mod x^4)
        // Using NTT of size 4
        let plan = NttPlan::new(P_NTT, 4).unwrap();
        let mut a = vec![1u64, 2, 0, 0];
        let mut b = vec![3u64, 4, 0, 0];
        plan.forward(&mut a);
        plan.forward(&mut b);
        plan.pointwise_mul(&mut a, &b);
        plan.inverse(&mut a);
        // [1,2] * [3,4] = 3 + 4x + 6x + 8x^2 = 3 + 10x + 8x^2
        assert_eq!(&a[..3], &[3, 10, 8]);
        assert_eq!(a[3], 0);
    }
}
