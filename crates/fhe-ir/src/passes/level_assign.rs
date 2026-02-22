//! Level assignment pass: compute the FHE level for each node.
//!
//! Starting from inputs (at max_depth), each multiplication decreases the level by 1
//! (after rescaling). This pass propagates level information through the graph.

use crate::{graph::IrGraph, node::IrNode, passes::Pass, IrError};

pub struct LevelAssignPass {
    pub max_depth: usize,
}

impl Pass for LevelAssignPass {
    fn name(&self) -> &'static str { "level_assign" }

    fn run(&self, graph: &mut IrGraph) -> Result<(), IrError> {
        todo!(
            "topological sort; for each node: \
             InputCt → level = max_depth; \
             AddCtCt/SubCtCt → level = min(inputs); \
             MulCtCt → level = min(inputs); \
             Rescale → level = predecessor.level - 1; \
             propagate forward"
        )
    }
}
