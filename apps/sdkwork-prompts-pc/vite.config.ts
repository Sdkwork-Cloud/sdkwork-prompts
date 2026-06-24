import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "node:path";

export default defineConfig({
  plugins: [react()],
  root: ".",
  resolve: {
    alias: {
      "@sdkwork/prompts-pc-workspace": path.resolve(
        __dirname,
        "packages/sdkwork-prompts-pc-workspace/src",
      ),
    },
  },
  server: {
    port: 5175,
  },
});
