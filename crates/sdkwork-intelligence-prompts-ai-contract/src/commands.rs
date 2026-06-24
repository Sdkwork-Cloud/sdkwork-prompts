use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PromptAiSubject {
    pub tenant_id: i64,
    pub organization_id: i64,
    pub operator_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptAiItem {
    pub id: i64,
    pub uuid: String,
    pub tenant_id: i64,
    pub organization_id: i64,
    pub prompt_key: String,
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<String>,
    pub category_code: Option<String>,
    pub prompt_type: String,
    pub visibility: String,
    pub owner_user_id: Option<i64>,
    pub latest_version_id: Option<i64>,
    pub published_version_id: Option<i64>,
    pub status: String,
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptAiVersionItem {
    pub id: i64,
    pub uuid: String,
    pub tenant_id: i64,
    pub organization_id: i64,
    pub prompt_id: i64,
    pub version_no: String,
    pub title: String,
    pub content: String,
    pub variable_schema: Value,
    pub output_schema: Value,
    pub model_constraints: Value,
    pub safety_policy: Value,
    pub examples_json: Value,
    pub checksum_hash: String,
    pub lifecycle_status: String,
    pub review_status: String,
    pub review_comment: Option<String>,
    pub created_by: i64,
    pub published_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptAiBindingItem {
    pub id: i64,
    pub uuid: String,
    pub tenant_id: i64,
    pub organization_id: i64,
    pub prompt_id: i64,
    pub prompt_version_id: Option<i64>,
    pub owner_type: String,
    pub owner_id: i64,
    pub binding_role: String,
    pub priority: i32,
    pub enabled: bool,
    pub policy_json: Value,
    pub snapshot_json: Value,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListPromptsQuery {
    pub subject: PromptAiSubject,
    pub keyword: Option<String>,
    pub prompt_type: Option<String>,
    pub visibility: Option<String>,
    pub status: Option<String>,
    pub category_id: Option<String>,
    pub page_no: i64,
    pub page_size: i64,
    pub offset: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatePromptCommand {
    pub subject: PromptAiSubject,
    pub prompt_key: String,
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<String>,
    pub prompt_type: String,
    pub visibility: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListPromptVersionsQuery {
    pub subject: PromptAiSubject,
    pub prompt_id: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreatePromptVersionCommand {
    pub subject: PromptAiSubject,
    pub prompt_id: i64,
    pub version_no: String,
    pub title: String,
    pub content: String,
    pub variable_schema: Value,
    pub output_schema: Value,
    pub model_constraints: Value,
    pub safety_policy: Value,
    pub examples_json: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublishPromptVersionCommand {
    pub subject: PromptAiSubject,
    pub version_id: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RenderPromptVersionCommand {
    pub subject: PromptAiSubject,
    pub version_id: i64,
    pub variables: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListPromptBindingsQuery {
    pub subject: PromptAiSubject,
    pub prompt_id: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreatePromptBindingCommand {
    pub subject: PromptAiSubject,
    pub prompt_id: i64,
    pub prompt_version_id: Option<i64>,
    pub owner_type: String,
    pub owner_id: i64,
    pub binding_role: String,
    pub priority: i32,
    pub enabled: bool,
    pub policy_json: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdatePromptBindingCommand {
    pub subject: PromptAiSubject,
    pub binding_id: i64,
    pub prompt_version_id: Option<Option<i64>>,
    pub owner_type: Option<String>,
    pub owner_id: Option<i64>,
    pub binding_role: Option<String>,
    pub priority: Option<i32>,
    pub enabled: Option<bool>,
    pub policy_json: Option<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdatePromptCommand {
    pub subject: PromptAiSubject,
    pub prompt_id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub status: Option<String>,
}
