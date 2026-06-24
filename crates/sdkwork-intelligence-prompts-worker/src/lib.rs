use sdkwork_intelligence_prompts_service::PromptsService;
use sdkwork_intelligence_prompts_service::domain::commands::{
    FanoutNotificationsCommand, ListModerationQueueCommand, PublishOutboxCommand,
    RebuildSearchProjectionCommand, RebuildStatsCommand,
};
use sdkwork_intelligence_prompts_service::ports::repository::PromptsRepository;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptsWorkerJob {
    PublishOutbox,
    RebuildSearchProjection,
    RebuildStats,
    EvaluateModerationPolicy,
    FanoutNotifications,
}

pub struct PromptsWorker<R: PromptsRepository> {
    service: PromptsService<R>,
}

impl<R: PromptsRepository> PromptsWorker<R> {
    pub fn new(service: PromptsService<R>) -> Self {
        Self { service }
    }

    pub fn run_job(
        &self,
        job: PromptsWorkerJob,
        ctx: &sdkwork_intelligence_prompts_service::value_objects::PromptsRequestContext,
    ) -> Result<(), String> {
        match job {
            PromptsWorkerJob::PublishOutbox => self.publish_outbox(ctx),
            PromptsWorkerJob::RebuildSearchProjection => self.rebuild_search_projection(ctx),
            PromptsWorkerJob::RebuildStats => self.rebuild_stats(ctx),
            PromptsWorkerJob::EvaluateModerationPolicy => self.evaluate_moderation_policy(ctx),
            PromptsWorkerJob::FanoutNotifications => self.fanout_notifications(ctx),
        }
    }

    fn publish_outbox(
        &self,
        ctx: &sdkwork_intelligence_prompts_service::value_objects::PromptsRequestContext,
    ) -> Result<(), String> {
        self.service
            .publish_pending_outbox(ctx, PublishOutboxCommand { limit: 100 })
            .map(|_| ())
            .map_err(|error| error.to_string())
    }

    fn rebuild_search_projection(
        &self,
        ctx: &sdkwork_intelligence_prompts_service::value_objects::PromptsRequestContext,
    ) -> Result<(), String> {
        self.service
            .rebuild_search_projection(
                ctx,
                RebuildSearchProjectionCommand {
                    scope: None,
                    board_id: None,
                },
            )
            .map(|_| ())
            .map_err(|error| error.to_string())
    }

    fn rebuild_stats(
        &self,
        ctx: &sdkwork_intelligence_prompts_service::value_objects::PromptsRequestContext,
    ) -> Result<(), String> {
        self.service
            .rebuild_stats(
                ctx,
                RebuildStatsCommand {
                    scope: Some("all".to_string()),
                },
            )
            .map(|_| ())
            .map_err(|error| error.to_string())
    }

    fn evaluate_moderation_policy(
        &self,
        ctx: &sdkwork_intelligence_prompts_service::value_objects::PromptsRequestContext,
    ) -> Result<(), String> {
        self.service
            .list_moderation_queue(
                ctx,
                ListModerationQueueCommand {
                    status_filter: Some("pending".to_string()),
                    severity_filter: None,
                    cursor: None,
                    limit: 100,
                },
            )
            .map(|_| ())
            .map_err(|error| error.to_string())
    }

    fn fanout_notifications(
        &self,
        ctx: &sdkwork_intelligence_prompts_service::value_objects::PromptsRequestContext,
    ) -> Result<(), String> {
        self.service
            .fanout_notifications(ctx, FanoutNotificationsCommand { limit: 100 })
            .map(|_| ())
            .map_err(|error| error.to_string())
    }
}
