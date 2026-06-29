# SDKWork Prompts PRD

Status: active  
Owner: prompts-platform  
Application: sdkwork-prompts  
Updated: 2026-06-29  
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
- Bind prompts to agents, workflows, and scenes
- Agent template catalog (`ai_prompt_template`)
- Public read-only catalog for published prompts

## 4. Non-goals

- Community forum or moderation features
- LLM execution or routing
- Dedicated search platform for v1 (DB indexes suffice)

## 5. Success metrics

- `pnpm verify` green on every merge
- Kernel depends only on `sdkwork-intelligence-prompts-ai-contract`
- All HTTP surfaces use `SdkWorkApiResponse` / `ProblemDetail` per API_SPEC

## 6. Linked requirements

- [REQ-2026-0001-prompt-registry.md](../requirements/REQ-2026-0001-prompt-registry.md)

## 7. Verification

```bash
pnpm verify
```
