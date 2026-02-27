export interface Tool {
  id: string;
  name: string;
  description: string;
  inputSchema: Record<string, unknown>;
  outputSchema?: Record<string, unknown>;
  handler: {
    type: 'http' | 'function';
    url?: string;
    method?: string;
    timeout?: number;
  };
  visibility: 'public' | 'private';
  owner: {
    id: string;
    username: string;
  };
  createdAt: string;
  updatedAt: string;
}

export interface ToolFilter {
  search?: string;
  visibility?: 'public' | 'private';
  page?: number;
  perPage?: number;
}
