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

const cargoToml = readText("Cargo.toml");
for (const dep of [
  "sdkwork-web-core",
  "sdkwork-web-axum",
  "sdkwork-database-config",
  "sdkwork-database-sqlx",
  "sdkwork-utils-rust",
  "sdkwork-id-core",
]) {
  assert(cargoToml.includes(dep), `Cargo.toml must declare ${dep}`);
}
assert(!cargoToml.includes("sdkwork-discovery"), "sdkwork-discovery deferred until RPC exists");

const componentSpec = readJson("specs/component.spec.json");
assert(componentSpec.component?.domain === "intelligence", "domain must be intelligence");
assert(componentSpec.component?.capability === "prompts", "capability must be prompts");

const dbManifest = readJson("database/database.manifest.json");
assert(dbManifest.tablePrefix === "prm_", "database tablePrefix must be prm_");
assert(dbManifest.moduleId === "prompts", "database moduleId must be prompts");

const schema = readText("database/contract/schema.yaml");
for (const table of [
  "prm_template",
  "prm_template_version",
  "prm_template_variable",
]) {
  assert(schema.includes(table), `schema must declare ${table}`);
}

if (failures.length) {
  process.stderr.write(`Architecture alignment failed:\n${failures.map((f) => `- ${f}`).join("\n")}\n`);
  process.exit(1);
}
process.stdout.write("Architecture alignment passed\n");
