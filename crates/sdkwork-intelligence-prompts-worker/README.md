# SDKWork Communication Prompts Worker

Worker for outbox publishing, search indexing, stats rebuilds, moderation policy evaluation, and notification fanout.

## Implementation Status

- `PromptsWorker<R: PromptsRepository>`: Wraps `PromptsService`, delegates job execution through service layer.
- `PromptsWorkerJob`: 5 variants - PublishOutbox, RebuildSearchProjection, RebuildStats, EvaluateModerationPolicy, FanoutNotifications.
- `run_prm_worker_once()`: Standalone entrypoint for single-job execution.
- Queue definitions: `jobs/queues/forum-worker.queues.yaml` with retry and dead-letter config.
- Schedule definitions: `jobs/schedules/forum-maintenance.schedule.yaml` with cron expressions.

Awaiting SQLx pool connection and real outbox poll/index/stat logic.
