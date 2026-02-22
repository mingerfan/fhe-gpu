pub mod dead_code;
pub mod level_assign;
pub mod rescale_insert;

use crate::{graph::IrGraph, IrError};

/// Trait for IR optimization passes.
pub trait Pass {
    fn name(&self) -> &'static str;
    fn run(&self, graph: &mut IrGraph) -> Result<(), IrError>;
}
