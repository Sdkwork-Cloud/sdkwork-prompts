# SDKWork Prompts PC

Browser/desktop prompt workspace for SDKWork Prompts (intelligence / prompts).

## Packages

| Package | Role |
| --- | --- |
| `sdkwork-prompts-pc-core` | Host composition, session, SDK inventory |
| `sdkwork-prompts-pc-commons` | Backend SDK client wiring, `@sdkwork/utils` helpers |
| `sdkwork-prompts-pc-admin-prompts` | Admin prompt service (definitions, versions, bindings) |
| `sdkwork-prompts-pc-workspace` | Prompt workspace UI |

## Development

```bash
pnpm install
cp ../../configs/local/.env.example .env.local   # optional API base URL
pnpm dev
```

Set `VITE_SDKWORK_PROMPTS_API_BASE_URL` to the standalone gateway (default `http://localhost:8080`).

## Verification

```bash
pnpm typecheck
pnpm test
```

Repository-wide checks: `pnpm verify` from `sdkwork-prompts` root.

## Documentation

See [repository docs](../../../docs/README.md).
