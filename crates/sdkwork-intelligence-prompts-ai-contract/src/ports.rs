use async_trait::async_trait;

use crate::commands::{
    CreatePromptBindingCommand, CreatePromptCommand, CreatePromptVersionCommand,
    ListPromptBindingsQuery, ListPromptVersionsQuery, ListPromptsQuery, PromptAiBindingItem,
    PromptAiItem, PromptAiVersionItem, PublishPromptVersionCommand, RenderPromptVersionCommand,
    UpdatePromptBindingCommand, UpdatePromptCommand,
};
use crate::domain::{
    AgentPromptTemplateRecord, PromptBindingRecord, PromptRecord, PromptVersionRecord,
};
use crate::error::{PromptAiError, PromptAiResult};

pub use crate::commands::PromptAiSubject;

#[derive(Debug, Clone)]
pub struct PromptListQuery {
    pub tenant_id: i64,
    pub organization_id: i64,
    pub prompt_type: Option<String>,
    pub limit: u32,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AgentPromptTemplateListQuery {
    pub tenant_id: i64,
    pub organization_id: i64,
    pub limit: u32,
}

/// Admin/backend prompt AI persistence port (ai_prompt, ai_prompt_version, ai_prompt_binding).
#[async_trait]
pub trait PromptAiRepository: Send + Sync {
    async fn list_prompts(&self, query: ListPromptsQuery) -> PromptAiResult<Vec<PromptAiItem>>;

    async fn create_prompt(&self, command: CreatePromptCommand) -> PromptAiResult<PromptAiItem>;

    async fn update_prompt(&self, command: UpdatePromptCommand) -> PromptAiResult<PromptAiItem>;

    async fn list_versions(
        &self,
        query: ListPromptVersionsQuery,
    ) -> PromptAiResult<Vec<PromptAiVersionItem>>;

    async fn create_version(
        &self,
        command: CreatePromptVersionCommand,
    ) -> PromptAiResult<PromptAiVersionItem>;

    async fn publish_version(
        &self,
        command: PublishPromptVersionCommand,
    ) -> PromptAiResult<Option<PromptAiVersionItem>>;

    async fn render_version(
        &self,
        command: RenderPromptVersionCommand,
    ) -> PromptAiResult<Option<String>>;

    async fn list_bindings(
        &self,
        query: ListPromptBindingsQuery,
    ) -> PromptAiResult<Vec<PromptAiBindingItem>>;

    async fn create_binding(
        &self,
        command: CreatePromptBindingCommand,
    ) -> PromptAiResult<PromptAiBindingItem>;

    async fn update_binding(
        &self,
        command: UpdatePromptBindingCommand,
    ) -> PromptAiResult<Option<PromptAiBindingItem>>;

    async fn get_prompt(&self, tenant_id: i64, id: i64) -> PromptAiResult<PromptRecord>;

    async fn get_prompt_version(
        &self,
        tenant_id: i64,
        id: i64,
    ) -> PromptAiResult<PromptVersionRecord>;

    async fn list_bindings_for_owner(
        &self,
        tenant_id: i64,
        organization_id: i64,
        owner_type: &str,
        owner_id: i64,
    ) -> PromptAiResult<Vec<PromptBindingRecord>>;

    async fn get_agent_prompt_template(
        &self,
        tenant_id: i64,
        id: i64,
    ) -> PromptAiResult<AgentPromptTemplateRecord>;

    async fn list_agent_prompt_templates(
        &self,
        query: AgentPromptTemplateListQuery,
    ) -> PromptAiResult<Vec<AgentPromptTemplateRecord>>;
}

impl PromptAiError {
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound(message.into())
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::Conflict(message.into())
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }
}
