'use client';

import { usePathname } from 'next/navigation';
import { MainLayout } from './MainLayout';
import { AuthGuard } from '@/components/AuthGuard';

const noLayoutRoutes = ['/', '/login', '/register', '/forgot-password', '/reset-password', '/onboarding'];

export function LayoutWrapper({ children }: { children: React.ReactNode }) {
  const pathname = usePathname();
  
  // Don't show main layout on home, login, register pages
  if (noLayoutRoutes.includes(pathname)) {
    return <>{children}</>;
  }
  
  return (
    <AuthGuard>
      <MainLayout>{children}</MainLayout>
    </AuthGuard>
  );
}