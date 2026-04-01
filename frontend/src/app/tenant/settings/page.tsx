'use client';

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useAuthStore } from '@/stores';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';

export default function SettingsPage() {
  const { t } = useTranslation();
  const { user, tenant } = useAuthStore();
  const [tenantName, setTenantName] = useState(tenant?.name || '');
  const [saving, setSaving] = useState(false);
  
  const currentUserRole = user?.tenant_role as string || 'member';
  const canEditSettings = currentUserRole === 'owner' || currentUserRole === 'admin';

  const handleSave = async () => {
    setSaving(true);
    // TODO: Call API to save settings
    setTimeout(() => setSaving(false), 1000);
  };

  return (
    <div className="container mx-auto py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold">{t('tenant.settings.title')}</h1>
        <p className="text-muted-foreground mt-1">
          {t('tenant.settings.subtitle')}
        </p>
      </div>

      <div className="space-y-6 max-w-3xl">
        {/* Organization Settings */}
        <Card>
          <CardHeader>
            <CardTitle>{t('tenant.settings.org.title')}</CardTitle>
            <CardDescription>{t('tenant.settings.org.desc')}</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div>
              <label htmlFor="tenantName" className="block text-sm font-medium text-foreground">
                {t('tenant.settings.org.nameLabel')}
              </label>
              <Input
                id="tenantName"
                value={tenantName}
                onChange={(e) => setTenantName(e.target.value)}
                disabled={!canEditSettings}
                className="mt-1 w-full max-w-md"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-foreground">
                {t('tenant.settings.org.urlLabel')}
              </label>
              <div className="mt-1 flex flex-col sm:flex-row max-w-md items-stretch sm:items-center gap-2">
                <span className="inline-flex items-center rounded-md border border-input bg-muted px-3 text-sm text-muted-foreground">
                  evolith.io/
                </span>
                <Input
                  value={tenant?.slug || 'my-company'}
                  disabled
                  className="flex-1"
                />
              </div>
            </div>
            <div>
              <label className="block text-sm font-medium text-foreground">
                {t('tenant.settings.org.planLabel')}
              </label>
              <p className="mt-1 text-sm text-muted-foreground capitalize">
                {tenant?.plan || 'Free'}
              </p>
            </div>
            {canEditSettings && (
              <Button onClick={handleSave} disabled={saving} className="mt-4">
                {saving ? t('common.saving') : t('tenant.settings.org.saveChanges')}
              </Button>
            )}
          </CardContent>
        </Card>

        {/* Security Settings */}
        <Card>
          <CardHeader>
            <CardTitle>{t('tenant.settings.security.title')}</CardTitle>
            <CardDescription>{t('tenant.settings.security.desc')}</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 py-3 border-b border-border">
              <div>
                <p className="font-medium text-foreground">{t('tenant.settings.security.twoFactor')}</p>
                <p className="text-sm text-muted-foreground">{t('tenant.settings.security.twoFactorDesc')}</p>
              </div>
              <Button variant="outline" size="sm" disabled={!canEditSettings} className="w-full sm:w-auto">
                {t('common.enable')}
              </Button>
            </div>
            <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 py-3 border-b border-border">
              <div>
                <p className="font-medium text-foreground">{t('tenant.settings.security.sessionTimeout')}</p>
                <p className="text-sm text-muted-foreground">{t('tenant.settings.security.sessionTimeoutDesc')}</p>
              </div>
              <select 
                className="w-full sm:w-auto rounded-md border border-input bg-background px-3 py-2 text-sm"
                defaultValue="24h"
              >
                <option value="1h">{t('tenant.settings.security.1h')}</option>
                <option value="24h">{t('tenant.settings.security.24h')}</option>
                <option value="7d">{t('tenant.settings.security.7d')}</option>
                <option value="30d">{t('tenant.settings.security.30d')}</option>
              </select>
            </div>
            <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 py-3">
              <div>
                <p className="font-medium text-foreground">{t('tenant.settings.security.apiKeyReq')}</p>
                <p className="text-sm text-muted-foreground">{t('tenant.settings.security.apiKeyReqDesc')}</p>
              </div>
              <Button variant="outline" size="sm" disabled={!canEditSettings} className="w-full sm:w-auto">
                {t('common.configure')}
              </Button>
            </div>
          </CardContent>
        </Card>

        {/* Notification Settings */}
        <Card>
          <CardHeader>
            <CardTitle>{t('tenant.settings.notifications.title')}</CardTitle>
            <CardDescription>{t('tenant.settings.notifications.desc')}</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 py-3 border-b border-border">
              <div>
                <p className="font-medium text-foreground">{t('tenant.settings.notifications.email')}</p>
                <p className="text-sm text-muted-foreground">{t('tenant.settings.notifications.emailDesc')}</p>
              </div>
              <input 
                type="checkbox" 
                defaultChecked 
                className="h-4 w-4 rounded border-input"
              />
            </div>
            <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 py-3 border-b border-border">
              <div>
                <p className="font-medium text-foreground">{t('tenant.settings.notifications.usage')}</p>
                <p className="text-sm text-muted-foreground">{t('tenant.settings.notifications.usageDesc')}</p>
              </div>
              <input 
                type="checkbox" 
                defaultChecked 
                className="h-4 w-4 rounded border-input"
              />
            </div>
            <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 py-3">
              <div>
                <p className="font-medium text-foreground">{t('tenant.settings.notifications.security')}</p>
                <p className="text-sm text-muted-foreground">{t('tenant.settings.notifications.securityDesc')}</p>
              </div>
              <input 
                type="checkbox" 
                defaultChecked 
                className="h-4 w-4 rounded border-input"
              />
            </div>
          </CardContent>
        </Card>

        {/* Danger Zone */}
        <Card className="border-destructive">
          <CardHeader>
            <CardTitle className="text-destructive">{t('tenant.settings.danger.title')}</CardTitle>
            <CardDescription>{t('tenant.settings.danger.desc')}</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 py-3 border-b border-border">
              <div>
                <p className="font-medium text-foreground">{t('tenant.settings.danger.deleteOrg')}</p>
                <p className="text-sm text-muted-foreground">{t('tenant.settings.danger.deleteOrgDesc')}</p>
              </div>
              <Button variant="destructive" size="sm" disabled={currentUserRole !== 'owner'} className="w-full sm:w-auto">
                {t('common.delete')}
              </Button>
            </div>
            {currentUserRole !== 'owner' && (
              <p className="text-xs text-muted-foreground">
                {t('tenant.settings.danger.ownerOnly')}
              </p>
            )}
          </CardContent>
        </Card>
      </div>
    </div>
  );
}