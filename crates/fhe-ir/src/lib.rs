//! `fhe-ir` — Scheme-agnostic intermediate representation for FHE computation graphs.
//!
//! The IR represents FHE programs as a directed acyclic graph (DAG) where:
//! - Nodes represent operations (add, mul, rotate, rescale, etc.)
//! - Edges represent data flow between operations
//! - Node metadata carries type information (level, scale, step)
//!
//! # Design Principle
//! The IR is intentionally **scheme-agnostic**: it does not import `ckks` or any
//! concrete FHE implementation. The `fhe-compiler` crate bridges IR → concrete calls.
//!
//! # Typical Workflow
//! ```text
//! User writes computation → IrBuilder constructs IrGraph →
//! Optimization passes (level assignment, rescale insertion, DCE) →
//! fhe-compiler lowering → concrete CKKS evaluator calls
//! ```

pub mod builder;
pub mod error;
pub mod graph;
pub mod node;
pub mod passes;
pub mod types;

pub use builder::IrBuilder;
pub use error::IrError;
pub use graph::IrGraph;
pub use node::IrNode;
pub use types::IrType;
