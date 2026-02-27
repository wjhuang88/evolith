// ============================================
// Types
// ============================================

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  meta?: PaginationMeta;
  error?: ErrorInfo;
}

export interface ErrorInfo {
  code: string;
  message: string;
  retryable?: boolean;
  details?: Record<string, unknown>;
}

export interface PaginationMeta {
  page: number;
  per_page: number;
  total: number;
  total_pages: number;
}

// ============================================
// Auth Types
// ============================================

export interface User {
  id: string;
  email: string;
  username: string;
  role: string;
  created_at?: string;
}

export interface AuthToken {
  token: string;
  expires_at: number;
  user: User;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface RegisterRequest {
  email: string;
  username: string;
  password: string;
}

// ============================================
// Tool Types
// ============================================

export interface Tool {
  id: string;
  name: string;
  description: string;
  category: string;
  schema: Record<string, unknown>;
  is_public: boolean;
  created_by?: string;
  created_at?: string;
  updated_at?: string;
}

export interface CreateToolRequest {
  name: string;
  description: string;
  category: string;
  schema: Record<string, unknown>;
  is_public?: boolean;
}

export interface UpdateToolRequest extends Partial<CreateToolRequest> {}

// ============================================
// Skill Types
// ============================================

export interface Skill {
  id: string;
  name: string;
  description: string;
  version: string;
  content: string;
  category: string;
  tags: string[];
  is_public: boolean;
  created_by?: string;
  created_at?: string;
  updated_at?: string;
}

export interface CreateSkillRequest {
  name: string;
  description: string;
  version: string;
  content: string;
  category: string;
  tags?: string[];
  is_public?: boolean;
}

export interface UpdateSkillRequest extends Partial<CreateSkillRequest> {}

// ============================================
// Snippet Types
// ============================================

export interface Snippet {
  id: string;
  title: string;
  description: string;
  code: string;
  language: string;
  category: string;
  tags: string[];
  is_public: boolean;
  created_by?: string;
  created_at?: string;
  updated_at?: string;
}

export interface CreateSnippetRequest {
  title: string;
  description: string;
  code: string;
  language: string;
  category: string;
  tags?: string[];
  is_public?: boolean;
}

export interface UpdateSnippetRequest extends Partial<CreateSnippetRequest> {}

// ============================================
// Query Options
// ============================================

export interface ListQueryParams {
  page?: number;
  per_page?: number;
  search?: string;
  category?: string;
  sort_by?: string;
  sort_order?: 'asc' | 'desc';
}

export interface ListResponse<T> {
  items: T[];
  meta: PaginationMeta;
}
