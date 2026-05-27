import { apiClient, buildQueryString } from './client';
import type {
  ApiResponse,
  Member,
  Invitation,
  InviteMemberRequest,
  ListQueryParams,
} from './types';

export const membersApi = {
  async list(tenantId: string, params?: ListQueryParams): Promise<ApiResponse<{ members: Member[]; total: number }>> {
    const query = params ? buildQueryString(params) : '';
    const response = await apiClient.get<ApiResponse<{ members: Member[]; total: number }>>(
      `/tenant/${tenantId}/members${query}`
    );
    return response.data;
  },

  async listInvitations(tenantId: string): Promise<ApiResponse<{ invitations: Invitation[]; total: number }>> {
    const response = await apiClient.get<ApiResponse<{ invitations: Invitation[]; total: number }>>(
      `/tenant/${tenantId}/members/invitations`
    );
    return response.data;
  },

  async invite(tenantId: string, data: InviteMemberRequest): Promise<ApiResponse<{ id: string; email: string; role: string; invite_url: string; expires_at: string }>> {
    const response = await apiClient.post<ApiResponse<{ id: string; email: string; role: string; invite_url: string; expires_at: string }>>(
      `/tenant/${tenantId}/members/invite`,
      data
    );
    return response.data;
  },

  async remove(tenantId: string, memberId: string): Promise<ApiResponse<{ message: string }>> {
    const response = await apiClient.delete<ApiResponse<{ message: string }>>(
      `/tenant/${tenantId}/members/${memberId}`
    );
    return response.data;
  },

  async acceptInvitation(data: { token: string; password: string; username: string }): Promise<ApiResponse<unknown>> {
    const response = await apiClient.post<ApiResponse<unknown>>(
      '/auth/accept-invite',
      data
    );
    return response.data;
  },
};
