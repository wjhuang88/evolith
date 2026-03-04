'use client';

import { ReactNode } from 'react';
import { usePermission, TenantRole } from '@/hooks/usePermission';

interface PermissionGuardProps {
  children: ReactNode;
  requiredRole?: TenantRole;
  fallback?: ReactNode;
}

/**
 * Component that conditionally renders children based on user's role
 * 
 * Usage:
 * <PermissionGuard requiredRole="admin">
 *   <AdminPanel />
 * </PermissionGuard>
 * 
 * <PermissionGuard requiredRole="admin" fallback={<UpgradePrompt />}>
 *   <PremiumFeature />
 * </PermissionGuard>
 */
export function PermissionGuard({ 
  children, 
  requiredRole = 'member',
  fallback = null 
}: PermissionGuardProps) {
  const { can } = usePermission();
  
  if (!can(requiredRole)) {
    return fallback;
  }
  
  return <>{children}</>;
}

interface OwnerOnlyProps {
  children: ReactNode;
  fallback?: ReactNode;
}

/**
 * Component that only renders children for owners
 */
export function OwnerOnly({ children, fallback = null }: OwnerOnlyProps) {
  return (
    <PermissionGuard requiredRole="owner" fallback={fallback}>
      {children}
    </PermissionGuard>
  );
}

interface AdminOnlyProps {
  children: ReactNode;
  fallback?: ReactNode;
}

/**
 * Component that renders children for admins and owners
 */
export function AdminOnly({ children, fallback = null }: AdminOnlyProps) {
  return (
    <PermissionGuard requiredRole="admin" fallback={fallback}>
      {children}
    </PermissionGuard>
  );
}

interface PermissionButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  requiredRole?: TenantRole;
  children: ReactNode;
}

/**
 * Button that is disabled if user doesn't have required permissions
 */
export function PermissionButton({
  requiredRole = 'member',
  children,
  disabled,
  ...props
}: PermissionButtonProps) {
  const { can } = usePermission();
  const hasPermission = can(requiredRole);
  
  return (
    <button
      {...props}
      disabled={disabled || !hasPermission}
      className={`
        ${props.className || ''}
        ${!hasPermission ? 'opacity-50 cursor-not-allowed' : ''}
      `}
    >
      {children}
    </button>
  );
}
