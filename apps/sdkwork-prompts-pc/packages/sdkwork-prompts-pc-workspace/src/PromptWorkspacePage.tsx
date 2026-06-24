import { useMemo } from "react";
import { isBlank } from "@sdkwork/utils/string";

export function PromptWorkspacePage() {
  const subtitle = useMemo(() => {
    const env = import.meta.env.VITE_SDKWORK_PROMPTS_ENV ?? "development";
    return isBlank(env) ? "development" : env;
  }, []);

  return (
    <section>
      <h1>Prompt Workspace</h1>
      <p>
        Manage prompt templates, versions, and variables for the intelligence prompts domain.
      </p>
      <p>
        Environment: <code>{subtitle}</code>
      </p>
    </section>
  );
}
