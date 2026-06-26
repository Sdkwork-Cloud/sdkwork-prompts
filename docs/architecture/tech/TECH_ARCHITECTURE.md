# SDKWork Prompts Technical Architecture

Status: active  
Owner: prompts-platform  
Updated: 2026-06-26  
Specs: ARCHITECTURE_DECISION_SPEC.md, DOCUMENTATION_SPEC.md

## Document map

| Topic | Document |
| --- | --- |
| System boundaries | [TECH-prompts-architecture.md](TECH-prompts-architecture.md) |
| Database | [TECH-prompts-database-design.md](TECH-prompts-database-design.md) |
| HTTP APIs | [TECH-prompts-api-design.md](TECH-prompts-api-design.md) |
| Integrations | [TECH-prompts-integration-roadmap.md](TECH-prompts-integration-roadmap.md) |

## 1. Overview

`sdkwork-prompts` is the intelligence-domain **prompt registry**: definitions, versions, bindings, templates, and audit. Single capability `prompts`; six `ai_` tables; 18 HTTP routes across three API surfaces.

## 2. Technology

- Rust (Axum) API server + SQLx repository
- PostgreSQL primary store
- TypeScript composed SDK facades
- OpenAPI 3.1 contracts as authority

## 3. Deployment

- Binary: `sdkwork-prompts-api-server`
- Docker: `deployments/docker/docker-compose.yml` (API + Postgres)
- Topology: `specs/topology.spec.json`

## 4. Verification

```bash
cargo build --workspace
pnpm verify
node ../sdkwork-specs/tools/check-repository-docs-standard.mjs --root .
```
