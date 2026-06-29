import { readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";

const root = process.cwd();
const failures = [];

const forbiddenPatterns = [
  /\bprm_[a-z_]+\b/,
  /\bforum-database\.schema\.yaml\b/,
  /\bcompose_prm_api_routes\b/,
  /\bbuild_prm_routes\b/,
  /\bbuild_prm_service\b/,
  /\bai_agent_prompt_template\b/,
  /\bai_prompt_usage_event\b/,
  /\/app\/v3\/api\/forum\b/,
  /\/backend\/v3\/api\/forum\b/,
  /\bforum-api\b/,
  /\bforum-worker\b/,
  /SDKWORK_CLAW_/,
];

const scanRoots = [
  "AGENTS.md",
  "specs",
  "database",
  "crates",
  "configs",
  "apps/sdkwork-prompts-pc/specs",
  "apps/sdkwork-prompts-pc/README.md",
  "sdks/README.md",
  "sdks/test/README.md",
  "sdks/_route-manifests",
  "apis",
  "deployments",
  "jobs",
  "sdkwork.app.config.json",
  "package.json",
  "Cargo.toml",
  "README.md",
];

const skipPaths = [
  "specs/prompts-ai-database.schema.yaml",
  "tests/static/no-forum-debt.test.mjs",
  ".env.postgres.example",
  "docs/archive",
  "docs/changelogs",
  "docs/migrations",
  "docs/product/requirements",
  "target",
  "node_modules",
];

function shouldSkip(relativePath) {
  return skipPaths.some((skip) => relativePath.replace(/\\/g, "/").includes(skip));
}

function scanFile(relativePath) {
  if (shouldSkip(relativePath)) return;
  const text = readFileSync(join(root, relativePath), "utf8");
  for (const pattern of forbiddenPatterns) {
    if (pattern.test(text)) {
      failures.push(`${relativePath} matches forbidden ${pattern}`);
    }
  }
}

function walk(relativeDir) {
  const absolute = join(root, relativeDir);
  let stat;
  try {
    stat = statSync(absolute);
  } catch {
    return;
  }
  if (stat.isFile()) {
    scanFile(relativeDir);
    return;
  }
  for (const entry of readdirSync(absolute)) {
    walk(join(relativeDir, entry));
  }
}

for (const entry of scanRoots) {
  walk(entry);
}

if (failures.length > 0) {
  console.error("forum/prm debt detected:");
  for (const failure of failures) console.error(`  - ${failure}`);
  process.exit(1);
}

console.log("no forum/prm debt checks passed");
