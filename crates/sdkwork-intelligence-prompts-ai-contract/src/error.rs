#[derive(Debug, thiserror::Error)]
pub enum PromptAiError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("validation: {0}")]
    Validation(String),
    #[error("internal: {0}")]
    Internal(String),
}

pub type PromptAiResult<T> = Result<T, PromptAiError>;
