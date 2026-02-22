//! `fhe-compiler` — Compilation pipeline bridging IR → concrete CKKS operations.
//!
//! This crate is the only one that imports both `fhe-ir` and `ckks`.
//! It provides:
//! - `CompilationPipeline`: runs optimization passes and lowers to evaluator calls
//! - `DifftestHarness`: differential testing against the Python OpenFHE oracle

pub mod difftest;
pub mod error;
pub mod lowering;
pub mod optimizer;
pub mod pipeline;

pub use error::CompilerError;
pub use pipeline::CompilationPipeline;
