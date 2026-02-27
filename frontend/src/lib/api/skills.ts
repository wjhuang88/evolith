import { apiClient, buildQueryString } from './client';
import type {
  ApiResponse,
  PaginatedResponse,
  Skill,
  CreateSkillRequest,
  UpdateSkillRequest,
  ListQueryParams,
} from './types';

// ============================================
// Skills Service
// ============================================

export const skillsApi = {
  /**
   * List skills with pagination and filters
   */
  async list(params?: ListQueryParams): Promise<PaginatedResponse<Skill>> {
    const qs = params ? buildQueryString(params) : '';
    const response = await apiClient.get<PaginatedResponse<Skill>>(`/skills${qs}`);
    return response.data;
  },

  /**
   * Get a single skill by ID
   */
  async get(id: string): Promise<ApiResponse<Skill>> {
    const response = await apiClient.get<ApiResponse<Skill>>(`/skills/${id}`);
    return response.data;
  },

  /**
   * Create a new skill
   */
  async create(data: CreateSkillRequest): Promise<ApiResponse<Skill>> {
    const response = await apiClient.post<ApiResponse<Skill>>('/skills', data);
    return response.data;
  },

  /**
   * Update an existing skill
   */
  async update(id: string, data: UpdateSkillRequest): Promise<ApiResponse<Skill>> {
    const response = await apiClient.patch<ApiResponse<Skill>>(`/skills/${id}`, data);
    return response.data;
  },

  /**
   * Delete a skill
   */
  async delete(id: string): Promise<ApiResponse<null>> {
    const response = await apiClient.delete<ApiResponse<null>>(`/skills/${id}`);
    return response.data;
  },

  /**
   * Execute a skill (run skill code)
   */
  async execute(id: string, params: Record<string, unknown>): Promise<ApiResponse<unknown>> {
    const response = await apiClient.post<ApiResponse<unknown>>(`/skills/${id}/execute`, params);
    return response.data;
  },

  /**
   * Get skill categories
   */
  async categories(): Promise<ApiResponse<string[]>> {
    const response = await apiClient.get<ApiResponse<string[]>>('/skills/categories');
    return response.data;
  },

  /**
   * Get skill versions
   */
  async versions(id: string): Promise<ApiResponse<Skill[]>> {
    const response = await apiClient.get<ApiResponse<Skill[]>>(`/skills/${id}/versions`);
    return response.data;
  },
};
