//! `IrBuilder`: a fluent API for constructing FHE computation graphs.

use crate::{
    graph::IrGraph,
    node::{CtMeta, IrNode},
};
use petgraph::graph::NodeIndex;

/// Builder for constructing `IrGraph` instances.
///
/// # Example
/// ```rust
/// let mut builder = IrBuilder::new();
/// let x = builder.input_ct("x");
/// let y = builder.input_ct("y");
/// let sum = builder.add_ct_ct(x, y);
/// builder.output(sum, "result");
/// let graph = builder.build();
/// ```
pub struct IrBuilder {
    graph: IrGraph,
}

impl IrBuilder {
    pub fn new() -> Self {
        Self { graph: IrGraph::new() }
    }

    /// Add an input ciphertext node.
    pub fn input_ct(&mut self, name: impl Into<String>) -> NodeIndex {
        self.graph.add_node(IrNode::InputCt {
            name: name.into(),
            meta: CtMeta::unknown(),
        })
    }

    /// Add an input plaintext constant node.
    pub fn input_pt(&mut self, name: impl Into<String>) -> NodeIndex {
        self.graph.add_node(IrNode::InputPt { name: name.into() })
    }

    /// Add a ct+ct addition node, connecting its inputs.
    pub fn add_ct_ct(&mut self, a: NodeIndex, b: NodeIndex) -> NodeIndex {
        let node = self.graph.add_node(IrNode::AddCtCt { meta: CtMeta::unknown() });
        self.graph.add_edge(a, node);
        self.graph.add_edge(b, node);
        node
    }

    /// Add a ct+pt addition node.
    pub fn add_ct_pt(&mut self, ct: NodeIndex, pt: NodeIndex) -> NodeIndex {
        let node = self.graph.add_node(IrNode::AddCtPt { meta: CtMeta::unknown() });
        self.graph.add_edge(ct, node);
        self.graph.add_edge(pt, node);
        node
    }

    /// Add a ct*ct multiplication node.
    pub fn mul_ct_ct(&mut self, a: NodeIndex, b: NodeIndex) -> NodeIndex {
        let node = self.graph.add_node(IrNode::MulCtCt { meta: CtMeta::unknown() });
        self.graph.add_edge(a, node);
        self.graph.add_edge(b, node);
        node
    }

    /// Add a relinearization node.
    pub fn relinearize(&mut self, ct: NodeIndex) -> NodeIndex {
        let node = self.graph.add_node(IrNode::Relinearize { meta: CtMeta::unknown() });
        self.graph.add_edge(ct, node);
        node
    }

    /// Add a rescale node.
    pub fn rescale(&mut self, ct: NodeIndex) -> NodeIndex {
        let node = self.graph.add_node(IrNode::Rescale { meta: CtMeta::unknown() });
        self.graph.add_edge(ct, node);
        node
    }

    /// Add a rotate node.
    pub fn rotate(&mut self, ct: NodeIndex, step: i32) -> NodeIndex {
        let node = self.graph.add_node(IrNode::Rotate { step, meta: CtMeta::unknown() });
        self.graph.add_edge(ct, node);
        node
    }

    /// Mark a node as an output.
    pub fn output(&mut self, ct: NodeIndex, name: impl Into<String>) -> NodeIndex {
        let node = self.graph.add_node(IrNode::Output { name: name.into() });
        self.graph.add_edge(ct, node);
        node
    }

    /// Consume the builder and return the completed graph.
    pub fn build(self) -> IrGraph {
        self.graph
    }
}

impl Default for IrBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_simple_graph() {
        let mut b = IrBuilder::new();
        let x = b.input_ct("x");
        let y = b.input_ct("y");
        let sum = b.add_ct_ct(x, y);
        b.output(sum, "result");
        let g = b.build();
        assert_eq!(g.node_count(), 4); // x, y, add, output
        assert_eq!(g.edge_count(), 3); // x→add, y→add, add→output
    }

    #[test]
    fn test_topological_sort() {
        let mut b = IrBuilder::new();
        let x = b.input_ct("x");
        let y = b.input_ct("y");
        let prod = b.mul_ct_ct(x, y);
        let relin = b.relinearize(prod);
        let rescaled = b.rescale(relin);
        b.output(rescaled, "out");
        let g = b.build();
        let order = g.topological_order().unwrap();
        assert_eq!(order.len(), 6);
    }
}
