'use client';

import { useState } from 'react';
import { useAuthStore } from '@/stores';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';

export default function SettingsPage() {
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
        <h1 className="text-3xl font-bold">Settings</h1>
        <p className="text-muted-foreground mt-1">
          Manage your organization settings and preferences
        </p>
      </div>

      <div className="space-y-6 max-w-3xl">
        {/* Organization Settings */}
        <Card>
          <CardHeader>
            <CardTitle>Organization</CardTitle>
            <CardDescription>Basic information about your organization</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div>
              <label htmlFor="tenantName" className="block text-sm font-medium text-foreground">
                Organization Name
              </label>
              <Input
                id="tenantName"
                value={tenantName}
                onChange={(e) => setTenantName(e.target.value)}
                disabled={!canEditSettings}
                className="mt-1 max-w-md"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-foreground">
                Organization URL
              </label>
              <div className="mt-1 flex max-w-md items-center">
                <span className="inline-flex items-center rounded-l-md border border-r-0 border-input bg-muted px-3 text-sm text-muted-foreground">
                  evolith.io/
                </span>
                <Input
                  value={tenant?.slug || 'my-company'}
                  disabled
                  className="rounded-l-none"
                />
              </div>
            </div>
            <div>
              <label className="block text-sm font-medium text-foreground">
                Plan
              </label>
              <p className="mt-1 text-sm text-muted-foreground capitalize">
                {tenant?.plan || 'Free'}
              </p>
            </div>
            {canEditSettings && (
              <Button onClick={handleSave} disabled={saving} className="mt-4">
                {saving ? 'Saving...' : 'Save Changes'}
              </Button>
            )}
          </CardContent>
        </Card>

        {/* Security Settings */}
        <Card>
          <CardHeader>
            <CardTitle>Security</CardTitle>
            <CardDescription>Security and authentication settings</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex items-center justify-between py-3 border-b border-border">
              <div>
                <p className="font-medium text-foreground">Two-Factor Authentication</p>
                <p className="text-sm text-muted-foreground">Add an extra layer of security to your account</p>
              </div>
              <Button variant="outline" size="sm" disabled={!canEditSettings}>
                Enable
              </Button>
            </div>
            <div className="flex items-center justify-between py-3 border-b border-border">
              <div>
                <p className="font-medium text-foreground">Session Timeout</p>
                <p className="text-sm text-muted-foreground">Automatically log out after period of inactivity</p>
              </div>
              <select 
                className="rounded-md border border-input bg-background px-3 py-2 text-sm"
                defaultValue="24h"
              >
                <option value="1h">1 hour</option>
                <option value="24h">24 hours</option>
                <option value="7d">7 days</option>
                <option value="30d">30 days</option>
              </select>
            </div>
            <div className="flex items-center justify-between py-3">
              <div>
                <p className="font-medium text-foreground">API Key Requirements</p>
                <p className="text-sm text-muted-foreground">Require API keys for programmatic access</p>
              </div>
              <Button variant="outline" size="sm" disabled={!canEditSettings}>
                Configure
              </Button>
            </div>
          </CardContent>
        </Card>

        {/* Notification Settings */}
        <Card>
          <CardHeader>
            <CardTitle>Notifications</CardTitle>
            <CardDescription>Choose how you want to be notified</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex items-center justify-between py-3 border-b border-border">
              <div>
                <p className="font-medium text-foreground">Email Notifications</p>
                <p className="text-sm text-muted-foreground">Receive email updates about your account</p>
              </div>
              <input 
                type="checkbox" 
                defaultChecked 
                className="h-4 w-4 rounded border-input"
              />
            </div>
            <div className="flex items-center justify-between py-3 border-b border-border">
              <div>
                <p className="font-medium text-foreground">Usage Alerts</p>
                <p className="text-sm text-muted-foreground">Get notified when approaching usage limits</p>
              </div>
              <input 
                type="checkbox" 
                defaultChecked 
                className="h-4 w-4 rounded border-input"
              />
            </div>
            <div className="flex items-center justify-between py-3">
              <div>
                <p className="font-medium text-foreground">Security Alerts</p>
                <p className="text-sm text-muted-foreground">Get notified about suspicious activity</p>
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
            <CardTitle className="text-destructive">Danger Zone</CardTitle>
            <CardDescription>Irreversible and destructive actions</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex items-center justify-between py-3 border-b border-border">
              <div>
                <p className="font-medium text-foreground">Delete Organization</p>
                <p className="text-sm text-muted-foreground">Permanently delete your organization and all data</p>
              </div>
              <Button variant="destructive" size="sm" disabled={currentUserRole !== 'owner'}>
                Delete
              </Button>
            </div>
            {currentUserRole !== 'owner' && (
              <p className="text-xs text-muted-foreground">
                Only the organization owner can delete the organization.
              </p>
            )}
          </CardContent>
        </Card>
      </div>
    </div>
  );
}