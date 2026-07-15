export interface AdminPromptCreateRequest {
  promptKey: string;
  name: string;
  description?: string;
  categoryId?: string;
  promptType?: string;
  visibility?: string;
  tags?: string[];
}
