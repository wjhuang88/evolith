'use client';

import { useState } from 'react';
import { Link, useRouter, useSearchParams } from '@/lib/router';
import { useTranslation } from 'react-i18next';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { authApi } from '@/lib/api';
import { useAuthStore } from '@/stores/authStore';
import { resolveAuthenticatedEntry } from '@/lib/entry';

export default function JoinPage() {
  const { t } = useTranslation();
  const router = useRouter();
  const [searchParams] = useSearchParams();
  const token = searchParams.get('token') || '';
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState('');
  const [invitationAccepted, setInvitationAccepted] = useState(false);
  const [entryError, setEntryError] = useState(false);
  const redirect = searchParams.get('redirect');

  const resolveEntry = async () => {
    const tenantId = useAuthStore.getState().tenant?.id;
    if (!tenantId) {
      setEntryError(true);
      return;
    }
    setIsLoading(true);
    setEntryError(false);
    try {
      router.replace(await resolveAuthenticatedEntry(tenantId, redirect));
    } catch {
      setEntryError(true);
    } finally {
      setIsLoading(false);
    }
  };

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    setError('');

    if (!token) {
      setError(t('auth.joinPage.invalidLink'));
      return;
    }

    if (password !== confirmPassword) {
      setError(t('auth.joinPage.passwordsNoMatch'));
      return;
    }

    if (password.length < 8) {
      setError(t('auth.joinPage.passwordTooShort'));
      return;
    }

    setIsLoading(true);
    try {
      const response = await authApi.acceptInvitation({ token, username, password });
      if (response.success && response.data) {
        useAuthStore.setState({
          user: response.data.user,
          tenant: response.data.tenant || null,
          isAuthenticated: true,
          isLoading: false,
          error: null,
        });
        setInvitationAccepted(true);
        await resolveEntry();
        return;
      }
      setError(response.error?.message || t('auth.joinPage.acceptFailed'));
    } catch {
      setError(t('auth.joinPage.acceptFailed'));
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="flex min-h-screen items-center justify-center bg-background px-4 py-12">
      <div className="w-full max-w-md">
        <div className="mb-8 text-center">
          <Link to="/" className="inline-flex items-center gap-2">
            <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-[var(--primary)]">
              <span className="font-bold text-white">E</span>
            </div>
          </Link>
          <h1 className="mt-6 text-2xl font-bold text-foreground">{t('auth.joinPage.title')}</h1>
          <p className="mt-2 text-sm text-muted-foreground">
            {t('auth.joinPage.subtitle')}
          </p>
        </div>

        <div className="rounded-lg border border-border bg-card p-6 shadow-sm">
          {!token ? (
            <div className="text-center">
              <p className="mb-6 text-sm text-destructive">
                {t('auth.joinPage.invalidLink')}
              </p>
              <Link to="/login">
                <Button variant="outline" className="w-full">
                  {t('auth.joinPage.backToSignIn')}
                </Button>
              </Link>
            </div>
          ) : invitationAccepted ? (
            <div className="space-y-4 text-center">
              <h2 className="text-lg font-semibold">{t('auth.joinPage.accepted')}</h2>
              <p className="text-sm text-muted-foreground">{t('auth.entry.loadDescription')}</p>
              {entryError && (
                <Button type="button" variant="outline" className="w-full" onClick={resolveEntry} disabled={isLoading}>
                  {isLoading ? t('common.loading') : t('common.retry')}
                </Button>
              )}
            </div>
          ) : (
            <form onSubmit={handleSubmit} className="space-y-4">
              {error && (
                <div className="rounded-md bg-destructive/10 p-3 text-sm text-destructive">
                  {error}
                </div>
              )}

              <div className="space-y-2">
                <label htmlFor="username" className="text-sm font-medium">
                  {t('auth.username')}
                </label>
                <Input
                  id="username"
                  type="text"
                  value={username}
                  onChange={(event) => setUsername(event.target.value)}
                  placeholder={t('auth.joinPage.usernamePlaceholder')}
                  required
                  minLength={3}
                />
              </div>

              <div className="space-y-2">
                <label htmlFor="password" className="text-sm font-medium">
                  {t('auth.password')}
                </label>
                <Input
                  id="password"
                  type="password"
                  value={password}
                  onChange={(event) => setPassword(event.target.value)}
                  placeholder={t('auth.joinPage.passwordPlaceholder')}
                  required
                  minLength={8}
                />
              </div>

              <div className="space-y-2">
                <label htmlFor="confirmPassword" className="text-sm font-medium">
                  {t('auth.joinPage.confirmPassword')}
                </label>
                <Input
                  id="confirmPassword"
                  type="password"
                  value={confirmPassword}
                  onChange={(event) => setConfirmPassword(event.target.value)}
                  placeholder={t('auth.joinPage.confirmPasswordPlaceholder')}
                  required
                  minLength={8}
                />
              </div>

              <Button type="submit" className="w-full" disabled={isLoading}>
                {isLoading ? t('auth.joinPage.joining') : t('auth.joinPage.acceptInvitation')}
              </Button>
            </form>
          )}
        </div>
      </div>
    </div>
  );
}
