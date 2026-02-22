use thiserror::Error;

#[derive(Debug, Error)]
pub enum MathError {
    #[error("modulus must be prime, got {0}")]
    NotPrime(u64),

    #[error("polynomial degree {0} is not a power of two")]
    DegreeNotPowerOfTwo(usize),

    #[error("NTT modulus {modulus} does not support degree {degree}: need modulus ≡ 1 (mod 2*degree)")]
    NttUnsupported { modulus: u64, degree: usize },

    #[error("RNS basis mismatch: expected {expected} limbs, got {got}")]
    RnsBasisMismatch { expected: usize, got: usize },

    #[error("index out of bounds: index {index} for length {len}")]
    IndexOutOfBounds { index: usize, len: usize },
}
