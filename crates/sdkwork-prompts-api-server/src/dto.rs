use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateTopicRequest {
    pub title: String,
    pub body: String,
    pub board_id: i64,
    pub body_format: Option<String>,
    pub topic_type: Option<String>,
    pub visibility: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReplyRequest {
    pub body: String,
    pub body_format: Option<String>,
    pub parent_reply_id: Option<i64>,
}

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
