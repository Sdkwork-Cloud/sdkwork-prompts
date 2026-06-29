#!/usr/bin/env node

import { existsSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const utilsRoot = join(repoRoot, "../sdkwork-utils/packages/sdkwork-utils-typescript");
const utilsDist = join(utilsRoot, "dist/index.js");

if (existsSync(utilsDist)) {
  process.stdout.write("workspace deps ok (@sdkwork/utils dist present)\n");
  process.exit(0);
}

process.stdout.write("building @sdkwork/utils dist for TypeScript consumers...\n");
const result = spawnSync(process.platform === "win32" ? "pnpm.cmd" : "pnpm", ["build"], {
  cwd: utilsRoot,
  stdio: "inherit",
  shell: process.platform === "win32",
});

if (result.status !== 0 || !existsSync(utilsDist)) {
  process.stderr.write("failed to build @sdkwork/utils; run: cd ../sdkwork-utils/packages/sdkwork-utils-typescript && pnpm build\n");
  process.exit(result.status ?? 1);
}

process.stdout.write("workspace deps ready\n");
