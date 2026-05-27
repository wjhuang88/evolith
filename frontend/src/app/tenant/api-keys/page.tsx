'use client';

import { useState, useEffect, useCallback } from 'react';
import { useTranslation } from 'react-i18next';
import { useAuthStore } from '@/stores';
import { apiKeysApi } from '@/lib/api/api-keys';
import { useTenantPermissions } from '@/hooks/usePermission';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';
import type { ApiKey } from '@/lib/api/types';

export default function ApiKeysPage() {
  const { t } = useTranslation();
  const { user } = useAuthStore();
  const { canManageApiKeys } = useTenantPermissions();

  const tenantId = user?.tenant_id ?? '';

  const [apiKeys, setApiKeys] = useState<ApiKey[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const [showCreate, setShowCreate] = useState(false);
  const [newKeyName, setNewKeyName] = useState('');
  const [isCreating, setIsCreating] = useState(false);
  const [createError, setCreateError] = useState<string | null>(null);
  const [newlyCreatedKey, setNewlyCreatedKey] = useState<string | null>(null);

  const [revokingId, setRevokingId] = useState<string | null>(null);
  const [confirmRevokeId, setConfirmRevokeId] = useState<string | null>(null);

  const fetchKeys = useCallback(async () => {
    if (!tenantId) return;
    setIsLoading(true);
    setError(null);
    try {
      const response = await apiKeysApi.list(tenantId);
      if (response.success && response.data) {
        setApiKeys(response.data.keys);
      } else {
        setError(response.error?.message ?? t('common.error'));
      }
    } catch {
      setError(t('common.networkError'));
    } finally {
      setIsLoading(false);
    }
  }, [tenantId, t]);

  useEffect(() => {
    fetchKeys();
  }, [fetchKeys]);

  const handleCreateKey = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!tenantId || !newKeyName.trim()) return;

    setIsCreating(true);
    setCreateError(null);
    try {
      const response = await apiKeysApi.create(tenantId, { name: newKeyName.trim() });
      if (response.success && response.data) {
        if (response.data.key) {
          setNewlyCreatedKey(response.data.key);
        }
        setNewKeyName('');
        setShowCreate(false);
        await fetchKeys();
      } else {
        setCreateError(response.error?.message ?? t('common.error'));
      }
    } catch {
      setCreateError(t('common.networkError'));
    } finally {
      setIsCreating(false);
    }
  };

  const handleRevokeKey = async (keyId: string) => {
    if (!tenantId) return;

    setRevokingId(keyId);
    try {
      const response = await apiKeysApi.revoke(tenantId, keyId);
      if (response.success) {
        setConfirmRevokeId(null);
        await fetchKeys();
      } else {
        setError(response.error?.message ?? t('common.error'));
      }
    } catch {
      setError(t('common.networkError'));
    } finally {
      setRevokingId(null);
    }
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  return (
    <div className="container mx-auto py-8">
      <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 mb-8">
        <div>
          <h1 className="text-3xl font-bold">{t('tenant.apiKeys.title')}</h1>
          <p className="text-muted-foreground mt-1">
            {t('tenant.apiKeys.subtitle')}
          </p>
        </div>
        {canManageApiKeys && (
          <Button onClick={() => setShowCreate(true)}>
            {t('tenant.apiKeys.createApiKey')}
          </Button>
        )}
      </div>

      {!canManageApiKeys && (
        <div className="rounded-md bg-amber-50 dark:bg-amber-900/20 p-4 mb-6">
          <p className="text-sm text-amber-800 dark:text-amber-200">
            {t('tenant.apiKeys.adminOnly')}
          </p>
        </div>
      )}

      {error && (
        <div className="rounded-md bg-destructive/10 p-4 mb-6">
          <p className="text-sm text-destructive">{error}</p>
          <Button variant="outline" size="sm" className="mt-2" onClick={fetchKeys}>
            {t('common.retry')}
          </Button>
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
                  disabled={isCreating}
                />
              </div>
              {createError && (
                <p className="text-sm text-destructive">{createError}</p>
              )}
              <div className="flex flex-col sm:flex-row gap-2">
                <Button type="submit" className="w-full sm:w-auto" disabled={isCreating}>
                  {isCreating ? t('common.creating') : t('tenant.apiKeys.createNew.createKey')}
                </Button>
                <Button
                  type="button"
                  variant="outline"
                  onClick={() => { setShowCreate(false); setCreateError(null); }}
                  className="w-full sm:w-auto"
                  disabled={isCreating}
                >
                  {t('common.cancel')}
                </Button>
              </div>
            </form>
          </CardContent>
        </Card>
      )}

      <Card>
        <CardContent className="p-0">
          {isLoading ? (
            <div className="p-6 space-y-4">
              {[1, 2, 3].map((i) => (
                <div key={i} className="h-12 bg-muted animate-pulse rounded" />
              ))}
            </div>
          ) : apiKeys.length === 0 ? (
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
                      {t('tenant.apiKeys.table.status')}
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-semibold text-muted-foreground uppercase">
                      {t('tenant.apiKeys.table.created')}
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-semibold text-muted-foreground uppercase">
                      {t('tenant.apiKeys.table.lastUsed')}
                    </th>
                    {canManageApiKeys && (
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
                        {key.permissions.length > 0 && (
                          <div className="flex gap-1 mt-1">
                            {key.permissions.map((p) => (
                              <span key={p} className="text-xs bg-muted px-1.5 py-0.5 rounded">
                                {p}
                              </span>
                            ))}
                          </div>
                        )}
                      </td>
                      <td className="px-6 py-4">
                        <code className="text-sm text-muted-foreground">{key.key_prefix}****</code>
                      </td>
                      <td className="px-6 py-4">
                        <span className={`inline-flex items-center text-xs font-medium px-2 py-1 rounded-full ${
                          key.status === 'active'
                            ? 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400'
                            : key.status === 'revoked'
                              ? 'bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400'
                              : 'bg-gray-100 text-gray-700 dark:bg-gray-900/30 dark:text-gray-400'
                        }`}>
                          {key.status}
                        </span>
                      </td>
                      <td className="px-6 py-4 text-sm text-muted-foreground">
                        {new Date(key.created_at).toLocaleDateString()}
                      </td>
                      <td className="px-6 py-4 text-sm text-muted-foreground">
                        {key.last_used_at
                          ? new Date(key.last_used_at).toLocaleDateString()
                          : t('common.never')}
                      </td>
                      {canManageApiKeys && (
                        <td className="px-6 py-4 text-right">
                          {key.status === 'active' && (
                            <>
                              {confirmRevokeId === key.id ? (
                                <div className="flex items-center justify-end gap-2">
                                  <Button
                                    variant="ghost"
                                    size="sm"
                                    className="text-destructive"
                                    onClick={() => handleRevokeKey(key.id)}
                                    disabled={revokingId === key.id}
                                  >
                                    {revokingId === key.id ? t('common.loading') : t('common.confirm')}
                                  </Button>
                                  <Button
                                    variant="ghost"
                                    size="sm"
                                    onClick={() => setConfirmRevokeId(null)}
                                    disabled={revokingId === key.id}
                                  >
                                    {t('common.cancel')}
                                  </Button>
                                </div>
                              ) : (
                                <Button
                                  variant="ghost"
                                  size="sm"
                                  className="text-destructive"
                                  onClick={() => setConfirmRevokeId(key.id)}
                                >
                                  {t('tenant.apiKeys.revoke')}
                                </Button>
                              )}
                            </>
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
