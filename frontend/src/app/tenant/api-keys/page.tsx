'use client';

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
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
  const { t } = useTranslation();
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
      <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 mb-8">
        <div>
          <h1 className="text-3xl font-bold">{t('tenant.apiKeys.title')}</h1>
          <p className="text-muted-foreground mt-1">
            {t('tenant.apiKeys.subtitle')}
          </p>
        </div>
        {canManageKeys && (
          <Button onClick={() => setShowCreate(true)}>
            {t('tenant.apiKeys.createApiKey')}
          </Button>
        )}
      </div>

      {!canManageKeys && (
        <div className="rounded-md bg-amber-50 dark:bg-amber-900/20 p-4 mb-6">
          <p className="text-sm text-amber-800 dark:text-amber-200">
            {t('tenant.apiKeys.adminOnly')}
          </p>
        </div>
      )}

      {newlyCreatedKey && (
        <Card className="mb-6 border-green-500">
          <CardHeader className="bg-green-50 dark:bg-green-900/20">
            <CardTitle className="text-green-700 dark:text-green-300">{t('tenant.apiKeys.keyCreated')}</CardTitle>
            <CardDescription>
              {t('tenant.apiKeys.keyCreatedDesc')}
            </CardDescription>
          </CardHeader>
          <CardContent className="pt-4">
            <div className="flex flex-col sm:flex-row items-stretch sm:items-center gap-2">
              <code className="flex-1 rounded bg-muted px-3 py-2 font-mono text-sm break-all">
                {newlyCreatedKey}
              </code>
              <Button variant="outline" onClick={() => copyToClipboard(newlyCreatedKey)}>
                {t('common.copy')}
              </Button>
            </div>
            <Button 
              variant="ghost" 
              className="mt-4" 
              onClick={() => setNewlyCreatedKey(null)}
            >
              {t('tenant.apiKeys.storedSecurely')}
            </Button>
          </CardContent>
        </Card>
      )}

      {showCreate && (
        <Card className="mb-6">
          <CardHeader>
            <CardTitle>{t('tenant.apiKeys.createNew.title')}</CardTitle>
            <CardDescription>{t('tenant.apiKeys.createNew.desc')}</CardDescription>
          </CardHeader>
          <CardContent>
            <form onSubmit={handleCreateKey} className="space-y-4">
              <div>
                <label htmlFor="keyName" className="block text-sm font-medium text-foreground">
                  {t('tenant.apiKeys.createNew.nameLabel')}
                </label>
                <Input
                  id="keyName"
                  value={newKeyName}
                  onChange={(e) => setNewKeyName(e.target.value)}
                  placeholder={t('tenant.apiKeys.createNew.namePlaceholder')}
                  required
                  className="mt-1 w-full"
                />
              </div>
              <div className="flex flex-col sm:flex-row gap-2">
                <Button type="submit" className="w-full sm:w-auto">{t('tenant.apiKeys.createNew.createKey')}</Button>
                <Button type="button" variant="outline" onClick={() => setShowCreate(false)} className="w-full sm:w-auto">
                  {t('common.cancel')}
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
              {t('tenant.apiKeys.noKeys')}
            </div>
          ) : (
            <div className="overflow-x-auto">
              <table className="w-full min-w-[600px]">
              <thead>
                <tr className="border-b border-border">
                  <th className="px-6 py-3 text-left text-xs font-semibold text-muted-foreground uppercase">
                    {t('tenant.apiKeys.table.name')}
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-semibold text-muted-foreground uppercase">
                    {t('tenant.apiKeys.table.key')}
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-semibold text-muted-foreground uppercase">
                    {t('tenant.apiKeys.table.created')}
                  </th>
                  {canManageKeys && (
                    <th className="px-6 py-3 text-right text-xs font-semibold text-muted-foreground uppercase">
                      {t('tenant.apiKeys.table.actions')}
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
                             {t('tenant.apiKeys.revoke')}
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
    </div>
  );
}