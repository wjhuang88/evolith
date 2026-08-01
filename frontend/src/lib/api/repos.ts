import { apiClient } from './client';
import type {
  ApiResponse,
  Repo,
  CreateRepoRequest,
  UpdateRepoRequest,
} from './types';

interface RepoListData {
  repos: Repo[];
  total: number;
}

function unwrap<T>(payload: ApiResponse<T>): T | undefined {
  if (!payload.success) {
    return undefined;
  }
  return payload.data;
}

export const reposApi = {
  async list(tenantId: string): Promise<Repo[]> {
    const response = await apiClient.get<ApiResponse<RepoListData>>(
      `/tenant/${tenantId}/repos`
    );
    return unwrap(response.data)?.repos ?? [];
  },

  async get(tenantId: string, repoId: string): Promise<Repo | null> {
    const response = await apiClient.get<ApiResponse<Repo>>(
      `/tenant/${tenantId}/repos/${repoId}`
    );
    return unwrap(response.data) ?? null;
  },

  async create(tenantId: string, req: CreateRepoRequest): Promise<Repo> {
    const response = await apiClient.post<ApiResponse<Repo>>(
      `/tenant/${tenantId}/repos`,
      req
    );
    const data = unwrap(response.data);
    if (!data) {
      throw new Error(response.data.error?.message ?? 'Failed to create repo');
    }
    return data;
  },

  async update(
    tenantId: string,
    repoId: string,
    req: UpdateRepoRequest
  ): Promise<Repo> {
    const response = await apiClient.patch<ApiResponse<Repo>>(
      `/tenant/${tenantId}/repos/${repoId}`,
      req
    );
    const data = unwrap(response.data);
    if (!data) {
      throw new Error(response.data.error?.message ?? 'Failed to update repo');
    }
    return data;
  },

  async delete(tenantId: string, repoId: string): Promise<void> {
    await apiClient.delete(`/tenant/${tenantId}/repos/${repoId}`);
  },
};