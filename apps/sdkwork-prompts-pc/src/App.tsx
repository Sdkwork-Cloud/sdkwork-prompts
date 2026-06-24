import { Routes, Route, Link } from "react-router-dom";
import { PromptWorkspacePage } from "../packages/sdkwork-prompts-pc-workspace/src/PromptWorkspacePage";

export function App() {
  return (
    <div className="app-shell">
      <header className="app-header">
        <Link to="/">SDKWork Prompts</Link>
      </header>
      <main className="app-main">
        <Routes>
          <Route path="/" element={<PromptWorkspacePage />} />
        </Routes>
      </main>
    </div>
  );
}
