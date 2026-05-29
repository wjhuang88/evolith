'use client';

import { useAuthStore } from '@/stores';

export type TenantRole = 'owner' | 'admin' | 'member';

/**
 * Hook for checking user permissions
 * 
 * Usage:
 * const { can, isOwner, isAdmin, isMember, tenantRole } = usePermission();
 * 
 * if (can('admin')) {
 *   // Show admin-only features
 * }
 */
export function usePermission() {
  const { user, isAuthenticated } = useAuthStore();
  
  const tenantRole = (user?.tenant_role || 'member') as TenantRole;
  
  /**
   * Check if user has required role or higher
   * Role hierarchy: owner > admin > member
   */
  const can = (requiredRole: TenantRole): boolean => {
    if (!isAuthenticated || !user) return false;
    
    const roleLevels: Record<TenantRole, number> = {
      owner: 3,
      admin: 2,
      member: 1,
    };
    
    return roleLevels[tenantRole] >= roleLevels[requiredRole];
  };
  
  const isOwner = tenantRole === 'owner';
  const isAdmin = tenantRole === 'owner' || tenantRole === 'admin';
  const isMember = tenantRole === 'member';
  
  return {
    can,
    isOwner,
    isAdmin,
    isMember,
    tenantRole,
    isAuthenticated,
  };
}

/**
 * Hook for checking specific permissions
 * 
 * Usage:
 * const { canInvite, canRemoveMember, canTransferOwnership } = useMemberPermissions();
 */
export function useMemberPermissions() {
  const { can, isOwner, isAdmin } = usePermission();
  
  return {
    // Member management permissions
    canInvite: isAdmin,                    // Admin and owner can invite
    canRemoveMember: isAdmin,              // Admin and owner can remove members
    canUpdateRole: isAdmin,                // Admin and owner can update roles
    canCancelInvitation: isAdmin,          // Admin and owner can cancel invitations
    canTransferOwnership: isOwner,         // Only owner can transfer ownership
    canViewMembers: true,                  // All members can view member list
    canViewInvitations: true,              // All members can view invitations
  };
}

/**
 * Hook for checking tenant settings permissions
 */
export function useTenantPermissions() {
  const { isOwner, isAdmin } = usePermission();
  
  return {
    canViewSettings: true,
    canUpdateSettings: isAdmin,
    canManageBilling: isOwner,
    canManageApiKeys: isAdmin,
    canViewUsage: true,
    canUpgradePlan: isOwner,
  };
}

/**
 * Hook for checking resource management permissions
 */
export function useResourcePermissions() {
  const { can, isOwner, isAdmin } = usePermission();
  
  return {
    // Tools
    canCreateTool: true,
    canUpdateTool: (toolOwnerId?: string, currentUserId?: string) => {
      if (isAdmin) return true;
      return toolOwnerId === currentUserId;
    },
    canDeleteTool: (toolOwnerId?: string, currentUserId?: string) => {
      if (isAdmin) return true;
      return toolOwnerId === currentUserId;
    },
    canPublishTool: isAdmin,
    
    // Skills
    canCreateSkill: true,
    canUpdateSkill: (skillOwnerId?: string, currentUserId?: string) => {
      if (isAdmin) return true;
      return skillOwnerId === currentUserId;
    },
    canDeleteSkill: (skillOwnerId?: string, currentUserId?: string) => {
      if (isAdmin) return true;
      return skillOwnerId === currentUserId;
    },
    canPublishSkill: isAdmin,
    
    // CLI Interfaces
    canCreateInterface: true,
    canUpdateInterface: (interfaceOwnerId?: string, currentUserId?: string) => {
      if (isAdmin) return true;
      return interfaceOwnerId === currentUserId;
    },
    canDeleteInterface: (interfaceOwnerId?: string, currentUserId?: string) => {
      if (isAdmin) return true;
      return interfaceOwnerId === currentUserId;
    },
    canPublishInterface: isAdmin,
  };
}
