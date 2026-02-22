//! `ckks` — Full CKKS Fully Homomorphic Encryption implementation.
//!
//! # Structure
//! - `core` — Parameter sets, key types, plaintext/ciphertext structs
//! - `encoding` — Encoding/decoding between complex vectors and polynomials
//! - `crypto` — Key generation, encryption, decryption
//! - `eval` — Homomorphic operations (add, mul, relin, rescale, rotate)
//!
//! # Recommended Learning Order
//! 1. `core::params` — Understand the parameter space
//! 2. `encoding::encoder` — CKKS encoding/decoding (IDFT based)
//! 3. `crypto::keygen` → `crypto::encrypt` → `crypto::decrypt`
//! 4. `eval::add` → `eval::mul` → `eval::relin` → `eval::rescale` → `eval::rotate`
//!
//! # Key Papers
//! - CKKS original: https://eprint.iacr.org/2016/421.pdf
//! - Full RNS CKKS: https://eprint.iacr.org/2018/931.pdf
//! - CKKS bootstrapping: https://eprint.iacr.org/2018/153.pdf

pub mod core;
pub mod crypto;
pub mod encoding;
pub mod error;
pub mod eval;

pub use error::CkksError;
