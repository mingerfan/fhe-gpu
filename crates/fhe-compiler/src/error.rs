use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("IR error: {0}")]
    Ir(#[from] fhe_ir::IrError),

    #[error("CKKS error: {0}")]
    Ckks(#[from] ckks::CkksError),

    #[error("lowering error: {0}")]
    Lowering(String),

    #[error("difftest error: {0}")]
    Difftest(String),

    #[error("subprocess error: {0}")]
    Subprocess(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
