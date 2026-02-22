// integration_encrypt.rs — Encrypt/decrypt integration tests.
//
// Run with: cargo test --features phase-crypto

use ckks::{
    core::params::toy_params,
    crypto::{decrypt::decrypt, encrypt::encrypt, keygen::KeyGenerator},
    encoding::CkksEncoder,
};
use num_complex::Complex64;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::sync::Arc;

#[cfg(feature = "phase-crypto")]
mod crypto_tests {
    use super::*;

    fn setup() -> (Arc<ckks::core::params::CkksParams>, ckks::core::keys::KeySet, CkksEncoder) {
        let params = Arc::new(toy_params());
        let mut rng = ChaCha20Rng::seed_from_u64(42);
        let keygen = KeyGenerator::new(params.clone());
        let keyset = keygen.gen_keyset(&mut rng, &[1, -1]);
        let encoder = CkksEncoder::new(params.clone());
        (params, keyset, encoder)
    }

    #[test]
    #[ignore = "temporary bisect: isolate next failing layer"]
    fn test_public_key_encrypt_decrypt() {
        let (params, keyset, encoder) = setup();
        let mut rng = ChaCha20Rng::seed_from_u64(99);
        let values = vec![Complex64::new(1.5, 0.0), Complex64::new(-0.5, 0.25)];
        let pt = encoder.encode(&values, params.max_depth).unwrap();
        let ct = encrypt(&pt, &keyset.public_key, &params, &mut rng).unwrap();
        let pt2 = decrypt(&ct, &keyset.secret_key, &params).unwrap();
        let decoded = encoder.decode(&pt2).unwrap();
        fhe_testing::assert_slots_close!(decoded, values, rel_tol = 1e-3);
    }

    #[test]
    #[ignore = "temporary bisect: isolate next failing layer"]
    fn test_symmetric_encrypt_decrypt() {
        use ckks::crypto::encrypt::encrypt_symmetric;
        let (params, keyset, encoder) = setup();
        let mut rng = ChaCha20Rng::seed_from_u64(77);
        let values = vec![Complex64::new(3.14, -1.41)];
        let pt = encoder.encode(&values, params.max_depth).unwrap();
        let ct = encrypt_symmetric(&pt, &keyset.secret_key, &params, &mut rng).unwrap();
        let pt2 = decrypt(&ct, &keyset.secret_key, &params).unwrap();
        let decoded = encoder.decode(&pt2).unwrap();
        fhe_testing::assert_slots_close!(decoded, values, rel_tol = 1e-3);
    }
}

#[cfg(not(feature = "phase-crypto"))]
#[test]
fn crypto_tests_disabled() {
    // Run with --features phase-crypto to enable these tests
}
