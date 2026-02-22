//! CKKS plaintext type.

use fhe_math::rns::RnsPoly;
use serde::{Deserialize, Serialize};

/// A CKKS plaintext: an RNS polynomial at a given level and scale.
///
/// The plaintext polynomial `m` encodes a vector of complex numbers via
/// the canonical embedding (see `encoding::encoder`).
///
/// # Invariant
/// `poly.coeffs[i] ≈ round(m_i * scale)` where `m_i` are the original values.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Plaintext {
    /// The encoded polynomial.
    pub poly: RnsPoly,
    /// Current level (number of RNS limbs is `level + 1`).
    pub level: usize,
    /// Current scale factor Δ (encodes the precision).
    pub scale: f64,
}

impl Plaintext {
    pub fn new(poly: RnsPoly, level: usize, scale: f64) -> Self {
        Self { poly, level, scale }
    }
}
