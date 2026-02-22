use thiserror::Error;

#[derive(Debug, Error)]
pub enum CkksError {
    #[error("math error: {0}")]
    Math(#[from] fhe_math::MathError),

    #[error("encoding error: {0}")]
    Encoding(String),

    #[error("decryption error: level mismatch (expected {expected}, got {got})")]
    LevelMismatch { expected: usize, got: usize },

    #[error("evaluation error: {0}")]
    Eval(String),

    #[error("key not found: {0}")]
    KeyNotFound(String),

    #[error("parameter error: {0}")]
    Params(String),
}
