use thiserror::Error;

#[derive(Debug, Error)]
pub enum IrError {
    #[error("graph contains a cycle (FHE programs must be acyclic)")]
    CycleDetected,

    #[error("node {0:?} not found in graph")]
    NodeNotFound(petgraph::graph::NodeIndex),

    #[error("type error: {0}")]
    TypeError(String),

    #[error("pass error: {pass}: {message}")]
    PassError { pass: &'static str, message: String },
}
