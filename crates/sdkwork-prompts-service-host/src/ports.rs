use sdkwork_intelligence_prompts_service::integration::drive::{PromptsDrivePort, LoggingPromptsDrivePort, NoopPromptsDrivePort};
use sdkwork_intelligence_prompts_service::integration::notifications::{
    PromptsNotificationPort, HttpPromptsNotificationPort, LoggingPromptsNotificationPort,
    NoopPromptsNotificationPort,
};
use sdkwork_intelligence_prompts_service::integration::search::{
    PromptsSearchPort, HttpPromptsSearchPort, LoggingPromptsSearchPort, NoopPromptsSearchPort,
};

pub fn build_drive_port() -> Box<dyn PromptsDrivePort> {
    match std::env::var("SDKWORK_PROMPTS_DRIVE_URL") {
        Ok(url) if !url.trim().is_empty() => Box::new(LoggingPromptsDrivePort),
        _ => Box::new(NoopPromptsDrivePort),
    }
}

pub fn build_search_port() -> Box<dyn PromptsSearchPort> {
    match std::env::var("SDKWORK_PROMPTS_SEARCH_URL") {
        Ok(url) if !url.trim().is_empty() => Box::new(HttpPromptsSearchPort::configured(
            url,
            std::env::var("SDKWORK_PROMPTS_SEARCH_INDEX_ID").unwrap_or_else(|_| "prompts".to_string()),
            None,
            std::env::var("SDKWORK_ACCESS_TOKEN").ok(),
        )),
        _ if logging_ports_enabled() => Box::new(LoggingPromptsSearchPort),
        _ => Box::new(NoopPromptsSearchPort),
    }
}

pub fn build_notification_port() -> Box<dyn PromptsNotificationPort> {
    match std::env::var("SDKWORK_PROMPTS_NOTIFICATION_URL") {
        Ok(url) if !url.trim().is_empty() => Box::new(HttpPromptsNotificationPort::new(url)),
        _ if logging_ports_enabled() => Box::new(LoggingPromptsNotificationPort),
        _ => Box::new(NoopPromptsNotificationPort),
    }
}

fn logging_ports_enabled() -> bool {
    matches!(
        std::env::var("SDKWORK_PROMPTS_INTEGRATION_LOG").as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE")
    )
}
