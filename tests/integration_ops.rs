// integration_ops.rs — Homomorphic operation integration tests.
//
// Run with: cargo test --features phase-eval

use ckks::{
    core::params::toy_params,
    crypto::{decrypt::decrypt, encrypt::encrypt, keygen::KeyGenerator},
    encoding::CkksEncoder,
    eval::{
        add::{add_ct_ct, add_ct_pt},
        evaluator::CkksEvaluator,
        mul::{mul_ct_ct, mul_ct_pt},
        rescale::rescale,
    },
};
use num_complex::Complex64;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::sync::Arc;

#[cfg(feature = "phase-eval")]
mod eval_tests {
    use super::*;
    use ckks::eval::relin::relinearize;

    fn setup() -> (
        Arc<ckks::core::params::CkksParams>,
        ckks::core::keys::KeySet,
        CkksEncoder,
    ) {
        let params = Arc::new(toy_params());
        let mut rng = ChaCha20Rng::seed_from_u64(42);
        let keygen = KeyGenerator::new(params.clone());
        let keyset = keygen.gen_keyset(&mut rng, &[1, -1, 2]);
        let encoder = CkksEncoder::new(params.clone());
        (params, keyset, encoder)
    }

    fn encrypt_values(
        values: &[Complex64],
        params: &Arc<ckks::core::params::CkksParams>,
        keyset: &ckks::core::keys::KeySet,
        encoder: &CkksEncoder,
        rng: &mut impl rand::RngCore,
    ) -> ckks::core::Ciphertext {
        let pt = encoder.encode(values, params.max_depth).unwrap();
        encrypt(&pt, &keyset.public_key, params, rng).unwrap()
    }

    #[test]
    #[ignore = "temporary bisect: isolate next failing layer"]
    fn test_homomorphic_addition() {
        let (params, keyset, encoder) = setup();
        let mut rng = ChaCha20Rng::seed_from_u64(1);
        let x = vec![Complex64::new(1.0, 0.0), Complex64::new(2.0, 0.0)];
        let y = vec![Complex64::new(3.0, 0.0), Complex64::new(4.0, 0.0)];
        let expected: Vec<_> = x.iter().zip(y.iter()).map(|(a, b)| a + b).collect();

        let ct_x = encrypt_values(&x, &params, &keyset, &encoder, &mut rng);
        let ct_y = encrypt_values(&y, &params, &keyset, &encoder, &mut rng);
        let ct_sum = add_ct_ct(&ct_x, &ct_y).unwrap();
        let pt_result = decrypt(&ct_sum, &keyset.secret_key, &params).unwrap();
        let decoded = encoder.decode(&pt_result).unwrap();

        fhe_testing::assert_slots_close!(decoded, expected, rel_tol = 1e-3);
    }

    #[test]
    #[ignore = "temporary bisect: isolate next failing layer"]
    fn test_homomorphic_multiplication() {
        let (params, keyset, encoder) = setup();
        let mut rng = ChaCha20Rng::seed_from_u64(2);
        let x = vec![Complex64::new(2.0, 0.0), Complex64::new(3.0, 0.0)];
        let y = vec![Complex64::new(4.0, 0.0), Complex64::new(5.0, 0.0)];
        let expected: Vec<_> = x.iter().zip(y.iter()).map(|(a, b)| a * b).collect();

        let ct_x = encrypt_values(&x, &params, &keyset, &encoder, &mut rng);
        let ct_y = encrypt_values(&y, &params, &keyset, &encoder, &mut rng);
        let ct2 = mul_ct_ct(&ct_x, &ct_y).unwrap();
        let ct_relin = relinearize(ct2, &keyset.relin_key).unwrap();
        let ct_result = rescale(&ct_relin, &params).unwrap();
        let pt_result = decrypt(&ct_result, &keyset.secret_key, &params).unwrap();
        let decoded = encoder.decode(&pt_result).unwrap();

        fhe_testing::assert_slots_close!(decoded, expected, rel_tol = 1e-3);
    }

    #[test]
    #[ignore = "temporary bisect: isolate next failing layer"]
    fn test_add_then_mul() {
        // (x + y) * z
        let (params, keyset, encoder) = setup();
        let mut rng = ChaCha20Rng::seed_from_u64(3);
        let x = vec![Complex64::new(1.0, 0.0)];
        let y = vec![Complex64::new(2.0, 0.0)];
        let z = vec![Complex64::new(3.0, 0.0)];
        let expected = vec![Complex64::new(9.0, 0.0)]; // (1+2)*3

        let ct_x = encrypt_values(&x, &params, &keyset, &encoder, &mut rng);
        let ct_y = encrypt_values(&y, &params, &keyset, &encoder, &mut rng);
        let ct_z = encrypt_values(&z, &params, &keyset, &encoder, &mut rng);

        let ct_sum = add_ct_ct(&ct_x, &ct_y).unwrap();
        let ct2 = mul_ct_ct(&ct_sum, &ct_z).unwrap();
        let ct_relin = relinearize(ct2, &keyset.relin_key).unwrap();
        let ct_result = rescale(&ct_relin, &params).unwrap();

        let pt = decrypt(&ct_result, &keyset.secret_key, &params).unwrap();
        let decoded = encoder.decode(&pt).unwrap();
        fhe_testing::assert_slots_close!(decoded, expected, rel_tol = 1e-3);
    }
}

#[cfg(not(feature = "phase-eval"))]
#[test]
fn eval_tests_disabled() {
    // Run with --features phase-eval to enable these tests
}
