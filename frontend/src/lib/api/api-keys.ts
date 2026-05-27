import { apiClient, buildQueryString } from './client';
import type {
  ApiResponse,
  ApiKey,
  ApiKeyWithSecret,
  CreateApiKeyRequest,
  ListQueryParams,
} from './types';

export const apiKeysApi = {
  async list(tenantId: string, params?: ListQueryParams): Promise<ApiResponse<{ keys: ApiKey[]; total: number }>> {
    const query = params ? buildQueryString(params) : '';
    const response = await apiClient.get<ApiResponse<{ keys: ApiKey[]; total: number }>>(
      `/tenant/${tenantId}/api-keys${query}`
    );
    return response.data;
  },

  async create(tenantId: string, data: CreateApiKeyRequest): Promise<ApiResponse<ApiKeyWithSecret>> {
    const response = await apiClient.post<ApiResponse<ApiKeyWithSecret>>(
      `/tenant/${tenantId}/api-keys`,
      data
    );
    return response.data;
  },

  async revoke(tenantId: string, keyId: string): Promise<ApiResponse<{ message: string }>> {
    const response = await apiClient.delete<ApiResponse<{ message: string }>>(
      `/tenant/${tenantId}/api-keys/${keyId}`
    );
    return response.data;
  },
};
