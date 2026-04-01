'use client';

import { useEffect, useState } from 'react';
import { useRouter, usePathname } from 'next/navigation';
import { useAuthStore } from '@/stores/authStore';
import { getToken } from '@/lib/api';

interface AuthGuardProps {
  children: React.ReactNode;
}

export function AuthGuard({ children }: AuthGuardProps) {
  const router = useRouter();
  const pathname = usePathname();
  const { isAuthenticated, isLoading, fetchUser } = useAuthStore();
  const [isChecking, setIsChecking] = useState(true);

  useEffect(() => {
    async function checkAuth() {
      const token = getToken();

      if (!token) {
        // No token, redirect to login
        const redirectParam = new URLSearchParams({ redirect: pathname });
        router.push(`/login?${redirectParam.toString()}`);
        return;
      }

      // Token exists, check if we have user data
      const state = useAuthStore.getState();
      if (!state.user && !isLoading) {
        // Fetch user data to restore state
        await fetchUser();
      }

      setIsChecking(false);
    }

    checkAuth();
  }, [pathname, fetchUser, isLoading, router]);

  // Check if we should redirect (no token)
  const token = getToken();
  if (!token && !isChecking) {
    return null; // Router will handle redirect
  }

  // Show loading state while checking auth or fetching user
  if (isChecking || (isLoading && !useAuthStore.getState().user)) {
    return (
      <div className="flex min-h-screen items-center justify-center bg-background">
        <div className="flex flex-col items-center space-y-4">
          <div className="h-12 w-12 animate-spin rounded-full border-4 border-primary border-t-transparent" />
          <p className="text-muted-foreground">Loading...</p>
        </div>
      </div>
    );
  }

  // Not authenticated, will redirect
  if (!isAuthenticated) {
    return null;
  }

  // Authenticated, render children
  return <>{children}</>;
}
