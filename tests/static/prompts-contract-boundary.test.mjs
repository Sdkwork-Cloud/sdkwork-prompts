import { existsSync } from "node:fs";
import { join } from "node:path";

const root = process.cwd();
const boundaries = [
  "crates/sdkwork-routes-prompts-app-api",
  "crates/sdkwork-intelligence-prompts-service",
  "apps/sdkwork-prompts-pc",
];

for (const path of boundaries) {
  if (!existsSync(join(root, path))) {
    console.error(`missing boundary package: ${path}`);
    process.exit(1);
  }
}

console.log("prompts contract boundary tests passed");
