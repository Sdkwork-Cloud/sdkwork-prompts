export interface AgentPromptTemplate {
  id: string;
  uuid?: string;
  promptId?: string;
  code: string;
  displayName: string;
  description?: string | null;
  promptKind: string;
  templateFormat: string;
  templateBody?: string;
  safetyProfileId?: string | null;
  status: number;
  visibility?: number;
}
