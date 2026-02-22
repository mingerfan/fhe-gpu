//! Compilation pipeline: IR → optimization passes → lowering.

use crate::{
    error::CompilerError,
    lowering::Lowering,
    optimizer::PassManager,
};
use fhe_ir::IrGraph;

/// The main compilation pipeline.
pub struct CompilationPipeline {
    pass_manager: PassManager,
}

impl CompilationPipeline {
    /// Create a pipeline with the default pass sequence.
    ///
    /// Default passes (in order):
    /// 1. `LevelAssignPass` — annotate levels
    /// 2. `RescaleInsertPass` — insert missing rescales
    /// 3. `DeadCodePass` — remove unreachable nodes
    pub fn default_pipeline(max_depth: usize) -> Self {
        use fhe_ir::passes::{
            dead_code::DeadCodePass,
            level_assign::LevelAssignPass,
            rescale_insert::RescaleInsertPass,
        };
        let mut pm = PassManager::new();
        pm.add_pass(Box::new(LevelAssignPass { max_depth }));
        pm.add_pass(Box::new(RescaleInsertPass));
        pm.add_pass(Box::new(DeadCodePass));
        Self { pass_manager: pm }
    }

    /// Run the pipeline: optimize the graph and lower to evaluator calls.
    pub fn compile(&self, mut graph: IrGraph) -> Result<Lowering, CompilerError> {
        self.pass_manager.run(&mut graph)?;
        Ok(Lowering::new(graph))
    }
}
