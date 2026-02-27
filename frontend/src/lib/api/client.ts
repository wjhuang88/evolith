import axios, { AxiosError, InternalAxiosRequestConfig } from 'axios';
import type { ApiResponse, ErrorInfo } from './types';

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080/api/v1';

// ============================================
// Token Management
// ============================================

const TOKEN_KEY = 'evolith_token';
const TOKEN_EXPIRY_KEY = 'evolith_token_expiry';

export function getToken(): string | null {
  if (typeof window === 'undefined') return null;
  return localStorage.getItem(TOKEN_KEY);
}

export function setToken(token: string, expiresAt: number): void {
  if (typeof window === 'undefined') return;
  localStorage.setItem(TOKEN_KEY, token);
  localStorage.setItem(TOKEN_EXPIRY_KEY, String(expiresAt));
}

export function clearToken(): void {
  if (typeof window === 'undefined') return;
  localStorage.removeItem(TOKEN_KEY);
  localStorage.removeItem(TOKEN_EXPIRY_KEY);
}

export function isTokenExpired(): boolean {
  if (typeof window === 'undefined') return true;
  const expiry = localStorage.getItem(TOKEN_EXPIRY_KEY);
  if (!expiry) return true;
  // Add 30 second buffer
  return Date.now() / 1000 > parseInt(expiry) - 30;
}

// ============================================
// Axios Client
// ============================================

export const apiClient = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
  timeout: 30000,
});

// Request interceptor
apiClient.interceptors.request.use(
  (config: InternalAxiosRequestConfig) => {
    const token = getToken();
    if (token && !isTokenExpired()) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    // Add request ID for tracing
    config.headers['X-Request-ID'] = crypto.randomUUID();
    return config;
  },
  (error) => Promise.reject(error)
);

// Response interceptor
apiClient.interceptors.response.use(
  (response) => response,
  async (error: AxiosError<ApiResponse<unknown>>) => {
    const originalRequest = error.config as InternalAxiosRequestConfig & { _retry?: boolean };

    // Handle 401 - try to refresh token
    if (error.response?.status === 401 && !originalRequest._retry) {
      originalRequest._retry = true;
      
      // Clear invalid token
      clearToken();
      
      // Redirect to login or dispatch auth event
      if (typeof window !== 'undefined') {
        window.dispatchEvent(new CustomEvent('auth:unauthorized'));
      }
    }

    return Promise.reject(error);
  }
);

// ============================================
// Error Handling
// ============================================

export function parseApiError(error: unknown): ErrorInfo {
  if (axios.isAxiosError(error)) {
    const axiosError = error as AxiosError<ApiResponse<unknown>>;
    
    if (axiosError.response?.data?.error) {
      return axiosError.response.data.error;
    }
    
    if (axiosError.code === 'ECONNABORTED') {
      return {
        code: 'TIMEOUT',
        message: 'Request timed out. Please try again.',
        retryable: true,
      };
    }
    
    if (!axiosError.response) {
      return {
        code: 'NETWORK_ERROR',
        message: 'Network error. Please check your connection.',
        retryable: true,
      };
    }
    
    return {
      code: `HTTP_${axiosError.response.status}`,
      message: axiosError.message || 'An error occurred',
      retryable: axiosError.response.status >= 500,
    };
  }
  
  return {
    code: 'UNKNOWN',
    message: 'An unexpected error occurred',
    retryable: false,
  };
}

// ============================================
// Helper Functions
// ============================================

export function buildQueryString(params: Record<string, unknown>): string {
  const searchParams = new URLSearchParams();
  
  Object.entries(params).forEach(([key, value]) => {
    if (value !== undefined && value !== null) {
      if (Array.isArray(value)) {
        value.forEach((v) => searchParams.append(key, String(v)));
      } else {
        searchParams.set(key, String(value));
      }
    }
  });
  
  const qs = searchParams.toString();
  return qs ? `?${qs}` : '';
}
