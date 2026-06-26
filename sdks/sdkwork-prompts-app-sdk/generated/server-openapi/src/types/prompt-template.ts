export interface PromptTemplate {
  id: string;
  key: string;
  name: string;
  description?: string;
  status: 'draft' | 'active' | 'archived';
  tags?: string[];
  latest_version_id?: string | null;
  updated_at?: string;
}
