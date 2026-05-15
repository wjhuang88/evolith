import { apiClient, buildQueryString } from './client';
import type {
  ApiResponse,
  PaginatedResponse,
  Snippet,
  CreateSnippetRequest,
  UpdateSnippetRequest,
  ListQueryParams,
} from './types';

// ============================================
// Snippets Service
// ============================================

type BackendSnippetRequest = {
  name: string;
  language: string;
  framework?: string;
  tags: string[];
  content: string;
  code: string;
  dependencies: Array<{ name: string; version: string; required?: boolean }>;
  estimated_tokens?: number;
  is_public: boolean;
};

function toBackendSnippetRequest(data: CreateSnippetRequest | UpdateSnippetRequest): Partial<BackendSnippetRequest> {
  const name = data.name ?? data.title;
  return {
    ...(name !== undefined ? { name } : {}),
    ...(data.language !== undefined ? { language: data.language } : {}),
    ...(data.framework !== undefined ? { framework: data.framework } : {}),
    ...(data.tags !== undefined ? { tags: data.tags } : {}),
    ...(data.content !== undefined || data.description !== undefined
      ? { content: data.content ?? data.description ?? '' }
      : {}),
    ...(data.code !== undefined ? { code: data.code } : {}),
    ...(data.dependencies !== undefined ? { dependencies: data.dependencies } : {}),
    ...(data.estimated_tokens !== undefined ? { estimated_tokens: data.estimated_tokens } : {}),
    ...(data.is_public !== undefined ? { is_public: data.is_public } : {}),
  };
}

export const snippetsApi = {
  /**
   * List snippets with pagination and filters
   */
  async list(params?: ListQueryParams): Promise<PaginatedResponse<Snippet>> {
    const qs = params ? buildQueryString(params) : '';
    const response = await apiClient.get<PaginatedResponse<Snippet>>(`/snippets${qs}`);
    return response.data;
  },

  /**
   * Get a single snippet by ID
   */
  async get(id: string): Promise<ApiResponse<Snippet>> {
    const response = await apiClient.get<ApiResponse<Snippet>>(`/snippets/${id}`);
    return response.data;
  },

  /**
   * Create a new snippet
   */
  async create(data: CreateSnippetRequest): Promise<ApiResponse<Snippet>> {
    const response = await apiClient.post<ApiResponse<Snippet>>('/snippets', {
      dependencies: [],
      tags: [],
      is_public: false,
      ...toBackendSnippetRequest(data),
    });
    return response.data;
  },

  /**
   * Update an existing snippet
   */
  async update(id: string, data: UpdateSnippetRequest): Promise<ApiResponse<Snippet>> {
    const response = await apiClient.put<ApiResponse<Snippet>>(
      `/snippets/${id}`,
      toBackendSnippetRequest(data)
    );
    return response.data;
  },

  /**
   * Delete a snippet
   */
  async delete(id: string): Promise<ApiResponse<null>> {
    const response = await apiClient.delete<ApiResponse<null>>(`/snippets/${id}`);
    return response.data;
  },

  /**
   * Search snippets by code content
   */
  async search(query: string, language?: string): Promise<ApiResponse<Snippet[]>> {
    const qs = buildQueryString({ q: query, language });
    const response = await apiClient.get<ApiResponse<Snippet[]>>(`/snippets/search${qs}`);
    return response.data;
  },

  /**
   * Get snippet categories
   */
  async categories(): Promise<ApiResponse<string[]>> {
    return {
      success: true,
      data: ['custom', 'utility', 'api', 'data', 'ai'],
    };
  },

  /**
   * Get supported languages
   */
  async languages(): Promise<ApiResponse<string[]>> {
    return {
      success: true,
      data: ['typescript', 'javascript', 'python', 'rust', 'go', 'java', 'csharp', 'sql', 'bash', 'json', 'yaml', 'markdown'],
    };
  },

  /**
   * Get snippet reference (for LLM consumption)
   */
  async getReference(id: string, format: 'direct' | 'inline' | 'with_deps' = 'direct'): Promise<ApiResponse<unknown>> {
    const response = await apiClient.get<ApiResponse<unknown>>(`/snippets/${id}/reference?format=${format}`);
    return response.data;
  },
};
