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
- Capability: `forum`.
- Database table prefix: `prm_`.
- Public forum resource names are `topic` and `reply`.
- Do not use the term `thread` in table names, API paths, SDK resources, route manifests, or public method names.
- App API prefix: `/app/v3/api/forum`.
- Backend API prefix: `/backend/v3/api/forum`.
- Open API prefix: `/prompts/v3/api`.
- Open API public read operations must not declare SDKWork dual-token headers or custom business context headers.
- Generated SDK output under `sdks/**/generated/server-openapi` is generator-owned and must not be hand-edited.
- App/frontend implementation under `apps/` is out of scope for this foundation task.

Implementation handoff:
- TODO comments must be precise and small enough for another agent to implement without guessing.
- Database schema changes must update `specs/forum-database.schema.yaml`, OpenAPI schemas, SDK authority files, and tests together.
- API changes must update authored contracts under `apis/`, materialized SDK OpenAPI under `sdks/`, route manifests, and route crate descriptors together.

## Documentation Canon

- [docs/README.md](docs/README.md)
- [docs/product/PRD.md](docs/product/PRD.md)
- [docs/architecture/TECH_ARCHITECTURE.md](docs/architecture/TECH_ARCHITECTURE.md)
