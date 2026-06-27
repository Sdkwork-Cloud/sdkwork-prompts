import { readFileSync } from "node:fs";
import { join } from "node:path";

const root = process.cwd();
const iamSource = readFileSync(
  join(root, "crates/sdkwork-prompts-standalone-gateway/src/iam.rs"),
  "utf8",
);

if (iamSource.includes("/app/v3/api/forum") || iamSource.includes("/backend/v3/api/forum")) {
  console.error("IAM must resolve /prompts routes, not legacy /forum paths");
  process.exit(1);
}

if (!iamSource.includes("/app/v3/api/prompts") || !iamSource.includes("/backend/v3/api/prompts")) {
  console.error("IAM must protect app/backend /prompts routes");
  process.exit(1);
}

if (!iamSource.includes("/prompts/v3/api")) {
  console.error("IAM must exempt open /prompts/v3/api routes");
  process.exit(1);
}

console.log("prompts IAM path alignment checks passed");
