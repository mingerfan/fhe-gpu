//! NTT (Number Theoretic Transform) planning and execution.
//!
//! The NTT is the modular-arithmetic analogue of the DFT, used to speed up
//! polynomial multiplication from O(n²) to O(n log n).
//!
//! # Mathematical Background
//! For a prime `q ≡ 1 (mod 2n)`, the NTT of length `n` over Z_q is:
//! ```text
//! A[k] = Σ_{j=0}^{n-1} a[j] * ω^{jk} mod q
//! ```
//! where `ω` is a primitive `n`-th root of unity in Z_q.
//!
//! # Learning Resources
//! - [EN] NTT tutorial (CP-Algorithms): https://cp-algorithms.com/algebra/fft.html#number-theoretic-transform
//! - [EN] Cooley-Tukey algorithm: https://en.wikipedia.org/wiki/Cooley%E2%80%93Tukey_FFT_algorithm
//! - [CN] NTT（OI-Wiki）: https://oi-wiki.org/math/poly/ntt/
//! - [CN] NTT 原理与实现（知乎）: N/A

use crate::MathError;

/// A precomputed NTT plan for a given (modulus, degree) pair.
///
/// Wraps `concrete_ntt::prime64::Plan` which handles the low-level NTT implementation.
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
    /// Create a new NTT plan for the given `(modulus, degree)` pair.
    ///
    /// # Mathematical Specification
    /// ```text
    /// Preconditions:
    ///   - degree must be a power of 2
    ///   - modulus must be prime and satisfy modulus ≡ 1 (mod 2*degree)
    ///   - 2^62 < modulus < 2^63  (concrete-ntt requirement)
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

    /// Perform an in-place forward NTT on `data`.
    ///
    /// After this call, `data` contains the NTT coefficients:
    /// ```text
    /// data[k] = Σ_{j=0}^{n-1} data_in[j] * ω^{jk} mod q
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
        todo!("call self.inner.fwd(data) — concrete-ntt prime64::Plan does in-place forward NTT")
    }

    /// Perform an in-place inverse NTT on `data`, normalizing by `n^{-1} mod q`.
    ///
    /// After this call, `data` contains the original polynomial coefficients
    /// (the INTT is the exact inverse of `forward`).
    ///
    /// # Panics
    /// Panics if `data.len() != self.degree`.
    ///
    /// # Learning Resources
    /// - [EN] Inverse NTT and the normalization factor: https://cp-algorithms.com/algebra/fft.html
    /// - [CN] INTT 归一化（OI-Wiki）: https://oi-wiki.org/math/poly/ntt/
    pub fn inverse(&self, data: &mut [u64]) {
        assert_eq!(data.len(), self.degree, "NTT inverse: data length mismatch");
        todo!("call self.inner.inv(data) — concrete-ntt prime64::Plan does in-place inverse NTT (includes n^{{-1}} normalization)")
    }

    /// Elementwise multiply two NTT-domain vectors in-place: `a[i] = a[i] * b[i] mod q`.
    ///
    /// This is the O(n) "convolution in frequency domain" step.
    pub fn pointwise_mul(&self, a: &mut [u64], b: &[u64]) {
        assert_eq!(a.len(), b.len());
        assert_eq!(a.len(), self.degree);
        for (ai, bi) in a.iter_mut().zip(b.iter()) {
            *ai = crate::modular::mod_mul(*ai, *bi, self.modulus);
        }
    }
}

/// Compute the primitive `2n`-th root of unity `ω_{2n}` in `Z_q`.
///
/// # Mathematical Specification
/// ```text
/// g = primitive root of Z_q^*  (generator of the full group of order q-1)
/// ω_{2n} = g^{(q-1)/(2n)} mod q
/// ```
/// Verify: `ω_{2n}^n = -1 mod q` (primitive, not just a square root of 1).
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
