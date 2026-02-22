//! Pass manager: runs a sequence of IR optimization passes.

use crate::CompilerError;
use fhe_ir::{graph::IrGraph, passes::Pass};

pub struct PassManager {
    passes: Vec<Box<dyn Pass>>,
}

impl PassManager {
    pub fn new() -> Self {
        Self { passes: Vec::new() }
    }

    pub fn add_pass(&mut self, pass: Box<dyn Pass>) {
        self.passes.push(pass);
    }

    pub fn run(&self, graph: &mut IrGraph) -> Result<(), CompilerError> {
        for pass in &self.passes {
            pass.run(graph)?;
        }
        Ok(())
    }
}

impl Default for PassManager {
    fn default() -> Self {
        Self::new()
    }
}
