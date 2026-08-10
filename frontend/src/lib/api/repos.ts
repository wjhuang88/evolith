import { apiClient } from './client';
import type {
  ApiResponse,
  Repo,
  CreateRepoRequest,
  UpdateRepoRequest,
  FileTreeEntry,
  CommitInfo,
  CommitDetail,
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
    const data = unwrap(response.data);
    if (!data) {
      throw new Error(response.data.error?.message ?? 'Failed to list repos');
    }
    return data.repos;
  },

  async get(tenantId: string, repoId: string): Promise<Repo | null> {
    try {
      const response = await apiClient.get<ApiResponse<Repo>>(
        `/tenant/${tenantId}/repos/${repoId}`
      );
      return unwrap(response.data) ?? null;
    } catch (error) {
      const status = (error as { response?: { status?: number } }).response?.status;
      if (status === 404) return null;
      throw error;
    }
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

  async fileTree(tenantId: string, repoId: string, ref?: string): Promise<FileTreeEntry[]> {
    const response = await apiClient.get<ApiResponse<{ entries: FileTreeEntry[] }>>(
      `/tenant/${tenantId}/repos/${repoId}/file-tree`,
      { params: ref ? { ref } : undefined }
    );
    return unwrap(response.data)?.entries ?? [];
  },

  async blob(tenantId: string, repoId: string, sha: string): Promise<{ content: string; size: number; encoding: 'utf-8' | 'base64' }> {
    const response = await apiClient.get<ApiResponse<{ content: string; size: number; encoding: 'utf-8' | 'base64' }>>(
      `/tenant/${tenantId}/repos/${repoId}/blobs/${sha}`
    );
    const data = unwrap(response.data);
    if (!data) throw new Error(response.data.error?.message ?? 'Failed to load file');
    return data;
  },

  async commits(tenantId: string, repoId: string, ref?: string, limit = 50): Promise<CommitInfo[]> {
    const response = await apiClient.get<ApiResponse<{ commits: CommitInfo[] }>>(
      `/tenant/${tenantId}/repos/${repoId}/commits`,
      { params: { ...(ref ? { ref } : {}), limit } }
    );
    return unwrap(response.data)?.commits ?? [];
  },

  async commitDetail(tenantId: string, repoId: string, sha: string, ref?: string): Promise<CommitDetail> {
    const response = await apiClient.get<ApiResponse<CommitDetail>>(
      `/tenant/${tenantId}/repos/${repoId}/commits/${sha}`,
      { params: ref ? { ref } : undefined }
    );
    const data = unwrap(response.data);
    if (!data) throw new Error(response.data.error?.message ?? 'Failed to load commit detail');
    return data;
  },
};
