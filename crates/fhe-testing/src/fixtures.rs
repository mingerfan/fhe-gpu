//! Pre-built test fixtures: toy parameters, seeded RNG, toy keysets.

use ckks::{
    core::params::{toy_params, CkksParams},
    crypto::keygen::KeyGenerator,
    core::keys::KeySet,
};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::sync::Arc;

/// A deterministic RNG seeded with 42 (for reproducible tests).
pub fn seeded_rng() -> ChaCha20Rng {
    ChaCha20Rng::seed_from_u64(42)
}

/// The toy CKKS parameters (tiny, fast, insecure).
pub fn toy_params_arc() -> Arc<CkksParams> {
    Arc::new(toy_params())
}

/// A complete keyset generated from toy parameters with the seeded RNG.
///
/// **Note**: This will panic until `KeyGenerator` is fully implemented.
/// Use `#[ignore]` on tests that call this until then.
pub fn toy_keyset() -> (Arc<CkksParams>, KeySet) {
    let params = toy_params_arc();
    let mut rng = seeded_rng();
    let keygen = KeyGenerator::new(params.clone());
    let keyset = keygen.gen_keyset(&mut rng, &[1, -1, 2, -2]);
    (params, keyset)
}
