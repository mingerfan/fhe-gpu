//! IR type system.

use serde::{Deserialize, Serialize};

/// The type of an IR value.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum IrType {
    /// An encrypted value (ciphertext).
    CiphertextTy,
    /// An unencrypted polynomial (plaintext).
    PlaintextTy,
    /// A scalar constant (not a polynomial).
    ScalarTy,
}
