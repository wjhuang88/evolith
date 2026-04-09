import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { authApi, getToken, clearToken, parseApiError, type User, type TenantInfo, type AuthToken } from '@/lib/api';

interface AuthState {
  // State
  user: User | null;
  tenant: TenantInfo | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  error: string | null;
  
  // Actions
  login: (email: string, password: string) => Promise<void>;
  register: (email: string, username: string, password: string, tenantName?: string, tenantSlug?: string) => Promise<void>;
  logout: () => Promise<void>;
  fetchUser: () => Promise<void>;
  initialize: () => Promise<void>;
  clearError: () => void;
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set, get) => ({
      user: null,
      tenant: null,
      isAuthenticated: !!getToken(),
      isLoading: false,
      error: null,

      login: async (email: string, password: string) => {
        set({ isLoading: true, error: null });
        
        try {
          const response = await authApi.login({ email, password });
          
          if (response.success && response.data) {
            const authToken = response.data as AuthToken;
            set({ 
              user: authToken.user, 
              tenant: (authToken as unknown as { tenant?: TenantInfo }).tenant || null,
              isAuthenticated: true,
              isLoading: false 
            });
          } else {
            set({ 
              error: response.error?.message || 'Login failed',
              isLoading: false 
            });
          }
        } catch (error) {
          const errorInfo = parseApiError(error);
          set({ 
            error: errorInfo.message,
            isLoading: false 
          });
          throw error;
        }
      },

      register: async (email: string, username: string, password: string, tenantName?: string, tenantSlug?: string) => {
        set({ isLoading: true, error: null });
        
        try {
          const response = await authApi.register({ 
            email, 
            username, 
            password,
            tenant_name: tenantName,
            tenant_slug: tenantSlug
          });
          
          if (response.success && response.data) {
            const authToken = response.data as AuthToken;
            set({ 
              user: authToken.user, 
              tenant: (authToken as unknown as { tenant?: TenantInfo }).tenant || null,
              isAuthenticated: true,
              isLoading: false 
            });
          } else {
            set({ 
              error: response.error?.message || 'Registration failed',
              isLoading: false 
            });
          }
        } catch (error) {
          set({ 
            error: error instanceof Error ? error.message : 'Registration failed',
            isLoading: false 
          });
          throw error;
        }
      },

      logout: async () => {
        set({ isLoading: true });
        
        try {
          await authApi.logout();
        } catch {
          // Ignore logout errors - clear local state anyway
        } finally {
          clearToken();
          set({ 
            user: null, 
            tenant: null,
            isAuthenticated: false, 
            isLoading: false,
            error: null 
          });
        }
      },

      fetchUser: async () => {
        if (!getToken()) {
          set({ isAuthenticated: false, user: null, tenant: null });
          return;
        }
        
        set({ isLoading: true });
        
        try {
          const response = await authApi.me();
          
          if (response.success && response.data) {
            set({ 
              user: response.data, 
              isAuthenticated: true,
              isLoading: false 
            });
          } else {
            // Token invalid or expired
            clearToken();
            set({ 
              user: null, 
              tenant: null,
              isAuthenticated: false,
              isLoading: false 
            });
          }
        } catch {
          clearToken();
          set({ 
            user: null, 
            tenant: null,
            isAuthenticated: false,
            isLoading: false 
          });
        }
      },

      initialize: async () => {
        const token = getToken();
        if (!token) {
          set({ isAuthenticated: false, user: null, tenant: null });
          return;
        }

        await get().fetchUser();
      },

      clearError: () => set({ error: null }),
    }),
    {
      name: 'auth-storage',
      partialize: (state) => ({ 
        user: state.user,
        tenant: state.tenant,
        isAuthenticated: state.isAuthenticated,
      }),
    }
  )
);