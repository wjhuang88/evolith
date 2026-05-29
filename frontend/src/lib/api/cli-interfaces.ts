import { apiClient, buildQueryString } from './client';
import type {
  ApiResponse,
  PaginatedResponse,
  CliInterface,
  CreateCliInterfaceRequest,
  UpdateCliInterfaceRequest,
  ListQueryParams,
} from './types';

// ============================================
// CLI Interfaces Service
// ============================================

type BackendCliInterfaceRequest = {
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

function toBackendCliInterfaceRequest(data: CreateCliInterfaceRequest | UpdateCliInterfaceRequest): Partial<BackendCliInterfaceRequest> {
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

export const cliInterfacesApi = {
  /**
   * List CLI interfaces with pagination and filters
   */
  async list(params?: ListQueryParams): Promise<PaginatedResponse<CliInterface>> {
    const qs = params ? buildQueryString(params) : '';
    const response = await apiClient.get<PaginatedResponse<CliInterface>>(`/snippets${qs}`);
    return response.data;
  },

  /**
   * Get a single CLI interface by ID
   */
  async get(id: string): Promise<ApiResponse<CliInterface>> {
    const response = await apiClient.get<ApiResponse<CliInterface>>(`/snippets/${id}`);
    return response.data;
  },

  /**
   * Create a new CLI interface
   */
  async create(data: CreateCliInterfaceRequest): Promise<ApiResponse<CliInterface>> {
    const response = await apiClient.post<ApiResponse<CliInterface>>('/snippets', {
      dependencies: [],
      tags: [],
      is_public: false,
      ...toBackendCliInterfaceRequest(data),
    });
    return response.data;
  },

  /**
   * Update an existing CLI interface
   */
  async update(id: string, data: UpdateCliInterfaceRequest): Promise<ApiResponse<CliInterface>> {
    const response = await apiClient.put<ApiResponse<CliInterface>>(
      `/snippets/${id}`,
      toBackendCliInterfaceRequest(data)
    );
    return response.data;
  },

  /**
   * Delete a CLI interface
   */
  async delete(id: string): Promise<ApiResponse<null>> {
    const response = await apiClient.delete<ApiResponse<null>>(`/snippets/${id}`);
    return response.data;
  },

  /**
   * Search CLI interfaces by code content
   */
  async search(query: string, language?: string): Promise<ApiResponse<CliInterface[]>> {
    const qs = buildQueryString({ q: query, language });
    const response = await apiClient.get<ApiResponse<CliInterface[]>>(`/snippets/search${qs}`);
    return response.data;
  },

  /**
   * Get CLI interface categories
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
   * Get CLI interface reference (for LLM consumption)
   */
  async getReference(id: string, format: 'direct' | 'inline' | 'with_deps' = 'direct'): Promise<ApiResponse<unknown>> {
    const response = await apiClient.get<ApiResponse<unknown>>(`/snippets/${id}/reference?format=${format}`);
    return response.data;
  },
};
