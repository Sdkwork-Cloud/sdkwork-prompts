import { useEffect, useMemo, useState } from "react";
import { isBlank } from "@sdkwork/utils/string";
import { listPrompts } from "@sdkwork/prompts-pc-admin-prompts";

type PromptListData = {
  items?: Array<{ promptKey?: string; name?: string; promptType?: string }>;
};

export function PromptWorkspacePage() {
  const subtitle = useMemo(() => {
    const env = import.meta.env.VITE_SDKWORK_PROMPTS_ENV ?? "development";
    return isBlank(env) ? "development" : env;
  }, []);

  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [items, setItems] = useState<PromptListData["items"]>([]);

  useEffect(() => {
    let active = true;
    listPrompts()
      .then((result) => {
        if (!active) {
          return;
        }
        const data = (result.data ?? {}) as PromptListData;
        setItems(Array.isArray(data.items) ? data.items : []);
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
        Manage prompt definitions, versions, and bindings through the sdkwork-prompts backend
        SDK. This foundation app does not depend on clawrouter.
      </p>
      <p>
        Environment: <code>{subtitle}</code>
      </p>
      {loading ? <p>Loading prompts…</p> : null}
      {error ? (
        <p role="alert">
          Prompt list unavailable: {error}. Start <code>sdkwork-prompts-api-server</code> on port
          8080 or set <code>VITE_SDKWORK_PROMPTS_API_BASE_URL</code>.
        </p>
      ) : null}
      {!loading && !error ? (
        <ul>
          {(items ?? []).length === 0 ? (
            <li>No prompts yet.</li>
          ) : (
            (items ?? []).map((item) => (
              <li key={`${item.promptKey ?? item.name}`}>
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
