# SDKWork Prompts Service Host

Dependency composition for forum service runtime.

## Implementation Status

- `PromptsServiceHost`: Wraps `PromptsService<SqlxPromptsRepository>` with `service()` accessor and `build_request_context()` factory.
- `build_prm_service()`: Constructs `PromptsService<SqlxPromptsRepository>` with placeholder repository.

Awaiting SQLx pool injection, Drive/Search/Notification adapter wiring, and appbase context integration.
