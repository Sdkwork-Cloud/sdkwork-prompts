use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(msg: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg),
        }
    }
}

/// Java-compatible backend envelope consumed by generated TypeScript admin SDKs.
#[derive(Debug, Serialize)]
pub struct PlusApiResult<T: Serialize> {
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> PlusApiResult<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: "0".to_string(),
            msg: Some("ok".to_string()),
            data: Some(data),
        }
    }
}
