//! Dead code elimination pass: remove nodes whose outputs are never used.

use crate::{graph::IrGraph, passes::Pass, IrError};

pub struct DeadCodePass;

impl Pass for DeadCodePass {
    fn name(&self) -> &'static str { "dead_code" }

    fn run(&self, graph: &mut IrGraph) -> Result<(), IrError> {
        todo!(
            "reverse topological order; mark nodes reachable from Output nodes; \
             remove all unmarked nodes (and their edges)"
        )
    }
}
