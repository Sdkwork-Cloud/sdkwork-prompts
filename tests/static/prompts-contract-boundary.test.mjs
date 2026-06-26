import { existsSync } from "node:fs";
import { join } from "node:path";

const root = process.cwd();

if (!existsSync(join(root, "apps/sdkwork-prompts-pc"))) {
  console.error("missing boundary package: apps/sdkwork-prompts-pc");
  process.exit(1);
}

console.log("prompts contract boundary tests passed");
