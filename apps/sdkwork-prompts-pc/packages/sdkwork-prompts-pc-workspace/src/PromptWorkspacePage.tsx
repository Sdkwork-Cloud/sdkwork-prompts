import { useEffect, useMemo, useState } from "react";
import { isBlank } from "@sdkwork/utils/string";
import type { AdminPromptItem } from "@sdkwork/prompts-backend-sdk";
import { listPrompts } from "@sdkwork/prompts-pc-admin-prompts";

export function PromptWorkspacePage() {
  const subtitle = useMemo(() => {
    const env = import.meta.env.VITE_SDKWORK_PROMPTS_ENV ?? "development";
    return isBlank(env) ? "development" : env;
  }, []);

  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [items, setItems] = useState<AdminPromptItem[]>([]);

  useEffect(() => {
    let active = true;
    listPrompts()
      .then((promptItems) => {
        if (!active) {
          return;
        }
        setItems(promptItems);
        setError(null);
      })
      .catch((cause: unknown) => {
        if (!active) {
          return;
        }
        setError(cause instanceof Error ? cause.message : "Failed to load prompts");
        setItems([]);
      })
      .finally(() => {
        if (active) {
          setLoading(false);
        }
      });
    return () => {
      active = false;
    };
  }, []);

  return (
    <section>
      <h1>Prompt Workspace</h1>
      <p>
        Manage prompt definitions, versions, and bindings through the sdkwork-prompts backend SDK.
      </p>
      <p>
        Environment: <code>{subtitle}</code>
      </p>
      {loading ? <p>Loading prompts…</p> : null}
      {error ? (
        <p role="alert">
          Prompt list unavailable: {error}. Start <code>sdkwork-prompts-standalone-gateway</code> on port
          8080 or set <code>VITE_SDKWORK_PROMPTS_API_BASE_URL</code>.
        </p>
      ) : null}
      {!loading && !error ? (
        <ul>
          {items.length === 0 ? (
            <li>No prompts yet.</li>
          ) : (
            items.map((item) => (
              <li key={item.id ?? item.promptKey}>
                <strong>{item.name ?? item.promptKey}</strong>
                {item.promptType ? ` · ${item.promptType}` : null}
              </li>
            ))
          )}
        </ul>
      ) : null}
    </section>
  );
}
