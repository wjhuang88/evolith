export interface Skill {
  id: string;
  name: string;
  version: string;
  description: string;
  skillMd: string;
  runtime: 'python311' | 'node20' | 'wasm';
  dependencies: Array<{ name: string; version: string }>;
  visibility: 'public' | 'private';
  owner: {
    id: string;
    username: string;
  };
  createdAt: string;
  updatedAt: string;
}

export interface SkillFilter {
  search?: string;
  runtime?: 'python311' | 'node20' | 'wasm';
  visibility?: 'public' | 'private';
  page?: number;
  perPage?: number;
}
