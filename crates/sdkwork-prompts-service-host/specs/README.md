# Host Specs

Component spec for `sdkwork-prompts-service-host`.

- **Crate type**: In-process service host
- **Domain**: intelligence
- **Capability**: forum
- **Public exports**: PromptsServiceHost, build_prm_service()
- **Dependencies**: sdkwork-intelligence-prompts-repository-sqlx, sdkwork-intelligence-prompts-service
- **Implementation**: PromptsServiceHost wraps PromptsService with placeholder repository.
