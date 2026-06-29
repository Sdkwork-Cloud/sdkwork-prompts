# Prompts SDK tests

```bash
node tests/sdk/prompts-sdk.test.mjs
```

Checks:

- sdkgen configuration for app, backend, and open SDK families
- Route manifest alignment
- Composed facade exports (no stub throws)

Also included in `pnpm test`.
