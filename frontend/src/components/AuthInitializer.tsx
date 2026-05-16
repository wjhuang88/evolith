'use client';

import { useEffect, useRef } from 'react';
import { useRouter } from '@/lib/router';
import { useAuthStore } from '@/stores/authStore';
import { getToken } from '@/lib/api';

interface AuthInitializerProps {
  children: React.ReactNode;
}

export function AuthInitializer({ children }: AuthInitializerProps) {
  const router = useRouter();
  const { fetchUser, logout } = useAuthStore();
  const initialized = useRef(false);

  useEffect(() => {
    // Prevent double initialization
    if (initialized.current) return;
    initialized.current = true;

    // Restore auth state if token exists
    const token = getToken();
    if (token) {
      fetchUser();
    }
  }, [fetchUser]);

  useEffect(() => {
    // Listen for unauthorized events
    const handleUnauthorized = () => {
      logout();
      router.push('/login');
    };

    window.addEventListener('auth:unauthorized', handleUnauthorized);

    return () => {
      window.removeEventListener('auth:unauthorized', handleUnauthorized);
    };
  }, [logout, router]);

  return <>{children}</>;
}
