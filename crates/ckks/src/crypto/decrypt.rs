//! CKKS decryption.
//!
//! # Decryption Formula
//! Given ciphertext `(c0, c1)` at level `l` and scale `Δ`:
//! ```text
//! m̃ = [c0 + c1 * s]_{Q_l}    (polynomial arithmetic in R_{Q_l})
//! ```
//! The result `m̃` is close to `Δ * m` (the true plaintext polynomial).
//! To recover the approximate plaintext, divide by `Δ` after decoding.
//!
//! # Noise Growth
//! After fresh encryption: noise ≈ σ√n.
//! After k multiplications: noise grows exponentially (mitigated by rescaling).
//!
//! # Learning Resources
//! - [EN] CKKS decryption: https://eprint.iacr.org/2016/421.pdf §3
//! - [EN] Noise analysis: https://eprint.iacr.org/2016/421.pdf §4
//! - [CN] CKKS 解密推导: N/A
//! - [CN] 噪声增长分析: N/A

use crate::{
    core::{keys::SecretKey, params::CkksParams, Ciphertext, Plaintext},
    CkksError,
};
use std::sync::Arc;

/// Decrypt a ciphertext using the secret key.
///
/// # Mathematical Specification
/// ```text
/// result = c0 + c1 * s mod Q_level   (polynomial multiplication in R_{Q_l})
/// return Plaintext { poly: result, level: ct.level, scale: ct.scale }
/// ```
///
/// # Errors
/// Returns error if `ct.level` is inconsistent.
///
/// # Learning Resources
/// - [EN] RLWE decryption correctness: https://eprint.iacr.org/2012/230.pdf §3
/// - [CN] 解密正确性证明: N/A
pub fn decrypt(
    ct: &Ciphertext,
    sk: &SecretKey,
    params: &Arc<CkksParams>,
) -> Result<Plaintext, CkksError> {
    todo!(
        "compute result = ct.c0 + ct.c1 * sk.poly in RNS at ct.level; \
         return Plaintext {{ poly: result, level: ct.level, scale: ct.scale }}"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::params::toy_params,
        crypto::{encrypt::encrypt, keygen::KeyGenerator},
        encoding::CkksEncoder,
    };
    use num_complex::Complex64;
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;
    use std::sync::Arc;

    #[test]
    #[ignore = "implement all crypto functions first"]
    fn test_encrypt_decrypt_roundtrip() {
        let params = Arc::new(toy_params());
        let mut rng = ChaCha20Rng::seed_from_u64(42);
        let keygen = KeyGenerator::new(params.clone());
        let sk = keygen.gen_sk(&mut rng);
        let pk = keygen.gen_pk(&sk, &mut rng);

        let encoder = CkksEncoder::new(params.clone());
        let values = vec![Complex64::new(1.5, 0.0), Complex64::new(-0.5, 0.25)];
        let pt = encoder.encode(&values, params.max_depth).unwrap();
        let ct = encrypt(&pt, &pk, &params, &mut rng).unwrap();
        let pt2 = decrypt(&ct, &sk, &params).unwrap();
        let decoded = encoder.decode(&pt2).unwrap();

        let tol = 1e-3;
        for (orig, dec) in values.iter().zip(decoded.iter()) {
            assert!((orig.re - dec.re).abs() < tol, "re mismatch: {orig} vs {dec}");
            assert!((orig.im - dec.im).abs() < tol, "im mismatch");
        }
    }
}
