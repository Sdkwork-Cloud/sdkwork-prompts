# SDKWork Prompts PRD

Status: active  
Owner: prompts-platform  
Application: sdkwork-prompts  
Updated: 2026-06-26  
Specs: REQUIREMENTS_SPEC.md, DOCUMENTATION_SPEC.md

## 1. Problem

Prompt definitions are scattered across services with inconsistent table prefixes and no single governance surface. Operators and apps need one registry for versioned prompts, bindings, and catalog discovery.

## 2. Users

- Platform operators (backend API)
- Application developers (app API + SDK)
- Agent runtime (kernel via contract ports)
- Public integrators (open catalog)

## 3. Goals

- Single owner for `ai_` prompt tables in sdkwork-prompts
- Versioned, publishable prompt definitions with render support
- Bind prompts to agents/workflows/scenes
- Marketplace template catalog for agent templates
- Public read-only catalog for published prompts

## 4. Non-goals

- Forum, topics, replies, moderation
- LLM execution or routing
- Full-text search platform (use DB indexes for v1)

## 5. Success metrics

- `pnpm verify` green on every merge
- Kernel depends only on `sdkwork-intelligence-prompts-ai-contract`
- Zero `prm_` / forum routes in manifests and runtime

## 6. Linked requirements

- [REQ-2026-0001-prompt-tables-migration.md](../requirements/REQ-2026-0001-prompt-tables-migration.md)

## 7. Verification

```bash
pnpm verify
```
