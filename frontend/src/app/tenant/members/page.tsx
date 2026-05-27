'use client';

import { useState, useEffect, useCallback } from 'react';
import { useTranslation } from 'react-i18next';
import { useAuthStore } from '@/stores';
import { membersApi } from '@/lib/api/members';
import { useMemberPermissions } from '@/hooks/usePermission';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';
import type { Member, Invitation } from '@/lib/api/types';

type Tab = 'members' | 'invitations';

export default function MembersPage() {
  const { t } = useTranslation();
  const { user, tenant } = useAuthStore();
  const { canInvite, canRemoveMember } = useMemberPermissions();

  const tenantId = user?.tenant_id ?? '';

  const [members, setMembers] = useState<Member[]>([]);
  const [invitations, setInvitations] = useState<Invitation[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<Tab>('members');

  const [showInvite, setShowInvite] = useState(false);
  const [inviteEmail, setInviteEmail] = useState('');
  const [inviteRole, setInviteRole] = useState<string>('member');
  const [inviteMessage, setInviteMessage] = useState('');
  const [isInviting, setIsInviting] = useState(false);
  const [inviteError, setInviteError] = useState<string | null>(null);

  const [removingId, setRemovingId] = useState<string | null>(null);
  const [confirmRemoveId, setConfirmRemoveId] = useState<string | null>(null);

  const fetchData = useCallback(async () => {
    if (!tenantId) return;
    setIsLoading(true);
    setError(null);
    try {
      const [membersRes, invitationsRes] = await Promise.all([
        membersApi.list(tenantId),
        membersApi.listInvitations(tenantId),
      ]);
      if (membersRes.success && membersRes.data) {
        setMembers(membersRes.data.members);
      }
      if (invitationsRes.success && invitationsRes.data) {
        setInvitations(invitationsRes.data.invitations);
      }
      if (!membersRes.success && !invitationsRes.success) {
        setError(membersRes.error?.message ?? invitationsRes.error?.message ?? t('common.error'));
      }
    } catch {
      setError(t('common.networkError'));
    } finally {
      setIsLoading(false);
    }
  }, [tenantId, t]);

  useEffect(() => {
    fetchData();
  }, [fetchData]);

  const handleInvite = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!tenantId || !inviteEmail.trim()) return;

    setIsInviting(true);
    setInviteError(null);
    try {
      const response = await membersApi.invite(tenantId, {
        email: inviteEmail.trim(),
        role: inviteRole,
        message: inviteMessage.trim() || undefined,
      });
      if (response.success) {
        setInviteEmail('');
        setInviteRole('member');
        setInviteMessage('');
        setShowInvite(false);
        await fetchData();
      } else {
        setInviteError(response.error?.message ?? t('common.error'));
      }
    } catch {
      setInviteError(t('common.networkError'));
    } finally {
      setIsInviting(false);
    }
  };

  const handleRemove = async (memberId: string) => {
    if (!tenantId) return;

    setRemovingId(memberId);
    try {
      const response = await membersApi.remove(tenantId, memberId);
      if (response.success) {
        setConfirmRemoveId(null);
        await fetchData();
      } else {
        setError(response.error?.message ?? t('common.error'));
      }
    } catch {
      setError(t('common.networkError'));
    } finally {
      setRemovingId(null);
    }
  };

  const getRoleBadgeColor = (role: string) => {
    switch (role) {
      case 'owner':
        return 'bg-purple-100 text-purple-700 dark:bg-purple-900 dark:text-purple-300';
      case 'admin':
        return 'bg-blue-100 text-blue-700 dark:bg-blue-900 dark:text-blue-300';
      default:
        return 'bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-300';
    }
  };

  const getRoleLabel = (role: string) => {
    switch (role) {
      case 'owner': return t('tenant.members.roles.owner');
      case 'admin': return t('tenant.members.roles.admin');
      default: return t('tenant.members.roles.member');
    }
  };

  const getStatusBadge = (status: string) => {
    if (status === 'active') {
      return <span className="text-sm text-green-600">{t('tenant.members.table.active')}</span>;
    }
    if (status === 'pending') {
      return <span className="text-sm text-amber-600">{t('tenant.members.table.pending')}</span>;
    }
    if (status === 'expired') {
      return <span className="text-sm text-red-600">{t('tenant.members.table.expired')}</span>;
    }
    return <span className="text-sm text-muted-foreground">{status}</span>;
  };

  const pendingInvitations = invitations.filter((inv) => inv.status === 'pending');
  const expiredInvitations = invitations.filter((inv) => inv.status !== 'pending');

  return (
    <div className="container mx-auto py-8">
      <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 mb-8">
        <div>
          <h1 className="text-3xl font-bold">{t('tenant.members.title')}</h1>
          <p className="text-muted-foreground mt-1">
            {t('tenant.members.subtitle')}
          </p>
        </div>
        {canInvite && (
          <Button onClick={() => setShowInvite(true)}>
            {t('tenant.members.inviteMember')}
          </Button>
        )}
      </div>

      {error && (
        <div className="rounded-md bg-destructive/10 p-4 mb-6">
          <p className="text-sm text-destructive">{error}</p>
          <Button variant="outline" size="sm" className="mt-2" onClick={fetchData}>
            {t('common.retry')}
          </Button>
        </div>
      )}

      {showInvite && (
        <Card className="mb-6">
          <CardHeader>
            <CardTitle>{t('tenant.members.inviteNew.title')}</CardTitle>
            <CardDescription>{t('tenant.members.inviteNew.desc')}</CardDescription>
          </CardHeader>
          <CardContent>
            <form onSubmit={handleInvite} className="space-y-4">
              <div>
                <label htmlFor="email" className="block text-sm font-medium text-foreground">
                  {t('tenant.members.inviteNew.emailLabel')}
                </label>
                <Input
                  id="email"
                  type="email"
                  value={inviteEmail}
                  onChange={(e) => setInviteEmail(e.target.value)}
                  placeholder={t('tenant.members.inviteNew.emailPlaceholder')}
                  required
                  className="mt-1 w-full"
                  disabled={isInviting}
                />
              </div>
              <div>
                <label htmlFor="role" className="block text-sm font-medium text-foreground">
                  {t('tenant.members.inviteNew.roleLabel')}
                </label>
                <select
                  id="role"
                  value={inviteRole}
                  onChange={(e) => setInviteRole(e.target.value)}
                  className="mt-1 block w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                  disabled={isInviting}
                >
                  <option value="member">{t('tenant.members.roles.member')}</option>
                  <option value="admin">{t('tenant.members.roles.admin')}</option>
                </select>
              </div>
              <div>
                <label htmlFor="message" className="block text-sm font-medium text-foreground">
                  {t('tenant.members.inviteNew.messageLabel')} ({t('common.optional')})
                </label>
                <Input
                  id="message"
                  value={inviteMessage}
                  onChange={(e) => setInviteMessage(e.target.value)}
                  placeholder={t('tenant.members.inviteNew.messagePlaceholder')}
                  className="mt-1 w-full"
                  disabled={isInviting}
                />
              </div>
              {inviteError && (
                <p className="text-sm text-destructive">{inviteError}</p>
              )}
              <div className="flex flex-col sm:flex-row gap-2">
                <Button type="submit" className="w-full sm:w-auto" disabled={isInviting}>
                  {isInviting ? t('common.processing') : t('tenant.members.inviteNew.sendInvitation')}
                </Button>
                <Button
                  type="button"
                  variant="outline"
                  onClick={() => { setShowInvite(false); setInviteError(null); }}
                  className="w-full sm:w-auto"
                  disabled={isInviting}
                >
                  {t('common.cancel')}
                </Button>
              </div>
            </form>
          </CardContent>
        </Card>
      )}

      <div className="flex gap-1 mb-4 border-b border-border">
        <button
          className={`px-4 py-2 text-sm font-medium transition-colors ${
            activeTab === 'members'
              ? 'border-b-2 border-primary text-primary'
              : 'text-muted-foreground hover:text-foreground'
          }`}
          onClick={() => setActiveTab('members')}
        >
          {t('tenant.members.tabs.members')} ({members.length})
        </button>
        <button
          className={`px-4 py-2 text-sm font-medium transition-colors ${
            activeTab === 'invitations'
              ? 'border-b-2 border-primary text-primary'
              : 'text-muted-foreground hover:text-foreground'
          }`}
          onClick={() => setActiveTab('invitations')}
        >
          {t('tenant.members.tabs.invitations')} ({pendingInvitations.length})
        </button>
      </div>

      {activeTab === 'members' && (
        <Card>
          <CardContent className="p-0">
            {isLoading ? (
              <div className="p-6 space-y-4">
                {[1, 2, 3].map((i) => (
                  <div key={i} className="h-12 bg-muted animate-pulse rounded" />
                ))}
              </div>
            ) : members.length === 0 ? (
              <div className="p-6 text-center text-muted-foreground">
                {t('tenant.members.noMembers')}
              </div>
            ) : (
              <div className="overflow-x-auto">
                <table className="w-full min-w-[600px]">
                  <thead>
                    <tr className="border-b border-border">
                      <th className="px-6 py-3 text-left text-xs font-semibold text-muted-foreground uppercase">
                        {t('tenant.members.table.member')}
                      </th>
                      <th className="px-6 py-3 text-left text-xs font-semibold text-muted-foreground uppercase">
                        {t('tenant.members.table.role')}
                      </th>
                      <th className="px-6 py-3 text-left text-xs font-semibold text-muted-foreground uppercase">
                        {t('tenant.members.table.status')}
                      </th>
                      <th className="px-6 py-3 text-left text-xs font-semibold text-muted-foreground uppercase">
                        {t('tenant.members.table.joined')}
                      </th>
                      {canRemoveMember && (
                        <th className="px-6 py-3 text-right text-xs font-semibold text-muted-foreground uppercase">
                          {t('tenant.members.table.actions')}
                        </th>
                      )}
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-border">
                    {members.map((member) => (
                      <tr key={member.id} className="hover:bg-muted/50">
                        <td className="px-6 py-4">
                          <div className="flex items-center gap-3">
                            <div className="flex h-10 w-10 items-center justify-center rounded-full bg-primary-100 dark:bg-primary-900">
                              <span className="text-sm font-medium text-primary-600 dark:text-primary-400">
                                {member.username.charAt(0).toUpperCase()}
                              </span>
                            </div>
                            <div>
                              <p className="font-medium text-foreground">
                                {member.full_name || member.username}
                              </p>
                              <p className="text-sm text-muted-foreground">{member.email}</p>
                            </div>
                          </div>
                        </td>
                        <td className="px-6 py-4">
                          <span className={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium ${getRoleBadgeColor(member.tenant_role || member.role)}`}>
                            {getRoleLabel(member.tenant_role || member.role)}
                          </span>
                        </td>
                        <td className="px-6 py-4">
                          {getStatusBadge(member.status)}
                        </td>
                        <td className="px-6 py-4 text-sm text-muted-foreground">
                          {member.joined_at ? new Date(member.joined_at).toLocaleDateString() : '-'}
                        </td>
                        {canRemoveMember && (member.tenant_role || member.role) !== 'owner' && member.id !== user?.id && (
                          <td className="px-6 py-4 text-right">
                            {confirmRemoveId === member.id ? (
                              <div className="flex items-center justify-end gap-2">
                                <Button
                                  variant="ghost"
                                  size="sm"
                                  className="text-destructive"
                                  onClick={() => handleRemove(member.id)}
                                  disabled={removingId === member.id}
                                >
                                  {removingId === member.id ? t('common.loading') : t('common.confirm')}
                                </Button>
                                <Button
                                  variant="ghost"
                                  size="sm"
                                  onClick={() => setConfirmRemoveId(null)}
                                  disabled={removingId === member.id}
                                >
                                  {t('common.cancel')}
                                </Button>
                              </div>
                            ) : (
                              <Button
                                variant="ghost"
                                size="sm"
                                className="text-destructive"
                                onClick={() => setConfirmRemoveId(member.id)}
                              >
                                {t('common.remove')}
                              </Button>
                            )}
                          </td>
                        )}
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {activeTab === 'invitations' && (
        <Card>
          <CardContent className="p-0">
            {isLoading ? (
              <div className="p-6 space-y-4">
                {[1, 2].map((i) => (
                  <div key={i} className="h-12 bg-muted animate-pulse rounded" />
                ))}
              </div>
            ) : invitations.length === 0 ? (
              <div className="p-6 text-center text-muted-foreground">
                {t('tenant.members.noInvitations')}
              </div>
            ) : (
              <div className="overflow-x-auto">
                <table className="w-full min-w-[600px]">
                  <thead>
                    <tr className="border-b border-border">
                      <th className="px-6 py-3 text-left text-xs font-semibold text-muted-foreground uppercase">
                        {t('tenant.members.table.email')}
                      </th>
                      <th className="px-6 py-3 text-left text-xs font-semibold text-muted-foreground uppercase">
                        {t('tenant.members.table.role')}
                      </th>
                      <th className="px-6 py-3 text-left text-xs font-semibold text-muted-foreground uppercase">
                        {t('tenant.members.table.status')}
                      </th>
                      <th className="px-6 py-3 text-left text-xs font-semibold text-muted-foreground uppercase">
                        {t('tenant.members.table.expires')}
                      </th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-border">
                    {pendingInvitations.map((inv) => (
                      <tr key={inv.id} className="hover:bg-muted/50">
                        <td className="px-6 py-4">
                          <span className="font-medium text-foreground">{inv.email}</span>
                        </td>
                        <td className="px-6 py-4">
                          <span className={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium ${getRoleBadgeColor(inv.role)}`}>
                            {getRoleLabel(inv.role)}
                          </span>
                        </td>
                        <td className="px-6 py-4">
                          {getStatusBadge(inv.status)}
                        </td>
                        <td className="px-6 py-4 text-sm text-muted-foreground">
                          {new Date(inv.expires_at).toLocaleDateString()}
                        </td>
                      </tr>
                    ))}
                    {expiredInvitations.length > 0 && (
                      <>
                        <tr>
                          <td colSpan={4} className="px-6 py-2 text-xs font-semibold text-muted-foreground uppercase bg-muted/30">
                            {t('tenant.members.table.expired')}
                          </td>
                        </tr>
                        {expiredInvitations.map((inv) => (
                          <tr key={inv.id} className="hover:bg-muted/50 opacity-60">
                            <td className="px-6 py-4">
                              <span className="font-medium text-foreground">{inv.email}</span>
                            </td>
                            <td className="px-6 py-4">
                              <span className={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium ${getRoleBadgeColor(inv.role)}`}>
                                {getRoleLabel(inv.role)}
                              </span>
                            </td>
                            <td className="px-6 py-4">
                              {getStatusBadge(inv.status)}
                            </td>
                            <td className="px-6 py-4 text-sm text-muted-foreground">
                              {new Date(inv.expires_at).toLocaleDateString()}
                            </td>
                          </tr>
                        ))}
                      </>
                    )}
                  </tbody>
                </table>
              </div>
            )}
          </CardContent>
        </Card>
      )}

      <div className="mt-6 flex items-center gap-2 text-sm text-muted-foreground">
        <span className="font-medium">{t('tenant.members.planInfo')}</span>
        <span className="capitalize">{tenant?.plan || t('tenant.billing.free')}</span>
        <span>•</span>
        <span>{t('tenant.members.seatsUsed', { used: members.length, total: 10 })}</span>
      </div>
    </div>
  );
}
