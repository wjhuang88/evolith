import { apiClient, buildQueryString } from './client';
import type {
  ApiResponse,
  Tool,
  CreateToolRequest,
  UpdateToolRequest,
  ListQueryParams,
  ListResponse,
} from './types';

// ============================================
// Tools Service
// ============================================

export const toolsApi = {
  /**
   * List tools with pagination and filters
   */
  async list(params?: ListQueryParams): Promise<ApiResponse<ListResponse<Tool>>> {
    const qs = params ? buildQueryString(params) : '';
    const response = await apiClient.get<ApiResponse<ListResponse<Tool>>>(`/tools${qs}`);
    return response.data;
  },

  /**
   * Get a single tool by ID
   */
  async get(id: string): Promise<ApiResponse<Tool>> {
    const response = await apiClient.get<ApiResponse<Tool>>(`/tools/${id}`);
    return response.data;
  },

  /**
   * Create a new tool
   */
  async create(data: CreateToolRequest): Promise<ApiResponse<Tool>> {
    const response = await apiClient.post<ApiResponse<Tool>>('/tools', data);
    return response.data;
  },

  /**
   * Update an existing tool
   */
  async update(id: string, data: UpdateToolRequest): Promise<ApiResponse<Tool>> {
    const response = await apiClient.patch<ApiResponse<Tool>>(`/tools/${id}`, data);
    return response.data;
  },

  /**
   * Delete a tool
   */
  async delete(id: string): Promise<ApiResponse<null>> {
    const response = await apiClient.delete<ApiResponse<null>>(`/tools/${id}`);
    return response.data;
  },

  /**
   * Execute a tool (call the tool)
   */
  async execute(id: string, params: Record<string, unknown>): Promise<ApiResponse<unknown>> {
    const response = await apiClient.post<ApiResponse<unknown>>(`/tools/${id}/execute`, params);
    return response.data;
  },

  /**
   * Get tool categories
   */
  async categories(): Promise<ApiResponse<string[]>> {
    const response = await apiClient.get<ApiResponse<string[]>>('/tools/categories');
    return response.data;
  },
};
