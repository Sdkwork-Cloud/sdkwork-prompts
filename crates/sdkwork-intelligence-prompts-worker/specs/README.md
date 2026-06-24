# Worker Specs

Component spec for `sdkwork-intelligence-prompts-worker`.

- **Crate type**: Background job worker
- **Domain**: intelligence
- **Capability**: forum
- **Jobs**: PublishOutbox, RebuildSearchProjection, RebuildStats, EvaluateModerationPolicy, FanoutNotifications
- **Queues**: `jobs/queues/forum-worker.queues.yaml`
- **Schedules**: `jobs/schedules/forum-maintenance.schedule.yaml`
- **Implementation**: PromptsWorker wraps PromptsService. Jobs return Ok(()) pending SQLx pool connection.
