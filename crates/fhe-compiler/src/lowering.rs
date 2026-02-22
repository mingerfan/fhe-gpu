//! Lowering: translate an optimized IR graph into CKKS evaluator calls.
//!
//! The `Lowering` struct walks the IR graph in topological order and dispatches
//! each node to the appropriate `CkksEvaluator` method.

use crate::CompilerError;
use ckks::{core::Ciphertext, eval::CkksEvaluator};
use fhe_ir::{graph::IrGraph, node::IrNode};
use std::collections::HashMap;

/// A lowered, ready-to-execute CKKS computation.
pub struct Lowering {
    graph: IrGraph,
}

impl Lowering {
    pub fn new(graph: IrGraph) -> Self {
        Self { graph }
    }

    /// Execute the computation with the given input ciphertexts.
    ///
    /// `inputs` maps input names to ciphertext values.
    /// Returns a map from output names to result ciphertexts.
    ///
    /// # Implementation Notes
    /// Walk nodes in topological order; maintain a `values: HashMap<NodeIndex, Ciphertext>`.
    /// For each node, look up its inputs from `values`, call the appropriate evaluator method,
    /// and store the result back.
    pub fn execute(
        &self,
        inputs: HashMap<String, Ciphertext>,
        evaluator: &CkksEvaluator,
    ) -> Result<HashMap<String, Ciphertext>, CompilerError> {
        todo!(
            "topological sort graph; \
             walk nodes: InputCt → look up in inputs map; \
             AddCtCt → evaluator add; MulCtCt → evaluator mul; \
             Relinearize → evaluator relin; Rescale → evaluator rescale; \
             Rotate → evaluator rotate; Output → collect into results map"
        )
    }
}
