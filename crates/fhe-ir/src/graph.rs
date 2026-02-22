//! The IR computation graph.

use crate::{node::IrNode, IrError};
use petgraph::{
    algo::toposort,
    graph::{DiGraph, NodeIndex},
    Direction,
};
use serde::{Deserialize, Serialize};

/// A directed acyclic graph (DAG) of FHE operations.
///
/// Edges represent data flow: an edge from node A to node B means
/// "B consumes the output of A."
pub struct IrGraph {
    /// The underlying petgraph directed graph.
    pub graph: DiGraph<IrNode, ()>,
}

impl IrGraph {
    pub fn new() -> Self {
        Self { graph: DiGraph::new() }
    }

    /// Add a node to the graph, returning its index.
    pub fn add_node(&mut self, node: IrNode) -> NodeIndex {
        self.graph.add_node(node)
    }

    /// Add a data-flow edge from `from` to `to` (to uses from's output).
    pub fn add_edge(&mut self, from: NodeIndex, to: NodeIndex) {
        self.graph.add_edge(from, to, ());
    }

    /// Compute a topological ordering of nodes (input → output order).
    ///
    /// Returns `IrError::CycleDetected` if the graph has a cycle.
    pub fn topological_order(&self) -> Result<Vec<NodeIndex>, IrError> {
        toposort(&self.graph, None).map_err(|_| IrError::CycleDetected)
    }

    /// Get the node at a given index.
    pub fn node(&self, idx: NodeIndex) -> Result<&IrNode, IrError> {
        self.graph.node_weight(idx).ok_or(IrError::NodeNotFound(idx))
    }

    /// Get a mutable reference to the node at a given index.
    pub fn node_mut(&mut self, idx: NodeIndex) -> Result<&mut IrNode, IrError> {
        self.graph.node_weight_mut(idx).ok_or(IrError::NodeNotFound(idx))
    }

    /// Get indices of predecessor nodes (inputs to `idx`).
    pub fn predecessors(&self, idx: NodeIndex) -> Vec<NodeIndex> {
        self.graph.neighbors_directed(idx, Direction::Incoming).collect()
    }

    /// Number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Number of edges in the graph.
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
}

impl Default for IrGraph {
    fn default() -> Self {
        Self::new()
    }
}
