//! `fhe-math` — Pure mathematical building blocks for FHE.
//!
//! This crate contains no FHE-specific semantics. It provides:
//! - Modular arithmetic primitives (`modular`)
//! - NTT planning & transforms (`ntt`)
//! - Single-modulus polynomials over Z_q (`poly`)
//! - RNS (Residue Number System) multi-limb polynomials (`rns`)
//!
//! # Learning Path
//! Start with `modular`, then `ntt`, then `poly`, then `rns`.
//! Each module contains `todo!()` skeletons with mathematical specs and references.

pub mod error;
pub mod modular;
pub mod ntt;
pub mod poly;
pub mod rns;

pub use error::MathError;
