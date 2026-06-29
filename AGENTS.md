# SDKWork Prompts Agent Guide

This root follows the SDKWork standards in `../sdkwork-specs/README.md`.

Required references:
- `../sdkwork-specs/SOUL.md`
- `../sdkwork-specs/AGENTS_SPEC.md`
- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/APPLICATION_SPEC.md`
- `../sdkwork-specs/API_SPEC.md`
- `../sdkwork-specs/DATABASE_SPEC.md`
- `../sdkwork-specs/SDK_SPEC.md`
- `../sdkwork-specs/SDK_WORKSPACE_GENERATION_SPEC.md`
- `../sdkwork-specs/WEB_BACKEND_SPEC.md`
- `../sdkwork-specs/DOMAIN_SPEC.md`

Project rules:
- Canonical domain: `intelligence`.
- Capability: `prompts` only (`ai_` tables).
- Prompt table prefix: `ai_` per DATABASE_SPEC.md.
- Core tables: `ai_prompt_category`, `ai_prompt`, `ai_prompt_version`, `ai_prompt_binding`, `ai_prompt_template`, `ai_prompt_usage`.
- App API prefix: `/app/v3/api` with prompts routes under `/prompts/...`.
- Backend API prefix: `/backend/v3/api` with prompts routes under `/prompts/...`.
- Open API prefix: `/prompts/v3/api`.
- Open API public read operations must not declare SDKWork dual-token headers or custom business context headers.
- Generated SDK output under `sdks/**/generated/server-openapi` is generator-owned and must not be hand-edited.
- sdkwork-kernel MUST depend on `sdkwork-intelligence-prompts-ai-contract` for prompt ports; it MUST NOT own prompt table DDL.
- PC app under `apps/sdkwork-prompts-pc` consumes `@sdkwork/prompts-backend-sdk`; no raw HTTP or manual auth headers.

Implementation handoff:
- TODO comments must be precise and small enough for another agent to implement without guessing.
- Prompt schema changes must update `specs/prompts-ai-database.schema.yaml`, OpenAPI schemas, SDK authority files, database DDL, and tests together.
- API changes must update authored contracts under `apis/`, materialized SDK OpenAPI under `sdks/`, route manifests, and route crate descriptors together.

## Documentation Canon

- [docs/README.md](docs/README.md)
- [docs/product/prd/PRD.md](docs/product/prd/PRD.md)
- [docs/architecture/tech/TECH_ARCHITECTURE.md](docs/architecture/tech/TECH_ARCHITECTURE.md)
