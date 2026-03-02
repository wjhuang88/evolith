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
    const response = await apiClient.post<ApiResponse<Snippet>>('/snippets', data);
    return response.data;
  },

  /**
   * Update an existing snippet
   */
  async update(id: string, data: UpdateSnippetRequest): Promise<ApiResponse<Snippet>> {
    const response = await apiClient.patch<ApiResponse<Snippet>>(`/snippets/${id}`, data);
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
    const response = await apiClient.get<ApiResponse<string[]>>('/snippets/categories');
    return response.data;
  },

  /**
   * Get supported languages
   */
  async languages(): Promise<ApiResponse<string[]>> {
    const response = await apiClient.get<ApiResponse<string[]>>('/snippets/languages');
    return response.data;
  },

  /**
   * Get snippet reference (for LLM consumption)
   */
  async getReference(id: string, format: 'direct' | 'inline' | 'with_deps' = 'direct'): Promise<ApiResponse<unknown>> {
    const response = await apiClient.get<ApiResponse<unknown>>(`/snippets/${id}/reference?format=${format}`);
    return response.data;
  },
};
