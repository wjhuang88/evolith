'use client';

import { useState } from 'react';
import { useAuthStore } from '@/stores';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';

interface Member {
  id: string;
  email: string;
  username: string;
  full_name?: string;
  role: 'owner' | 'admin' | 'member';
  status: 'active' | 'invited';
  joined_at?: string;
  avatar_url?: string;
}

export default function MembersPage() {
  const { user, tenant } = useAuthStore();
  const [showInvite, setShowInvite] = useState(false);
  const [inviteEmail, setInviteEmail] = useState('');
  const [inviteRole, setInviteRole] = useState<'admin' | 'member'>('member');
  const [members, setMembers] = useState<Member[]>([
    {
      id: '1',
      email: user?.email || 'owner@example.com',
      username: user?.username || 'owner',
      full_name: 'Owner User',
      role: 'owner',
      status: 'active',
      joined_at: new Date().toISOString(),
    },
  ]);

  const handleInvite = async (e: React.FormEvent) => {
    e.preventDefault();
    // TODO: Call API to send invite
    setMembers([...members, {
      id: Date.now().toString(),
      email: inviteEmail,
      username: inviteEmail.split('@')[0],
      role: inviteRole,
      status: 'invited',
    }]);
    setInviteEmail('');
    setShowInvite(false);
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

  const currentUserRole = user?.tenant_role as string || 'member';

  return (
    <div className="container mx-auto py-8">
      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-3xl font-bold">Team Members</h1>
          <p className="text-muted-foreground mt-1">
            Manage your organization team members and roles
          </p>
        </div>
        {currentUserRole !== 'member' && (
          <Button onClick={() => setShowInvite(true)}>
            Invite Member
          </Button>
        )}
      </div>

      {showInvite && (
        <Card className="mb-6">
          <CardHeader>
            <CardTitle>Invite New Member</CardTitle>
            <CardDescription>Send an invitation to join your organization</CardDescription>
          </CardHeader>
          <CardContent>
            <form onSubmit={handleInvite} className="space-y-4">
              <div>
                <label htmlFor="email" className="block text-sm font-medium text-foreground">
                  Email Address
                </label>
                <Input
                  id="email"
                  type="email"
                  value={inviteEmail}
                  onChange={(e) => setInviteEmail(e.target.value)}
                  placeholder="colleague@company.com"
                  required
                  className="mt-1"
                />
              </div>
              <div>
                <label htmlFor="role" className="block text-sm font-medium text-foreground">
                  Role
                </label>
                <select
                  id="role"
                  value={inviteRole}
                  onChange={(e) => setInviteRole(e.target.value as 'admin' | 'member')}
                  className="mt-1 block w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                >
                  <option value="member">Member</option>
                  <option value="admin">Admin</option>
                </select>
              </div>
              <div className="flex gap-2">
                <Button type="submit">Send Invitation</Button>
                <Button type="button" variant="outline" onClick={() => setShowInvite(false)}>
                  Cancel
                </Button>
              </div>
            </form>
          </CardContent>
        </Card>
      )}

      <Card>
        <CardContent className="p-0">
          <table className="w-full">
            <thead>
              <tr className="border-b border-border">
                <th className="px-6 py-3 text-left text-xs font-semibold text-muted-foreground uppercase">
                  Member
                </th>
                <th className="px-6 py-3 text-left text-xs font-semibold text-muted-foreground uppercase">
                  Role
                </th>
                <th className="px-6 py-3 text-left text-xs font-semibold text-muted-foreground uppercase">
                  Status
                </th>
                <th className="px-6 py-3 text-left text-xs font-semibold text-muted-foreground uppercase">
                  Joined
                </th>
                {(currentUserRole === 'owner' || currentUserRole === 'admin') && (
                  <th className="px-6 py-3 text-right text-xs font-semibold text-muted-foreground uppercase">
                    Actions
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
                    <span className={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium ${getRoleBadgeColor(member.role)}`}>
                      {member.role}
                    </span>
                  </td>
                  <td className="px-6 py-4">
                    {member.status === 'invited' ? (
                      <span className="text-sm text-amber-600">Pending</span>
                    ) : (
                      <span className="text-sm text-green-600">Active</span>
                    )}
                  </td>
                  <td className="px-6 py-4 text-sm text-muted-foreground">
                    {member.joined_at ? new Date(member.joined_at).toLocaleDateString() : '-'}
                  </td>
                  {(currentUserRole === 'owner' || currentUserRole === 'admin') && member.role !== 'owner' && (
                    <td className="px-6 py-4 text-right">
                      <Button variant="ghost" size="sm" className="text-destructive">
                        Remove
                      </Button>
                    </td>
                  )}
                </tr>
              ))}
            </tbody>
          </table>
        </CardContent>
      </Card>

      <div className="mt-6 flex items-center gap-2 text-sm text-muted-foreground">
        <span className="font-medium">Your plan:</span>
        <span className="capitalize">{tenant?.plan || 'Free'}</span>
        <span>•</span>
        <span>{members.length} of {10} seats used</span>
      </div>
    </div>
  );
}