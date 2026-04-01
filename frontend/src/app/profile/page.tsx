'use client';

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { Form, FormField, FormLabel, FormControl, FormMessage } from '@/components/ui/Form';
import { useAuthStore } from '@/stores';
import { authApi } from '@/lib/api/auth';

export default function ProfilePage() {
  const { t } = useTranslation();
  const { user, fetchUser } = useAuthStore();
  
  const [profileForm, setProfileForm] = useState({
    username: user?.username || '',
    email: user?.email || '',
  });
  const [profileLoading, setProfileLoading] = useState(false);
  const [profileError, setProfileError] = useState<string | null>(null);
  const [profileSuccess, setProfileSuccess] = useState<string | null>(null);

  const [passwordForm, setPasswordForm] = useState({
    currentPassword: '',
    newPassword: '',
    confirmPassword: '',
  });
  const [passwordLoading, setPasswordLoading] = useState(false);
  const [passwordError, setPasswordError] = useState<string | null>(null);
  const [passwordSuccess, setPasswordSuccess] = useState<string | null>(null);

  const validateProfileForm = () => {
    const errors: Record<string, string> = {};
    
    if (!profileForm.username || profileForm.username.trim() === '') {
      errors.username = t('profile.errors.usernameRequired');
    }
    
    if (!profileForm.email || profileForm.email.trim() === '') {
      errors.email = t('profile.errors.emailRequired');
    } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(profileForm.email)) {
      errors.email = t('profile.errors.invalidEmail');
    }
    
    return errors;
  };

  const validatePasswordForm = () => {
    const errors: Record<string, string> = {};
    
    if (!passwordForm.currentPassword) {
      errors.currentPassword = t('profile.errors.currentPasswordRequired');
    }
    
    if (!passwordForm.newPassword) {
      errors.newPassword = t('profile.errors.newPasswordRequired');
    } else if (passwordForm.newPassword.length < 8) {
      errors.newPassword = t('profile.errors.passwordTooShort');
    }
    
    if (!passwordForm.confirmPassword) {
      errors.confirmPassword = t('profile.errors.confirmPasswordRequired');
    } else if (passwordForm.newPassword !== passwordForm.confirmPassword) {
      errors.confirmPassword = t('profile.errors.passwordsDoNotMatch');
    }
    
    return errors;
  };

  const handleProfileUpdate = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setProfileError(null);
    setProfileSuccess(null);
    
    const errors = validateProfileForm();
    if (Object.keys(errors).length > 0) {
      setProfileError(Object.values(errors)[0]);
      return;
    }
    
    setProfileLoading(true);
    
    try {
      const response = await authApi.updateProfile({
        username: profileForm.username,
        email: profileForm.email,
      });
      
      if (response.success && response.data) {
        setProfileSuccess(t('profile.messages.profileUpdated'));
        await fetchUser();
      } else {
        setProfileError(response.error?.message || t('profile.errors.updateFailed'));
      }
    } catch (error) {
      setProfileError(error instanceof Error ? error.message : t('profile.errors.updateFailed'));
    } finally {
      setProfileLoading(false);
    }
  };

  const handlePasswordChange = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setPasswordError(null);
    setPasswordSuccess(null);
    
    const errors = validatePasswordForm();
    if (Object.keys(errors).length > 0) {
      setPasswordError(Object.values(errors)[0]);
      return;
    }
    
    setPasswordLoading(true);
    
    try {
      const response = await authApi.changePassword(
        passwordForm.currentPassword,
        passwordForm.newPassword
      );
      
      if (response.success) {
        setPasswordSuccess(t('profile.messages.passwordChanged'));
        setPasswordForm({
          currentPassword: '',
          newPassword: '',
          confirmPassword: '',
        });
      } else {
        setPasswordError(response.error?.message || t('profile.errors.passwordChangeFailed'));
      }
    } catch (error) {
      setPasswordError(error instanceof Error ? error.message : t('profile.errors.passwordChangeFailed'));
    } finally {
      setPasswordLoading(false);
    }
  };

  if (!user) {
    return null;
  }

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">
          {t('profile.title')}
        </h1>
        <p className="text-muted-foreground">
          {t('profile.subtitle')}
        </p>
      </div>

      <div className="grid gap-6 md:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle>{t('profile.profileInformation.title')}</CardTitle>
            <CardDescription>{t('profile.profileInformation.description')}</CardDescription>
          </CardHeader>
          <CardContent>
            <Form onSubmit={handleProfileUpdate} errors={{}}>
              <div className="mb-4 p-4 rounded-md bg-muted">
                <div className="grid gap-2 text-sm">
                  <div className="flex justify-between">
                    <span className="text-muted-foreground">{t('profile.role')}</span>
                    <span className="font-medium capitalize">{user.role}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-muted-foreground">{t('profile.tenantRole')}</span>
                    <span className="font-medium capitalize">{user.tenant_role}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-muted-foreground">{t('profile.emailVerified')}</span>
                    <span className="font-medium text-green-600">{t('common.yes')}</span>
                  </div>
                </div>
              </div>

              <FormField name="username">
                <FormLabel required>{t('profile.username')}</FormLabel>
                <FormControl>
                  <Input
                    type="text"
                    value={profileForm.username}
                    onChange={(e) => setProfileForm({ ...profileForm, username: e.target.value })}
                    placeholder={t('profile.username')}
                    disabled={profileLoading}
                  />
                </FormControl>
              </FormField>

              <FormField name="email">
                <FormLabel required>{t('profile.email')}</FormLabel>
                <FormControl>
                  <Input
                    type="email"
                    value={profileForm.email}
                    onChange={(e) => setProfileForm({ ...profileForm, email: e.target.value })}
                    placeholder={t('profile.email')}
                    disabled={profileLoading}
                  />
                </FormControl>
              </FormField>

              <FormMessage error={profileError || undefined} success={profileSuccess || undefined} />

              <div className="flex items-center gap-3 pt-2">
                <Button type="submit" disabled={profileLoading}>
                  {profileLoading ? t('common.loading') : t('profile.actions.updateProfile')}
                </Button>
              </div>
            </Form>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>{t('profile.changePassword.title')}</CardTitle>
            <CardDescription>{t('profile.changePassword.description')}</CardDescription>
          </CardHeader>
          <CardContent>
            <Form onSubmit={handlePasswordChange} errors={{}}>
              <FormField name="currentPassword">
                <FormLabel required>{t('profile.changePassword.currentPassword')}</FormLabel>
                <FormControl>
                  <Input
                    type="password"
                    value={passwordForm.currentPassword}
                    onChange={(e) => setPasswordForm({ ...passwordForm, currentPassword: e.target.value })}
                    placeholder={t('profile.changePassword.currentPasswordPlaceholder')}
                    disabled={passwordLoading}
                  />
                </FormControl>
              </FormField>

              <FormField name="newPassword">
                <FormLabel required>{t('profile.changePassword.newPassword')}</FormLabel>
                <FormControl>
                  <Input
                    type="password"
                    value={passwordForm.newPassword}
                    onChange={(e) => setPasswordForm({ ...passwordForm, newPassword: e.target.value })}
                    placeholder={t('profile.changePassword.newPasswordPlaceholder')}
                    disabled={passwordLoading}
                  />
                </FormControl>
              </FormField>

              <FormField name="confirmPassword">
                <FormLabel required>{t('profile.changePassword.confirmPassword')}</FormLabel>
                <FormControl>
                  <Input
                    type="password"
                    value={passwordForm.confirmPassword}
                    onChange={(e) => setPasswordForm({ ...passwordForm, confirmPassword: e.target.value })}
                    placeholder={t('profile.changePassword.confirmPasswordPlaceholder')}
                    disabled={passwordLoading}
                  />
                </FormControl>
              </FormField>

              <FormMessage error={passwordError || undefined} success={passwordSuccess || undefined} />

              <div className="flex items-center gap-3 pt-2">
                <Button 
                  type="submit" 
                  variant="destructive"
                  disabled={passwordLoading}
                >
                  {passwordLoading ? t('common.loading') : t('profile.actions.changePassword')}
                </Button>
              </div>
            </Form>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
