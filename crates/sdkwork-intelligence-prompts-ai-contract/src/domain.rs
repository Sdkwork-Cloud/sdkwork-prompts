use serde::{Deserialize, Serialize};

/// Mirrors `ai_prompt` row semantics (see specs/prompts-ai-database.schema.yaml).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptRecord {
    pub id: i64,
    pub uuid: String,
    pub tenant_id: i64,
    pub organization_id: i64,
    pub prompt_key: String,
    pub name: String,
    pub description: Option<String>,
    pub prompt_type: String,
    pub visibility: String,
    pub latest_version_id: Option<i64>,
    pub published_version_id: Option<i64>,
    pub status: i32,
}

/// Mirrors `ai_prompt_version`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptVersionRecord {
    pub id: i64,
    pub uuid: String,
    pub tenant_id: i64,
    pub organization_id: i64,
    pub prompt_id: i64,
    pub version_no: String,
    pub title: Option<String>,
    pub content: String,
    pub lifecycle_status: String,
}

/// Mirrors `ai_prompt_binding`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptBindingRecord {
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
}

/// Mirrors `ai_prompt_template` (migrated from kernel `a_agent_prompt_template`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentPromptTemplateRecord {
    pub id: i64,
    pub uuid: String,
    pub tenant_id: i64,
    pub organization_id: i64,
    pub owner_user_id: i64,
    pub prompt_id: String,
    pub code: String,
    pub display_name: String,
    pub description: Option<String>,
    pub prompt_kind: AgentPromptTemplateKind,
    pub template_format: String,
    pub template_body: String,
    pub safety_profile_id: Option<String>,
    pub status: i32,
    pub visibility: i32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AgentPromptTemplateKind {
    System,
    Developer,
    User,
    Workflow,
    Tool,
    #[serde(rename = "mcp-prompt")]
    McpPrompt,
}
