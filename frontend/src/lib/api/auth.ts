import { apiClient, setToken, clearToken } from './client';
import type {
  ApiResponse,
  AuthToken,
  LoginRequest,
  RegisterRequest,
  User,
} from './types';

// ============================================
// Auth Service
// ============================================

export const authApi = {
  /**
   * Register a new user
   */
  async register(data: RegisterRequest): Promise<ApiResponse<AuthToken>> {
    const response = await apiClient.post<ApiResponse<AuthToken>>('/auth/register', data);
    
    if (response.data.success && response.data.data) {
      setToken(response.data.data.token, response.data.data.expires_at);
    }
    
    return response.data;
  },

  /**
   * Login with email and password
   */
  async login(data: LoginRequest): Promise<ApiResponse<AuthToken>> {
    const response = await apiClient.post<ApiResponse<AuthToken>>('/auth/login', data);
    
    if (response.data.success && response.data.data) {
      setToken(response.data.data.token, response.data.data.expires_at);
    }
    
    return response.data;
  },

  /**
   * Logout current user
   */
  async logout(): Promise<void> {
    try {
      await apiClient.post('/auth/logout');
    } finally {
      clearToken();
    }
  },

  /**
   * Get current user profile
   */
  async me(): Promise<ApiResponse<User>> {
    const response = await apiClient.get<ApiResponse<User>>('/auth/me');
    return response.data;
  },

  /**
   * Update current user profile
   */
  async updateProfile(data: Partial<Pick<User, 'email' | 'username'>>): Promise<ApiResponse<User>> {
    const response = await apiClient.patch<ApiResponse<User>>('/auth/profile', data);
    return response.data;
  },

  /**
   * Change password
   */
  async changePassword(currentPassword: string, newPassword: string): Promise<ApiResponse<null>> {
    const response = await apiClient.post<ApiResponse<null>>('/auth/change-password', {
      old_password: currentPassword,
      new_password: newPassword,
    });
    return response.data;
  },

  /**
   * Accept an invitation and create the invited user account.
   */
  async acceptInvitation(data: {
    token: string;
    username: string;
    password: string;
  }): Promise<ApiResponse<AuthToken>> {
    const response = await apiClient.post<ApiResponse<AuthToken>>('/invitations/accept', data);

    if (response.data.success && response.data.data) {
      setToken(response.data.data.token, response.data.data.expires_at);
    }

    return response.data;
  },
};
