//! Prompt AI contract — ports and DTOs for sdkwork-kernel and sdkwork-prompts.

pub mod commands;
pub mod domain;
pub mod error;
pub mod ports;

pub use commands::{
    CreatePromptBindingCommand, CreatePromptCommand, CreatePromptVersionCommand,
    ListPromptBindingsQuery, ListPromptsQuery, ListPromptVersionsQuery, PromptAiBindingItem,
    PromptAiItem, PromptAiSubject, PromptAiVersionItem, PublishPromptVersionCommand,
    RenderPromptVersionCommand, UpdatePromptBindingCommand, UpdatePromptCommand,
};
pub use domain::{
    AgentPromptTemplateKind, AgentPromptTemplateRecord, PromptBindingRecord, PromptRecord,
    PromptVersionRecord,
};
pub use error::{PromptAiError, PromptAiResult};
pub use ports::{AgentPromptTemplateListQuery, PromptAiRepository, PromptListQuery};

#[cfg(test)]
mod tests {
    use crate::domain::{AgentPromptTemplateKind, AgentPromptTemplateRecord};

    #[test]
    fn agent_prompt_template_kind_serializes_kebab_case() {
        let kind = AgentPromptTemplateKind::McpPrompt;
        let json = serde_json::to_string(&kind).expect("serialize");
        assert_eq!(json, "\"mcp-prompt\"");
    }

    #[test]
    fn agent_prompt_template_record_roundtrip() {
        let record = AgentPromptTemplateRecord {
            id: 1,
            uuid: "uuid-1".into(),
            tenant_id: 100001,
            organization_id: 0,
            owner_user_id: 42,
            prompt_id: "prompt.demo".into(),
            code: "demo".into(),
            display_name: "Demo".into(),
            description: None,
            prompt_kind: AgentPromptTemplateKind::System,
            template_format: "plain-text".into(),
            template_body: "Hello".into(),
            safety_profile_id: None,
            status: 1,
            visibility: 1,
        };
        let json = serde_json::to_string(&record).expect("serialize");
        let parsed: AgentPromptTemplateRecord = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(record, parsed);
    }
}
