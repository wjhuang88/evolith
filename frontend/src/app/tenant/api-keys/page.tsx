'use client';

import { useState } from 'react';
import { useAuthStore } from '@/stores';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';

interface ApiKey {
  id: string;
  name: string;
  key_prefix: string;
  permissions: string[];
  created_at: string;
  last_used_at?: string;
  expires_at?: string;
  status: 'active' | 'revoked';
}

export default function ApiKeysPage() {
  const { user } = useAuthStore();
  const [showCreate, setShowCreate] = useState(false);
  const [newKeyName, setNewKeyName] = useState('');
  const [newKeyPermissions, setNewKeyPermissions] = useState<string[]>(['read']);
  const [newlyCreatedKey, setNewlyCreatedKey] = useState<string | null>(null);
  const [apiKeys] = useState<ApiKey[]>([
    {
      id: '1',
      name: 'Development',
      key_prefix: 'evo_sk_abc1...',
      permissions: ['read', 'write'],
      created_at: new Date(Date.now() - 7 * 24 * 60 * 60 * 1000).toISOString(),
      last_used_at: new Date(Date.now() - 1 * 24 * 60 * 60 * 1000).toISOString(),
      status: 'active',
    },
  ]);

  const handleCreateKey = async (e: React.FormEvent) => {
    e.preventDefault();
    const mockKey = `evo_sk_${Math.random().toString(36).substring(2, 18)}`;
    setNewlyCreatedKey(mockKey);
    setNewKeyName('');
    setNewKeyPermissions(['read']);
  };

  const handleRevokeKey = (id: string) => {
    // TODO: Implement revoke
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  const currentUserRole = user?.tenant_role as string || 'member';
  const canManageKeys = currentUserRole === 'owner' || currentUserRole === 'admin';

  return (
    <div className="container mx-auto py-8">
      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-3xl font-bold">API Keys</h1>
          <p className="text-muted-foreground mt-1">
            Manage API keys for programmatic access to your account
          </p>
        </div>
        {canManageKeys && (
          <Button onClick={() => setShowCreate(true)}>
            Create API Key
          </Button>
        )}
      </div>

      {!canManageKeys && (
        <div className="rounded-md bg-amber-50 dark:bg-amber-900/20 p-4 mb-6">
          <p className="text-sm text-amber-800 dark:text-amber-200">
            Only admins and owners can manage API keys.
          </p>
        </div>
      )}

      {newlyCreatedKey && (
        <Card className="mb-6 border-green-500">
          <CardHeader className="bg-green-50 dark:bg-green-900/20">
            <CardTitle className="text-green-700 dark:text-green-300">API Key Created</CardTitle>
            <CardDescription>
              Make sure to copy your API key now. You won&apos;t be able to see it again!
            </CardDescription>
          </CardHeader>
          <CardContent className="pt-4">
            <div className="flex items-center gap-2">
              <code className="flex-1 rounded bg-muted px-3 py-2 font-mono text-sm">
                {newlyCreatedKey}
              </code>
              <Button variant="outline" onClick={() => copyToClipboard(newlyCreatedKey)}>
                Copy
              </Button>
            </div>
            <Button 
              variant="ghost" 
              className="mt-4" 
              onClick={() => setNewlyCreatedKey(null)}
            >
              I&apos;ve stored it securely
            </Button>
          </CardContent>
        </Card>
      )}

      {showCreate && (
        <Card className="mb-6">
          <CardHeader>
            <CardTitle>Create New API Key</CardTitle>
            <CardDescription>Generate a new API key for programmatic access</CardDescription>
          </CardHeader>
          <CardContent>
            <form onSubmit={handleCreateKey} className="space-y-4">
              <div>
                <label htmlFor="keyName" className="block text-sm font-medium text-foreground">
                  Key Name
                </label>
                <Input
                  id="keyName"
                  value={newKeyName}
                  onChange={(e) => setNewKeyName(e.target.value)}
                  placeholder="My API Key"
                  required
                  className="mt-1"
                />
              </div>
              <div className="flex gap-2">
                <Button type="submit">Create Key</Button>
                <Button type="button" variant="outline" onClick={() => setShowCreate(false)}>
                  Cancel
                </Button>
              </div>
            </form>
          </CardContent>
        </Card>
      )}

      <Card>
        <CardContent className="p-0">
          {apiKeys.length === 0 ? (
            <div className="p-6 text-center text-muted-foreground">
              No API keys yet. Create one to get started.
            </div>
          ) : (
            <table className="w-full">
              <thead>
                <tr className="border-b border-border">
                  <th className="px-6 py-3 text-left text-xs font-semibold text-muted-foreground uppercase">
                    Name
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-semibold text-muted-foreground uppercase">
                    Key
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-semibold text-muted-foreground uppercase">
                    Created
                  </th>
                  {canManageKeys && (
                    <th className="px-6 py-3 text-right text-xs font-semibold text-muted-foreground uppercase">
                      Actions
                    </th>
                  )}
                </tr>
              </thead>
              <tbody className="divide-y divide-border">
                {apiKeys.map((key) => (
                  <tr key={key.id} className="hover:bg-muted/50">
                    <td className="px-6 py-4">
                      <span className="font-medium text-foreground">{key.name}</span>
                    </td>
                    <td className="px-6 py-4">
                      <code className="text-sm text-muted-foreground">{key.key_prefix}****</code>
                    </td>
                    <td className="px-6 py-4 text-sm text-muted-foreground">
                      {new Date(key.created_at).toLocaleDateString()}
                    </td>
                    {canManageKeys && (
                      <td className="px-6 py-4 text-right">
                        {key.status === 'active' && (
                          <Button 
                            variant="ghost" 
                            size="sm" 
                            className="text-destructive"
                          >
                            Revoke
                          </Button>
                        )}
                      </td>
                    )}
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </CardContent>
      </Card>
    </div>
  );
}