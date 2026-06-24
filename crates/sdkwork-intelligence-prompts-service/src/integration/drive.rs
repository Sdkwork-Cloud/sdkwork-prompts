pub trait PromptsDrivePort: Send + Sync {
    fn validate_media_reference(&self, media_resource_id: &str) -> Result<(), String>;
    fn create_download_grant(&self, media_resource_id: &str) -> Result<String, String>;
}

pub struct NoopPromptsDrivePort;

impl PromptsDrivePort for NoopPromptsDrivePort {
    fn validate_media_reference(&self, _media_resource_id: &str) -> Result<(), String> {
        Ok(())
    }

    fn create_download_grant(&self, media_resource_id: &str) -> Result<String, String> {
        Ok(format!("noop-grant:{media_resource_id}"))
    }
}

pub struct LoggingPromptsDrivePort;

impl PromptsDrivePort for LoggingPromptsDrivePort {
    fn validate_media_reference(&self, media_resource_id: &str) -> Result<(), String> {
        eprintln!("[forum-drive] validate media reference: {media_resource_id}");
        Ok(())
    }

    fn create_download_grant(&self, media_resource_id: &str) -> Result<String, String> {
        let grant_id = format!("grant:{media_resource_id}");
        eprintln!("[forum-drive] created download grant: {grant_id}");
        Ok(grant_id)
    }
}
