import { apiClient, buildQueryString } from './client';
import type {
  ApiResponse,
  PaginatedResponse,
  Tool,
  CreateToolRequest,
  UpdateToolRequest,
  ListQueryParams,
} from './types';

// ============================================
// Tools Service
// ============================================

interface ToolListData {
  tools: Tool[];
  page: number;
  per_page: number;
  total: number;
}

function normalizeTool(tool: Tool): Tool {
  return {
    ...tool,
    input_schema: tool.input_schema ?? tool.schema ?? {},
  };
}

export const toolsApi = {
  /**
   * List tools with pagination and filters
   */
  async list(params?: ListQueryParams): Promise<PaginatedResponse<Tool>> {
    const qs = params ? buildQueryString(params) : '';
    const response = await apiClient.get<ApiResponse<ToolListData>>(`/tools${qs}`);
    const payload = response.data;
    const data = payload.data?.tools.map(normalizeTool) ?? [];
    return {
      success: payload.success,
      data,
      meta: payload.data
        ? {
            page: payload.data.page,
            per_page: payload.data.per_page,
            total: payload.data.total,
          }
        : {
            page: 1,
            per_page: data.length,
            total: data.length,
          },
      error: payload.error,
    };
  },

  /**
   * Get a single tool by ID
   */
  async get(id: string): Promise<ApiResponse<Tool>> {
    const response = await apiClient.get<ApiResponse<Tool>>(`/tools/${id}`);
    return {
      ...response.data,
      data: response.data.data ? normalizeTool(response.data.data) : undefined,
    };
  },

  /**
   * Create a new tool
   */
  async create(data: CreateToolRequest): Promise<ApiResponse<Tool>> {
    const response = await apiClient.post<ApiResponse<Tool>>('/tools', data);
    return {
      ...response.data,
      data: response.data.data ? normalizeTool(response.data.data) : undefined,
    };
  },

  /**
   * Update an existing tool
   */
  async update(id: string, data: UpdateToolRequest): Promise<ApiResponse<Tool>> {
    const response = await apiClient.put<ApiResponse<Tool>>(`/tools/${id}`, data);
    return {
      ...response.data,
      data: response.data.data ? normalizeTool(response.data.data) : undefined,
    };
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
    void id;
    void params;
    return {
      success: false,
      error: {
        code: 'NOT_IMPLEMENTED',
        message: 'Tool execution is not implemented yet. Track EVO-005 for this capability.',
      },
    };
  },

  /**
   * Get tool categories
   */
  async categories(): Promise<ApiResponse<string[]>> {
    return {
      success: true,
      data: ['custom', 'utility', 'api', 'data', 'ai'],
    };
  },
};
