//! IR node types: the operations that can appear in an FHE computation graph.

use serde::{Deserialize, Serialize};

/// Metadata carried by every ciphertext-valued node.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CtMeta {
    /// FHE level (number of remaining rescale budgets).
    pub level: Option<usize>,
    /// Current scale factor Δ^k.
    pub scale: Option<f64>,
}

impl CtMeta {
    pub fn unknown() -> Self {
        Self { level: None, scale: None }
    }
}

/// An IR node representing one FHE operation.
///
/// Each variant carries only scalar metadata (not actual ciphertext objects).
/// The `fhe-compiler` crate maps these nodes to concrete evaluator calls.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum IrNode {
    /// Input ciphertext (a named program input).
    InputCt { name: String, meta: CtMeta },

    /// Input plaintext constant.
    InputPt { name: String },

    /// Output node (marks a value as a program result).
    Output { name: String },

    /// Ciphertext + Ciphertext addition.
    AddCtCt { meta: CtMeta },

    /// Ciphertext + Plaintext addition.
    AddCtPt { meta: CtMeta },

    /// Ciphertext - Ciphertext subtraction.
    SubCtCt { meta: CtMeta },

    /// Ciphertext negation.
    NegCt { meta: CtMeta },

    /// Ciphertext × Ciphertext multiplication (before relin).
    MulCtCt { meta: CtMeta },

    /// Ciphertext × Plaintext multiplication.
    MulCtPt { meta: CtMeta },

    /// Relinearization (reduces degree-2 → degree-1).
    Relinearize { meta: CtMeta },

    /// Rescale (drop one RNS limb, reduce scale by one prime).
    Rescale { meta: CtMeta },

    /// Modulus switch (drop level without adjusting scale).
    ModSwitch { target_level: usize, meta: CtMeta },

    /// Slot rotation by `step` positions.
    Rotate { step: i32, meta: CtMeta },

    /// Complex conjugation of all slots.
    Conjugate { meta: CtMeta },
}

impl IrNode {
    /// Return the output metadata of this node, if it produces a ciphertext.
    pub fn ct_meta(&self) -> Option<&CtMeta> {
        match self {
            Self::InputCt { meta, .. }
            | Self::AddCtCt { meta }
            | Self::AddCtPt { meta }
            | Self::SubCtCt { meta }
            | Self::NegCt { meta }
            | Self::MulCtCt { meta }
            | Self::MulCtPt { meta }
            | Self::Relinearize { meta }
            | Self::Rescale { meta }
            | Self::ModSwitch { meta, .. }
            | Self::Rotate { meta, .. }
            | Self::Conjugate { meta } => Some(meta),
            _ => None,
        }
    }

    /// Human-readable operation name.
    pub fn op_name(&self) -> &'static str {
        match self {
            Self::InputCt { .. }    => "InputCt",
            Self::InputPt { .. }    => "InputPt",
            Self::Output { .. }     => "Output",
            Self::AddCtCt { .. }    => "AddCtCt",
            Self::AddCtPt { .. }    => "AddCtPt",
            Self::SubCtCt { .. }    => "SubCtCt",
            Self::NegCt { .. }      => "NegCt",
            Self::MulCtCt { .. }    => "MulCtCt",
            Self::MulCtPt { .. }    => "MulCtPt",
            Self::Relinearize { .. } => "Relinearize",
            Self::Rescale { .. }    => "Rescale",
            Self::ModSwitch { .. }  => "ModSwitch",
            Self::Rotate { .. }     => "Rotate",
            Self::Conjugate { .. }  => "Conjugate",
        }
    }
}
