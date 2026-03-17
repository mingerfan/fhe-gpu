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
//!
//! This file intentionally exposes a learning-oriented self-written framework:
//! - forward transform: DIF
//! - inverse transform: DIT
//! - bit reversal: separate utility, only apply when a caller truly needs it
//!
//! `reference::ConcreteNttRefPlan` keeps the `concrete-ntt` backend available
//! as a correctness oracle.
//!
//! # Learning Resources
//! - [EN] NTT tutorial (CP-Algorithms): https://cp-algorithms.com/algebra/fft.html#number-theoretic-transform
//! - [EN] Cooley-Tukey algorithm: https://en.wikipedia.org/wiki/Cooley%E2%80%93Tukey_FFT_algorithm
//! - [CN] NTT（OI-Wiki）: https://oi-wiki.org/math/poly/ntt/
//! - [CN] NTT 原理与实现（知乎）: N/A

use crate::{
    MathError, modular::{mod_add, mod_inv, mod_mul, mod_pow, mod_sub}
};

/// Logical ordering of samples in the transform domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpectralLayout {
    /// Indices match the mathematical frequency order directly.
    Natural,
    /// Indices are stored in bit-reversed order.
    BitReversed,
}

/// Precomputed twiddles for one iterative butterfly stage.
#[derive(Clone, Debug)]
pub struct StageTwiddles {
    /// Butterfly span at this stage.
    pub len: usize,
    /// Twiddle powers used inside each butterfly block.
    pub twiddles: Vec<u64>,
}

/// A precomputed negacyclic NTT plan for a given `(modulus, degree)` pair.
///
/// This is the crate's main self-written framework. The intended execution
/// strategy is:
/// - forward: pre-twist then run cyclic DIF, yielding bit-reversed spectral data
/// - inverse: consume bit-reversed spectral data with cyclic DIT, then scale by
///   `n^{-1}` and post-untwist back to coefficient form
pub struct NttPlan {
    /// The prime modulus `q` (must satisfy `q ≡ 1 mod 2n`).
    pub modulus: u64,
    /// The transform length (number of coefficients, must be a power of 2).
    pub degree: usize,
    /// Primitive `n`-th root `ω = ψ²`.
    pub root: u64,
    /// Inverse of `ω`.
    pub root_inv: u64,
    /// Inverse of `degree` modulo `q`.
    pub inv_degree: u64,
    /// Powers `ψ^j` for the forward twist.
    pub psi_powers: Vec<u64>,
    /// Powers `ψ^{-j}` for the inverse untwist.
    pub psi_inv_powers: Vec<u64>,
    /// DIF stage tables for forward NTT.
    pub dif_stages: Vec<StageTwiddles>,
    /// DIT stage tables for inverse NTT.
    pub dit_stages: Vec<StageTwiddles>,
}

impl NttPlan {
    /// Create a new negacyclic NTT plan for the given `(modulus, degree)` pair.
    ///
    /// # Mathematical Specification
    /// ```text
    /// Preconditions:
    ///   - degree must be a power of 2
    ///   - modulus must admit a primitive 2*degree-th root of unity
    ///   - for the intended use in RLWE/CKKS, modulus is a prime
    /// ```
    pub fn new(modulus: u64, degree: usize) -> Result<Self, MathError> {
        if degree == 0 || degree & (degree - 1) != 0 {
            return Err(MathError::DegreeNotPowerOfTwo(degree));
        }

        let psi = primitive_root_of_unity(modulus, degree)?;
        let psi_inv = mod_inv(psi, modulus);
        let root = mod_mul(psi, psi, modulus);
        let root_inv = mod_inv(root, modulus);
        let inv_degree = mod_inv(degree as u64, modulus);

        Ok(Self {
            modulus,
            degree,
            root,
            root_inv,
            inv_degree,
            psi_powers: powers(psi, degree, modulus),
            psi_inv_powers: powers(psi_inv, degree, modulus),
            dif_stages: build_dif_stage_twiddles(root, degree, modulus),
            dit_stages: build_dit_stage_twiddles(root_inv, degree, modulus),
        })
    }

    /// The output layout produced by the planned forward transform.
    pub const fn forward_output_spectral_layout(&self) -> SpectralLayout {
        SpectralLayout::BitReversed
    }

    /// The input layout expected by the planned inverse transform.
    pub const fn inverse_input_spectral_layout(&self) -> SpectralLayout {
        SpectralLayout::BitReversed
    }

    /// Perform an in-place forward negacyclic NTT on `data`.
    ///
    /// Intended flow:
    /// 1. multiply by `ψ^j`
    /// 2. run iterative cyclic DIF
    /// 3. leave the output in bit-reversed order
    pub fn forward_to_bitrev(&self, data: &mut [u64]) {
        assert_eq!(
            data.len(),
            self.degree,
            "NTT forward_to_bitrev: data length mismatch"
        );
        self.apply_forward_twist(data);
        self.forward_core_dif(data);
    }

    /// Perform an in-place inverse negacyclic NTT on `data`.
    ///
    /// Intended flow:
    /// 1. consume bit-reversed spectral input
    /// 2. run iterative cyclic DIT
    /// 3. multiply by `n^{-1}`
    /// 4. multiply by `ψ^{-j}`
    pub fn inverse_from_bitrev(&self, data: &mut [u64]) {
        assert_eq!(
            data.len(),
            self.degree,
            "NTT inverse_from_bitrev: data length mismatch"
        );
        self.inverse_core_dit(data);
        self.apply_inverse_normalization(data);
        self.apply_inverse_untwist(data);
    }

    /// Elementwise multiply two NTT-domain vectors in-place: `a[i] = a[i] * b[i] mod q`.
    ///
    /// The two inputs must use the same spectral layout.
    pub fn pointwise_mul(&self, a: &mut [u64], b: &[u64]) {
        assert_eq!(a.len(), b.len());
        assert_eq!(a.len(), self.degree);
        for (ai, bi) in a.iter_mut().zip(b.iter()) {
            *ai = mod_mul(*ai, *bi, self.modulus);
        }
    }

    /// Reorder a power-of-two vector in-place with the usual bit-reversal permutation.
    ///
    /// This is intentionally separate from `forward`/`inverse` so callers can
    /// avoid paying for it on hot paths that naturally compose DIF and DIT.
    pub fn bit_reverse_in_place(data: &mut [u64]) {
        bit_reverse_permute(data);
    }

    fn apply_forward_twist(&self, data: &mut [u64]) {
        for (value, psi_pow) in data.iter_mut().zip(&self.psi_powers) {
            *value = mod_mul(*value, *psi_pow, self.modulus);
        }
    }

    fn apply_inverse_normalization(&self, data: &mut [u64]) {
        for value in data.iter_mut() {
            *value = mod_mul(*value, self.inv_degree, self.modulus);
        }
    }

    fn apply_inverse_untwist(&self, data: &mut [u64]) {
        for (value, psi_inv_pow) in data.iter_mut().zip(&self.psi_inv_powers) {
            *value = mod_mul(*value, *psi_inv_pow, self.modulus);
        }
    }

    fn forward_core_dif(&self, data: &mut [u64]) {
        // let root = self.root;
        // let degree = self.degree;
        // let psi_powers = &self.psi_powers;
        let dif_stages = &self.dif_stages;
        let layer_num = dif_stages.len();
        let data_len = data.len();
        let data_len_bits = data_len.trailing_zeros() as usize; // 2^n通过这个方法快速计算n
        let mut data_len_mut = data_len;
        let mut data_len_bits_mut = data_len_bits;
        assert_eq!(layer_num, data_len_bits);
        for layer in 0..layer_num {
            let groups_num = 1 << (data_len_bits - data_len_bits_mut);
            let cur_stage = &dif_stages[layer];
            let stride = data_len_mut >> 1;
            for group in 0..groups_num {
                let base = group << data_len_bits_mut; // group * 2^data_len_bits_mut
                for i in 0..stride {
                    let cur = data[base + i];
                    let cur_stride = data[base + i + stride];
                    data[base + i] = mod_add(cur, cur_stride, self.modulus);
                    let sub_ = mod_sub(cur, cur_stride, self.modulus);
                    data[base + i + stride] = mod_mul(sub_, cur_stage.twiddles[i], self.modulus);
                }
            }
            data_len_mut >>= 1;
            data_len_bits_mut -= 1;
        }
        // todo!("implement iterative cyclic DIF using self.dif_stages; leave output bit-reversed")
    }

    fn inverse_core_dit(&self, data: &mut [u64]) {
        let dit_stages = &self.dit_stages;
        let layer_num = dit_stages.len();
        let data_len = data.len();
        let data_len_bits = data_len.trailing_zeros() as usize;
        let mut group_len = 2;
        let mut group_len_bit = 1;
        assert_eq!(layer_num, data_len_bits);
        for layer in 0..layer_num {
            let group_num = data_len >> group_len_bit;
            let stride = group_len >> 1;           
            let cur_stage = &dit_stages[layer];
            for group in 0..group_num {
                let base = group << group_len_bit;
                for i in 0..stride {
                    let cur = data[base + i];
                    let cur_stride = data[base + i + stride];
                    let mul_ = mod_mul(cur_stride, cur_stage.twiddles[i], self.modulus);
                    data[base + i] = mod_add(cur, mul_, self.modulus);
                    data[base + i + stride] = mod_sub(cur, mul_, self.modulus);
                }
            }
            group_len <<= 1;
            group_len_bit += 1;
        }
        // todo!("implement iterative cyclic DIT using self.dit_stages; expect input bit-reversed")
    }
}

/// `concrete-ntt` wrappers used as a reference implementation.
pub mod reference {
    use super::*;

    /// A compatibility wrapper around `concrete_ntt::prime64::Plan`.
    ///
    /// This is intentionally not the crate's main implementation. It exists so
    /// tests and diffs can compare the self-written `NttPlan` against an
    /// external implementation.
    pub struct ConcreteNttRefPlan {
        pub modulus: u64,
        pub degree: usize,
        inv_degree: u64,
        inner: concrete_ntt::prime64::Plan,
    }

    impl ConcreteNttRefPlan {
        pub fn new(modulus: u64, degree: usize) -> Result<Self, MathError> {
            if degree == 0 || degree & (degree - 1) != 0 {
                return Err(MathError::DegreeNotPowerOfTwo(degree));
            }

            let inner = concrete_ntt::prime64::Plan::try_new(degree, modulus)
                .ok_or(MathError::NttUnsupported { modulus, degree })?;

            Ok(Self {
                modulus,
                degree,
                inv_degree: mod_inv(degree as u64, modulus),
                inner,
            })
        }

        /// Forward transform in the backend's native frequency layout.
        pub fn forward(&self, data: &mut [u64]) {
            assert_eq!(
                data.len(),
                self.degree,
                "NTT ref forward: data length mismatch"
            );
            self.inner.fwd(data);
        }

        /// Inverse transform with normalization so that `inverse(forward(x)) == x`.
        pub fn inverse(&self, data: &mut [u64]) {
            assert_eq!(
                data.len(),
                self.degree,
                "NTT ref inverse: data length mismatch"
            );
            self.inner.inv(data);
            for value in data.iter_mut() {
                *value = mod_mul(*value, self.inv_degree, self.modulus);
            }
        }

        pub fn pointwise_mul(&self, a: &mut [u64], b: &[u64]) {
            assert_eq!(a.len(), b.len());
            assert_eq!(a.len(), self.degree);
            for (ai, bi) in a.iter_mut().zip(b.iter()) {
                *ai = mod_mul(*ai, *bi, self.modulus);
            }
        }
    }
}

/// Compute a primitive `2n`-th root of unity `ψ` in `Z_q`.
///
/// Uses the same Tonelli-Shanks-based approach as `concrete-ntt`: start from
/// `q − 1 ≡ −1` (a primitive 2nd root of unity) and take repeated modular
/// square roots to reach a primitive `2n`-th root.
///
/// # Mathematical Specification
/// ```text
/// ψ^(2n) = 1 mod q
/// ψ^n    = -1 mod q
/// ```
pub fn primitive_root_of_unity(q: u64, n: usize) -> Result<u64, MathError> {
    let two_n = 2u64
        .checked_mul(n as u64)
        .ok_or(MathError::NttUnsupported {
            modulus: q,
            degree: n,
        })?;

    if q < 3 || (q - 1) % two_n != 0 {
        return Err(MathError::NttUnsupported {
            modulus: q,
            degree: n,
        });
    }

    // Decompose q-1 = odd_part * 2^two_adicity
    let mut odd_part = q - 1;
    let mut two_adicity = 0u64;
    while odd_part % 2 == 0 {
        odd_part /= 2;
        two_adicity += 1;
    }

    // Find smallest quadratic non-residue mod q
    let z = {
        let exp = (q - 1) / 2;
        (2..q)
            .find(|&c| mod_pow(c, exp, q) == q - 1)
            .ok_or(MathError::NttUnsupported {
                modulus: q,
                degree: n,
            })?
    };

    // Start from -1 (primitive 2nd root of unity) and take log2(2n)-1
    // Tonelli-Shanks square roots to reach a primitive 2n-th root.
    let num_iters = two_n.trailing_zeros() - 1;
    let mut root = q - 1;
    for _ in 0..num_iters {
        root = tonelli_shanks_sqrt(q, odd_part, two_adicity, z, root).ok_or(
            MathError::NttUnsupported {
                modulus: q,
                degree: n,
            },
        )?;
    }

    // Verify the result
    debug_assert_eq!(mod_pow(root, two_n, q), 1);
    debug_assert_eq!(mod_pow(root, n as u64, q), q - 1);

    Ok(root)
}

/// Tonelli-Shanks modular square root.
///
/// Given prime `p` with `p-1 = odd_part * 2^two_adicity`, a quadratic
/// non-residue `z`, and input `n`, returns `sqrt(n) mod p`.
///
/// # Learning Resources
/// - [EN] Tonelli-Shanks (Wikipedia): https://en.wikipedia.org/wiki/Tonelli%E2%80%93Shanks_algorithm
/// - [CN] Tonelli-Shanks（OI-Wiki）: https://oi-wiki.org/math/number-theory/quad-residue/
fn tonelli_shanks_sqrt(
    p: u64,
    odd_part: u64,
    two_adicity: u64,
    z: u64,
    n: u64,
) -> Option<u64> {
    let mut m = two_adicity;
    let mut c = mod_pow(z, odd_part, p);
    let mut t = mod_pow(n, odd_part, p);
    let mut r = mod_pow(n, (odd_part + 1) / 2, p);

    loop {
        if t == 0 {
            return Some(0);
        }
        if t == 1 {
            return Some(r);
        }

        // Find smallest i such that t^(2^i) = 1
        let mut i = 0u64;
        let mut t_pow = t;
        while i < m {
            t_pow = mod_mul(t_pow, t_pow, p);
            i += 1;
            if t_pow == 1 {
                break;
            }
        }
        if i == m {
            return None;
        }

        let b = mod_pow(c, 1u64 << (m - i - 1), p);
        m = i;
        c = mod_mul(b, b, p);
        t = mod_mul(t, c, p);
        r = mod_mul(r, b, p);
    }
}

fn powers(base: u64, len: usize, modulus: u64) -> Vec<u64> {
    let mut out = Vec::with_capacity(len);
    let mut current = 1u64;
    for _ in 0..len {
        out.push(current);
        current = mod_mul(current, base, modulus);
    }
    out
}

fn build_dif_stage_twiddles(root: u64, degree: usize, modulus: u64) -> Vec<StageTwiddles> {
    let mut stages = Vec::new();
    let mut len = degree;
    while len > 1 {
        stages.push(StageTwiddles {
            len,
            twiddles: stage_twiddles(root, degree, len, modulus),
        });
        len >>= 1;
    }
    stages
}

fn build_dit_stage_twiddles(root_inv: u64, degree: usize, modulus: u64) -> Vec<StageTwiddles> {
    let mut stages = Vec::new();
    let mut len = 2;
    while len <= degree {
        stages.push(StageTwiddles {
            len,
            twiddles: stage_twiddles(root_inv, degree, len, modulus),
        });
        len <<= 1;
    }
    stages
}

fn stage_twiddles(root: u64, degree: usize, len: usize, modulus: u64) -> Vec<u64> {
    let half = len / 2;
    let stride = degree / len;
    (0..half)
        .map(|j| mod_pow(root, (j * stride) as u64, modulus))
        .collect()
}

fn bit_reverse_permute(data: &mut [u64]) {
    if data.len() <= 1 {
        return;
    }
    assert!(
        data.len().is_power_of_two(),
        "bit reversal requires a power-of-two length"
    );

    let nbits = data.len().trailing_zeros();
    for i in 0..data.len() {
        let j = i.reverse_bits() >> (usize::BITS - nbits);
        if i < j {
            data.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ntt::reference::ConcreteNttRefPlan;

    const P_SMALL: u64 = 7681;
    const P_NTT: u64 = 998_244_353;

    fn assert_forward_matches_reference(input: &[u64]) {
        let plan = NttPlan::new(P_NTT, input.len()).unwrap();
        let reference = ConcreteNttRefPlan::new(P_NTT, input.len()).unwrap();

        let mut self_data = input.to_vec();
        let mut ref_data = input.to_vec();

        plan.forward_to_bitrev(&mut self_data);
        reference.forward(&mut ref_data);

        assert_eq!(self_data, ref_data);
    }

    fn assert_roundtrip(input: &[u64]) {
        let plan = NttPlan::new(P_NTT, input.len()).unwrap();
        let mut data = input.to_vec();
        plan.forward_to_bitrev(&mut data);
        plan.inverse_from_bitrev(&mut data);
        assert_eq!(data, input, "INTT(NTT(x)) should equal x");
    }

    fn assert_inverse_matches_reference(input: &[u64]) {
        let plan = NttPlan::new(P_NTT, input.len()).unwrap();
        let reference = ConcreteNttRefPlan::new(P_NTT, input.len()).unwrap();

        let mut spectral = input.to_vec();
        reference.forward(&mut spectral);

        let mut self_data = spectral.clone();
        let mut ref_data = spectral;

        plan.inverse_from_bitrev(&mut self_data);
        reference.inverse(&mut ref_data);

        assert_eq!(self_data, ref_data);
        assert_eq!(self_data, input);
    }

    fn assert_core_roundtrip(input: &[u64]) {
        let plan = NttPlan::new(P_NTT, input.len()).unwrap();
        let expected = input
            .iter()
            .map(|value| mod_mul(*value, input.len() as u64, P_NTT))
            .collect::<Vec<_>>();

        let mut data = input.to_vec();
        plan.forward_core_dif(&mut data);
        plan.inverse_core_dit(&mut data);

        assert_eq!(
            data, expected,
            "inverse_core_dit should undo forward_core_dif up to the global n factor"
        );
    }

    fn schoolbook_negacyclic_mul(lhs: &[u64], rhs: &[u64], modulus: u64) -> Vec<u64> {
        assert_eq!(lhs.len(), rhs.len());
        let n = lhs.len();
        let mut out = vec![0u64; n];

        for (i, &a) in lhs.iter().enumerate() {
            for (j, &b) in rhs.iter().enumerate() {
                let prod = mod_mul(a, b, modulus);
                let idx = i + j;
                if idx < n {
                    out[idx] = mod_add(out[idx], prod, modulus);
                } else {
                    out[idx - n] = mod_sub(out[idx - n], prod, modulus);
                }
            }
        }

        out
    }

    fn patterned_input(len: usize) -> Vec<u64> {
        (0..len)
            .map(|i| {
                let x = i as u64;
                (x * x * 17 + x * 29 + 11) % P_NTT
            })
            .collect()
    }

    #[test]
    fn test_plan_creation() {
        let plan = NttPlan::new(P_NTT, 8).unwrap();
        assert_eq!(
            plan.forward_output_spectral_layout(),
            SpectralLayout::BitReversed
        );
        assert_eq!(
            plan.inverse_input_spectral_layout(),
            SpectralLayout::BitReversed
        );
        assert_eq!(plan.dif_stages.len(), 3);
        assert_eq!(plan.dit_stages.len(), 3);
        assert!(NttPlan::new(P_NTT, 3).is_err());
    }

    #[test]
    fn test_plan_creation_rejects_invalid_inputs() {
        assert!(NttPlan::new(P_NTT, 0).is_err());
        assert!(NttPlan::new(17, 16).is_err());
    }

    #[test]
    fn test_primitive_root_of_unity_small_prime() {
        let psi = primitive_root_of_unity(P_SMALL, 8).unwrap();
        assert_eq!(mod_pow(psi, 16, P_SMALL), 1);
        assert_eq!(mod_pow(psi, 8, P_SMALL), P_SMALL - 1);
    }

    #[test]
    fn test_primitive_root_of_unity_degree_one() {
        let psi = primitive_root_of_unity(P_NTT, 1).unwrap();
        assert_eq!(mod_pow(psi, 2, P_NTT), 1);
        assert_eq!(mod_pow(psi, 1, P_NTT), P_NTT - 1);
    }

    #[test]
    fn test_primitive_root_of_unity_rejects_unsupported_modulus() {
        assert!(primitive_root_of_unity(17, 16).is_err());
    }

    #[test]
    fn test_bit_reverse_permute() {
        let mut data = vec![0u64, 1, 2, 3, 4, 5, 6, 7];
        NttPlan::bit_reverse_in_place(&mut data);
        assert_eq!(data, vec![0, 4, 2, 6, 1, 5, 3, 7]);
    }

    #[test]
    fn test_bit_reverse_permute_len1_is_noop() {
        let mut data = vec![42u64];
        NttPlan::bit_reverse_in_place(&mut data);
        assert_eq!(data, vec![42u64]);
    }

    #[test]
    fn test_reference_roundtrip() {
        let reference = ConcreteNttRefPlan::new(P_NTT, 16).unwrap();
        let original = vec![1u64, 2, 3, 4, 5, 6, 7, 8, 0, 1, 1, 2, 3, 5, 8, 13];
        let mut data = original.clone();
        reference.forward(&mut data);
        reference.inverse(&mut data);
        assert_eq!(data, original);
    }

    #[test]
    fn test_reference_plan_rejects_tiny_degrees() {
        assert!(ConcreteNttRefPlan::new(P_NTT, 1).is_err());
        assert!(ConcreteNttRefPlan::new(P_NTT, 2).is_err());
    }

    #[test]
    fn test_ntt_roundtrip_len1_high_value() {
        let input = vec![P_NTT - 7];
        assert_roundtrip(&input);
    }

    #[test]
    fn test_forward_matches_reference_len16_dense_input() {
        let input = vec![5u64, 9, 2, 7, 1, 8, 3, 6, 4, 0, 11, 13, 10, 12, 14, 15];
        assert_forward_matches_reference(&input);
    }

    #[test]
    fn test_forward_matches_reference_len16_sparse_input() {
        let input = vec![1u64, 0, 0, 0, 6, 0, 9, 0, 0, 0, 0, 0, 3, 0, 0, 0];
        assert_forward_matches_reference(&input);
    }

    #[test]
    fn test_forward_matches_reference_len32_dense_input() {
        let input = vec![
            0u64, 1, 4, 9, 16, 25, 5, 18, 2, 19, 7, 28, 12, 31, 21, 13, 8, 6, 27, 11, 30, 20,
            14, 10, 15, 22, 3, 17, 24, 26, 23, 29,
        ];
        assert_forward_matches_reference(&input);
    }

    #[test]
    fn test_forward_matches_reference_len256_patterned_input() {
        let input = patterned_input(256);
        assert_forward_matches_reference(&input);
    }

    #[test]
    fn test_core_roundtrip_len1_input() {
        let input = vec![P_NTT - 5];
        assert_core_roundtrip(&input);
    }

    #[test]
    fn test_core_roundtrip_len8_dense_input() {
        let input = vec![1u64, 2, 3, 4, 5, 6, 7, 8];
        assert_core_roundtrip(&input);
    }

    #[test]
    fn test_core_roundtrip_len16_sparse_input() {
        let input = vec![1u64, 0, 0, 0, 6, 0, 9, 0, 0, 0, 0, 0, 3, 0, 0, 0];
        assert_core_roundtrip(&input);
    }

    #[test]
    fn test_core_roundtrip_zero_vector() {
        let input = vec![0u64; 16];
        assert_core_roundtrip(&input);
    }

    #[test]
    fn test_ntt_roundtrip_len2_high_values() {
        let input = vec![P_NTT - 1, P_NTT - 2];
        assert_roundtrip(&input);
    }

    #[test]
    fn test_inverse_matches_reference_len16_dense_input() {
        let input = vec![5u64, 9, 2, 7, 1, 8, 3, 6, 4, 0, 11, 13, 10, 12, 14, 15];
        assert_inverse_matches_reference(&input);
    }

    #[test]
    fn test_inverse_matches_reference_len32_dense_input() {
        let input = vec![
            0u64, 1, 4, 9, 16, 25, 5, 18, 2, 19, 7, 28, 12, 31, 21, 13, 8, 6, 27, 11, 30, 20,
            14, 10, 15, 22, 3, 17, 24, 26, 23, 29,
        ];
        assert_inverse_matches_reference(&input);
    }

    #[test]
    fn test_inverse_matches_reference_len256_patterned_input() {
        let input = patterned_input(256);
        assert_inverse_matches_reference(&input);
    }

    #[test]
    fn test_ntt_roundtrip_len8_dense_input() {
        let input = vec![1u64, 2, 3, 4, 5, 6, 7, 8];
        assert_roundtrip(&input);
    }

    #[test]
    fn test_ntt_roundtrip_len16_dense_input() {
        let input = vec![5u64, 9, 2, 7, 1, 8, 3, 6, 4, 0, 11, 13, 10, 12, 14, 15];
        assert_roundtrip(&input);
    }

    #[test]
    fn test_ntt_roundtrip_len32_dense_input() {
        let input = vec![
            0u64, 1, 4, 9, 16, 25, 5, 18, 2, 19, 7, 28, 12, 31, 21, 13, 8, 6, 27, 11, 30, 20,
            14, 10, 15, 22, 3, 17, 24, 26, 23, 29,
        ];
        assert_roundtrip(&input);
    }

    #[test]
    fn test_ntt_roundtrip_zero_vector() {
        let input = vec![0u64; 32];
        assert_roundtrip(&input);
    }

    #[test]
    fn test_ntt_roundtrip_len256_patterned_input() {
        let input = patterned_input(256);
        assert_roundtrip(&input);
    }

    #[test]
    fn test_ntt_roundtrip_len1024_patterned_input() {
        let input = patterned_input(1024);
        assert_roundtrip(&input);
    }

    #[test]
    fn test_self_impl_matches_concrete_ref() {
        let plan = NttPlan::new(P_NTT, 16).unwrap();
        let reference = ConcreteNttRefPlan::new(P_NTT, 16).unwrap();

        let lhs = vec![1u64, 4, 2, 8, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let rhs = vec![3u64, 5, 7, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        let mut self_lhs = lhs.clone();
        let mut self_rhs = rhs.clone();
        plan.forward_to_bitrev(&mut self_lhs);
        plan.forward_to_bitrev(&mut self_rhs);
        plan.pointwise_mul(&mut self_lhs, &self_rhs);
        plan.inverse_from_bitrev(&mut self_lhs);

        let mut ref_lhs = lhs;
        let mut ref_rhs = rhs;
        reference.forward(&mut ref_lhs);
        reference.forward(&mut ref_rhs);
        reference.pointwise_mul(&mut ref_lhs, &ref_rhs);
        reference.inverse(&mut ref_lhs);

        assert_eq!(self_lhs, ref_lhs);
    }

    #[test]
    fn test_negacyclic_convolution_matches_schoolbook_len8() {
        let plan = NttPlan::new(P_NTT, 8).unwrap();
        let lhs = vec![1u64, 4, 2, 8, 6, 0, 7, 3];
        let rhs = vec![3u64, 5, 7, 9, 2, 1, 0, 4];

        let mut lhs_ntt = lhs.clone();
        let mut rhs_ntt = rhs.clone();
        plan.forward_to_bitrev(&mut lhs_ntt);
        plan.forward_to_bitrev(&mut rhs_ntt);
        plan.pointwise_mul(&mut lhs_ntt, &rhs_ntt);
        plan.inverse_from_bitrev(&mut lhs_ntt);

        let expected = schoolbook_negacyclic_mul(&lhs, &rhs, P_NTT);
        assert_eq!(lhs_ntt, expected);
    }

    #[test]
    fn test_negacyclic_convolution_matches_schoolbook_high_values() {
        let plan = NttPlan::new(P_NTT, 16).unwrap();
        let lhs = vec![
            P_NTT - 1,
            P_NTT - 2,
            P_NTT - 3,
            P_NTT - 4,
            5,
            6,
            7,
            8,
            9,
            10,
            11,
            12,
            13,
            14,
            15,
            16,
        ];
        let rhs = vec![
            16u64,
            15,
            14,
            13,
            12,
            11,
            10,
            9,
            8,
            7,
            6,
            5,
            P_NTT - 4,
            P_NTT - 3,
            P_NTT - 2,
            P_NTT - 1,
        ];

        let mut lhs_ntt = lhs.clone();
        let mut rhs_ntt = rhs.clone();
        plan.forward_to_bitrev(&mut lhs_ntt);
        plan.forward_to_bitrev(&mut rhs_ntt);
        plan.pointwise_mul(&mut lhs_ntt, &rhs_ntt);
        plan.inverse_from_bitrev(&mut lhs_ntt);

        let expected = schoolbook_negacyclic_mul(&lhs, &rhs, P_NTT);
        assert_eq!(lhs_ntt, expected);
    }
}
