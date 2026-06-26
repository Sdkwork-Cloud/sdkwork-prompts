# Prompts Jobs

Background job contracts for `sdkwork-prompts` are reserved for future maintenance tasks (usage compaction, catalog projection rebuild).

Current release runs synchronously in the API server; no worker process is required for core prompt CRUD, publish, render, or binding flows.

When async jobs are introduced, define queues under `jobs/queues/` and implement consumers in a dedicated worker crate.
