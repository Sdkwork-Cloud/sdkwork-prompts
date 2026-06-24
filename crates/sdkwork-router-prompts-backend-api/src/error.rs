#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptsRouteError {
    pub code: &'static str,
    pub status: u16,
    pub message: String,
}

impl PromptsRouteError {
    pub fn not_implemented(operation_id: &str) -> Self {
        Self {
            code: "prompts.backend.route.not_implemented",
            status: 501,
            message: format!("not implemented: {operation_id}"),
        }
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self {
            code: "prompts.backend.route.validation",
            status: 400,
            message: message.into(),
        }
    }

    pub fn not_found(resource: &str, id: &str) -> Self {
        Self {
            code: "prompts.backend.route.not_found",
            status: 404,
            message: format!("{resource} not found: {id}"),
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            code: "prompts.backend.route.internal",
            status: 500,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for PromptsRouteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for PromptsRouteError {}
