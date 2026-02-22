//! Random plaintext and ciphertext generators for property-based testing.

use ckks::core::params::CkksParams;
use num_complex::Complex64;
use rand::RngCore;

/// Generate a vector of random complex numbers with magnitude bounded by `max_val`.
pub fn random_complex_vec(num_slots: usize, max_val: f64, rng: &mut impl RngCore) -> Vec<Complex64> {
    (0..num_slots)
        .map(|_| {
            let re = (rng.next_u64() as f64 / u64::MAX as f64) * 2.0 * max_val - max_val;
            let im = (rng.next_u64() as f64 / u64::MAX as f64) * 2.0 * max_val - max_val;
            Complex64::new(re, im)
        })
        .collect()
}

/// Generate a vector of random real numbers (imaginary part = 0) bounded by `max_val`.
pub fn random_real_vec(num_slots: usize, max_val: f64, rng: &mut impl RngCore) -> Vec<Complex64> {
    (0..num_slots)
        .map(|_| {
            let re = (rng.next_u64() as f64 / u64::MAX as f64) * 2.0 * max_val - max_val;
            Complex64::new(re, 0.0)
        })
        .collect()
}

/// Compute the maximum absolute error between two complex vectors.
pub fn max_abs_error(a: &[Complex64], b: &[Complex64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).norm())
        .fold(0.0f64, f64::max)
}
