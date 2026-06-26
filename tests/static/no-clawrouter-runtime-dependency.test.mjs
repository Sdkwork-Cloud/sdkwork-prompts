import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

const root = process.cwd();
const forbidden = ["clawrouter", "claw-router", "sdkwork-clawrouter"];

function fail(message) {
  console.error(message);
  process.exit(1);
}

function dependencyValues(pkg) {
  const values = [];
  for (const section of ["dependencies", "devDependencies", "optionalDependencies"]) {
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

function scanDependencies(relativePath) {
  const absolute = join(root, relativePath);
  if (!existsSync(absolute)) {
    return;
  }
  const pkg = JSON.parse(readFileSync(absolute, "utf8"));
  for (const value of dependencyValues(pkg)) {
    for (const token of forbidden) {
      if (value.toLowerCase().includes(token)) {
        fail(`${relativePath} must not declare a runtime dependency on ${token}`);
      }
    }
  }
}

for (const relativePath of [
  "package.json",
  "apps/sdkwork-prompts-pc/package.json",
  "apps/sdkwork-prompts-pc/packages/sdkwork-prompts-pc-commons/package.json",
  "apps/sdkwork-prompts-pc/packages/sdkwork-prompts-pc-admin-prompts/package.json",
]) {
  scanDependencies(relativePath);
}

const generatorDir = join(root, "tools/generators");
if (existsSync(generatorDir)) {
  for (const entry of readdirSync(generatorDir)) {
    if (!entry.endsWith(".mjs")) {
      continue;
    }
    const text = readFileSync(join(generatorDir, entry), "utf8").toLowerCase();
    for (const token of forbidden) {
      if (text.includes(token)) {
        fail(`tools/generators/${entry} must not reference ${token}`);
      }
    }
  }
}

console.log("no clawrouter runtime dependency checks passed");
