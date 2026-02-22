//! Rescale insertion pass: automatically insert Rescale nodes after multiplications.
//!
//! In the CKKS workflow, every multiplication must be followed by a rescale
//! to prevent the scale from growing unboundedly. This pass detects patterns
//! where a MulCtCt + Relinearize is not followed by Rescale, and inserts one.

use crate::{graph::IrGraph, node::{CtMeta, IrNode}, passes::Pass, IrError};

pub struct RescaleInsertPass;

impl Pass for RescaleInsertPass {
    fn name(&self) -> &'static str { "rescale_insert" }

    fn run(&self, graph: &mut IrGraph) -> Result<(), IrError> {
        todo!(
            "for each Relinearize node: check if its successor is Rescale; \
             if not, insert a Rescale node between Relin and its successors"
        )
    }
}
