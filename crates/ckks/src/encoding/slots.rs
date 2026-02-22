//! Slot permutation and canonical embedding helpers.
//!
//! The CKKS slot ordering is determined by the bit-reversal permutation
//! and the mapping from slot indices to evaluation points (roots of unity).
//!
//! # Learning Resources
//! - [EN] Bit-reversal permutation in FFT: https://en.wikipedia.org/wiki/Bit-reversal_permutation
//! - [EN] CKKS slot indexing: N/A
//! - [CN] 比特逆序置换（OI-Wiki）: https://oi-wiki.org/math/poly/fft/
//! - [CN] CKKS 槽映射: N/A

/// Compute the bit-reversal permutation for an array of length `n`.
///
/// For the CKKS canonical embedding, the i-th slot corresponds to the
/// evaluation point ξ^{perm[i] * 2 + 1} where perm is the bit-reversal.
///
/// # Mathematical Specification
/// ```text
/// perm[i] = bit_reverse(i, log2(n))
/// where bit_reverse reverses the log2(n) least significant bits.
/// ```
///
/// # Learning Resources
/// - [EN] Bit-reversal in Cooley-Tukey FFT: https://cp-algorithms.com/algebra/fft.html
/// - [CN] 比特逆序置换推导（OI-Wiki）: https://oi-wiki.org/math/poly/fft/
pub fn bit_reversal_permutation(n: usize) -> Vec<usize> {
    assert!(n.is_power_of_two(), "n must be a power of 2");
    let log_n = n.trailing_zeros() as usize;
    (0..n).map(|i| i.reverse_bits() >> (usize::BITS as usize - log_n)).collect()
}

/// Compute the primitive `2n`-th roots of unity in C: ξ_j = exp(iπ(2j+1)/n).
///
/// These are the evaluation points for the CKKS canonical embedding.
/// Only the first `n/2` are needed (the rest are conjugates).
///
/// # Learning Resources
/// - [EN] Roots of unity in canonical embedding: https://eprint.iacr.org/2016/421.pdf §3
/// - [CN] 规范嵌入单位根: N/A
pub fn evaluation_points(n: usize) -> Vec<num_complex::Complex64> {
    use num_complex::Complex64;
    let half = n / 2;
    (0..half)
        .map(|j| {
            let angle = std::f64::consts::PI * (2 * j + 1) as f64 / n as f64;
            Complex64::from_polar(1.0, angle)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_reversal_n4() {
        let perm = bit_reversal_permutation(4);
        // For n=4: 00→00, 01→10, 10→01, 11→11 → [0, 2, 1, 3]
        assert_eq!(perm, vec![0, 2, 1, 3]);
    }

    #[test]
    fn test_bit_reversal_n8() {
        let perm = bit_reversal_permutation(8);
        assert_eq!(perm, vec![0, 4, 2, 6, 1, 5, 3, 7]);
    }

    #[test]
    fn test_evaluation_points_on_unit_circle() {
        let pts = evaluation_points(8);
        for pt in &pts {
            let norm = pt.re * pt.re + pt.im * pt.im;
            assert!((norm - 1.0).abs() < 1e-14, "point not on unit circle: {norm}");
        }
    }
}
