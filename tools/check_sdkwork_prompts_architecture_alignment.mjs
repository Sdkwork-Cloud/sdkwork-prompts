#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const failures = [];

function assert(condition, message) {
  if (!condition) failures.push(message);
}

function readJson(relativePath) {
  const absolute = path.join(repoRoot, relativePath);
  if (!fs.existsSync(absolute)) {
    failures.push(`${relativePath} must exist`);
    return {};
  }
  return JSON.parse(fs.readFileSync(absolute, "utf8"));
}

function readText(relativePath) {
  const absolute = path.join(repoRoot, relativePath);
  if (!fs.existsSync(absolute)) {
    failures.push(`${relativePath} must exist`);
    return "";
  }
  return fs.readFileSync(absolute, "utf8");
}

for (const dir of [
  "apis",
  "apps",
  "crates",
  "sdks",
  "database",
  "deployments",
  "configs",
  "scripts",
  "docs",
  "tests",
  ".sdkwork",
  "specs",
]) {
  assert(fs.existsSync(path.join(repoRoot, dir)), `${dir}/ must exist`);
}

assert(fs.existsSync(path.join(repoRoot, "sdkwork.app.config.json")), "sdkwork.app.config.json must exist");
assert(fs.existsSync(path.join(repoRoot, "apps/sdkwork-prompts-pc")), "apps/sdkwork-prompts-pc must exist");

const packageJson = readJson("package.json");
for (const script of ["dev", "build", "test", "check", "verify", "clean"]) {
  assert(packageJson.scripts?.[script], `package.json must expose pnpm ${script}`);
}
assert(packageJson.scripts?.["check:architecture-alignment"], "check:architecture-alignment required");
assert(packageJson.scripts?.["db:validate"], "db:validate required");
assert(packageJson.scripts?.["api:materialize"], "api:materialize required");

function dependencyValues(pkg) {
  const values = [];
  for (const section of ["dependencies", "devDependencies", "optionalDependencies", "pnpm"]) {
    const block = pkg[section];
    if (!block || typeof block !== "object") {
      continue;
    }
    for (const value of Object.values(block)) {
      if (typeof value === "string") {
        values.push(value);
      }
    }
  }
  return values;
}

const forbiddenRuntimeRefs = ["clawrouter", "claw-router", "sdkwork-clawrouter"];
for (const value of dependencyValues(packageJson)) {
  for (const token of forbiddenRuntimeRefs) {
    assert(
      !value.toLowerCase().includes(token),
      `package.json must not declare a runtime dependency on ${token}`,
    );
  }
}

const cargoToml = readText("Cargo.toml");
for (const dep of [
  "sdkwork-web-core",
  "sdkwork-web-axum",
  "sdkwork-database-config",
  "sdkwork-database-sqlx",
  "sdkwork-utils-rust",
]) {
  assert(cargoToml.includes(dep), `Cargo.toml must declare ${dep}`);
}
assert(!cargoToml.includes("sdkwork-discovery"), "sdkwork-discovery deferred until RPC exists");

for (const openapiPath of [
  "apis/app-api/intelligence/prompts/openapi.yaml",
  "apis/backend-api/intelligence/prompts/openapi.yaml",
  "apis/open-api/intelligence/prompts/openapi.yaml",
]) {
  const openapi = readText(openapiPath);
  assert(
    !/PlusApiEnvelope|PlusApiResult/.test(openapi),
    `${openapiPath} must not declare legacy PlusApi envelopes`,
  );
}

for (const docPath of [
  "configs/local/.env.example",
  "configs/test/.env.test",
  "deployments/docker/README.md",
  "sdks/README.md",
  "crates/sdkwork-prompts-service-host/README.md",
]) {
  const text = readText(docPath);
  assert(!/\bprm_|\bbuild_prm|SDKWORK_CLAW_|forum-api/.test(text), `${docPath} must not reference legacy forum/prm surfaces`);
}

const componentSpec = readJson("specs/component.spec.json");
assert(componentSpec.component?.domain === "intelligence", "domain must be intelligence");
assert(componentSpec.component?.capability === "prompts", "capability must be prompts");

const dbManifest = readJson("database/database.manifest.json");
assert(dbManifest.tablePrefix === "ai_", "database tablePrefix must be ai_ for prompt contract module");
assert(dbManifest.moduleId === "prompts", "database moduleId must be prompts");

const aiSchema = readText("specs/prompts-ai-database.schema.yaml");
for (const table of [
  "ai_prompt",
  "ai_prompt_version",
  "ai_prompt_binding",
  "ai_prompt_template",
]) {
  assert(aiSchema.includes(table), `prompts-ai-database.schema.yaml must declare ${table}`);
}

const pcPackageJson = readJson("apps/sdkwork-prompts-pc/package.json");
for (const value of dependencyValues(pcPackageJson)) {
  for (const token of forbiddenRuntimeRefs) {
    assert(
      !value.toLowerCase().includes(token),
      `apps/sdkwork-prompts-pc/package.json must not declare a runtime dependency on ${token}`,
    );
  }
}

if (fs.existsSync(path.join(repoRoot, "tools/generators"))) {
  for (const entry of fs.readdirSync(path.join(repoRoot, "tools/generators"))) {
    if (!entry.endsWith(".mjs")) {
      continue;
    }
    const generator = readText(path.join("tools/generators", entry));
    for (const token of forbiddenRuntimeRefs) {
      assert(
        !generator.toLowerCase().includes(token),
        `tools/generators/${entry} must not reference ${token}`,
      );
    }
  }
}

if (failures.length) {
  process.stderr.write(`Architecture alignment failed:\n${failures.map((f) => `- ${f}`).join("\n")}\n`);
  process.exit(1);
}
process.stdout.write("Architecture alignment passed\n");
