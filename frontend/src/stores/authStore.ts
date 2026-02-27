import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { authApi, getToken, clearToken, type User, type TenantInfo } from '@/lib/api';

interface AuthState {
  // State
  user: User | null;
  tenant: TenantInfo | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  error: string | null;
  
  // Actions
  login: (email: string, password: string) => Promise<void>;
  register: (email: string, username: string, password: string) => Promise<void>;
  logout: () => Promise<void>;
  fetchUser: () => Promise<void>;
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
            set({ 
              user: response.data.user, 
              tenant: (response.data as any).tenant || null,
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
          set({ 
            error: error instanceof Error ? error.message : 'Login failed',
            isLoading: false 
          });
          throw error;
        }
      },

      register: async (email: string, username: string, password: string) => {
        set({ isLoading: true, error: null });
        
        try {
          const response = await authApi.register({ email, username, password });
          
          if (response.success && response.data) {
            set({ 
              user: response.data.user, 
              tenant: (response.data as any).tenant || null,
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
