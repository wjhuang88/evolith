export interface CliInterface {
  id: string;
  name: string;
  language: string;
  framework?: string;
  tags: string[];
  content: string;
  code: string;
  dependencies: Array<{ name: string; version: string; required: boolean }>;
  estimatedTokens: number;
  visibility: 'public' | 'private';
  owner: {
    id: string;
    username: string;
  };
  createdAt: string;
  updatedAt: string;
}

export interface CliInterfaceFilter {
  search?: string;
  language?: string;
  framework?: string;
  tags?: string[];
  page?: number;
  perPage?: number;
}
