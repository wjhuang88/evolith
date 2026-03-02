'use client';

import { usePathname } from 'next/navigation';
import { MainLayout } from './MainLayout';

const noLayoutRoutes = ['/', '/login', '/register', '/forgot-password', '/reset-password'];

export function LayoutWrapper({ children }: { children: React.ReactNode }) {
  const pathname = usePathname();
  
  // Don't show main layout on home, login, register pages
  if (noLayoutRoutes.includes(pathname)) {
    return <>{children}</>;
  }
  
  return <MainLayout>{children}</MainLayout>;
}