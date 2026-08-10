'use client';

import { useState } from 'react';
import { Link, useRouter, useSearchParams } from '@/lib/router';
import { useTranslation } from 'react-i18next';
import { useAuthStore } from '@/stores';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { resolveAuthenticatedEntry } from '@/lib/entry';
import { hrefWithRedirect } from '@/lib/entry-policy';

export default function LoginPage() {
  const { t } = useTranslation();
  const router = useRouter();
  const [searchParams] = useSearchParams();
  const { login, isLoading, error, clearError } = useAuthStore();
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [isResolvingEntry, setIsResolvingEntry] = useState(false);
  const [entryError, setEntryError] = useState(false);
  const redirect = searchParams.get('redirect');

  const resolveEntry = async () => {
    const tenantId = useAuthStore.getState().tenant?.id;
    if (!tenantId) {
      setEntryError(true);
      return;
    }

    setIsResolvingEntry(true);
    setEntryError(false);
    try {
      router.replace(await resolveAuthenticatedEntry(tenantId, redirect));
    } catch {
      setEntryError(true);
    } finally {
      setIsResolvingEntry(false);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    clearError();
    
    try {
      await login(email, password);
      if (!useAuthStore.getState().isAuthenticated) return;
      await resolveEntry();
    } catch {
      // Error is handled in store
    }
  };

  return (
    <div className="flex min-h-screen items-center justify-center bg-background px-4">
      <div className="w-full max-w-md">
        <div className="mb-8 text-center">
          <Link to="/"className="inline-flex items-center gap-2"><div className="flex h-10 w-10 items-center justify-center rounded-lg bg-[var(--primary)]">
            <span className="text-white font-bold">E</span>
          </div></Link>
          <h1 className="mt-6 text-2xl font-bold text-foreground">{t('auth.loginPage.welcome')}</h1>
          <p className="mt-2 text-sm text-muted-foreground">
            {t('auth.loginPage.subtitle')}
          </p>
        </div>

        <div className="rounded-lg border border-border bg-card p-6 shadow-sm">
          <form onSubmit={handleSubmit} className="space-y-4">
            {error && (
              <div className="rounded-md bg-error/10 p-3 text-sm text-error">
                {error}
              </div>
            )}
            {entryError && (
              <div className="space-y-3 rounded-md bg-destructive/10 p-3 text-sm text-destructive">
                <p>{t('auth.entry.loadDescription')}</p>
                <Button type="button" variant="outline" size="sm" onClick={resolveEntry}>
                  {t('common.retry')}
                </Button>
              </div>
            )}

            <div className="space-y-2">
              <label htmlFor="email" className="text-sm font-medium">
                {t('auth.email')}
              </label>
              <Input
                id="email"
                type="email"
                placeholder="you@example.com"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                required
              />
            </div>

            <div className="space-y-2">
              <label htmlFor="password" className="text-sm font-medium">
                {t('auth.password')}
              </label>
              <Input
                id="password"
                type="password"
                placeholder="••••••••"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                required
              />
            </div>

            <div className="text-right text-sm">
              <Link to={hrefWithRedirect('/forgot-password', redirect)} className="text-primary hover:underline">{t('auth.loginPage.forgotPassword')}</Link>
            </div>

            <Button
              type="submit"
              className="w-full"
              disabled={isLoading || isResolvingEntry}
            >
              {isLoading || isResolvingEntry ? t('auth.loginPage.signingIn') : t('auth.loginPage.signIn')}
            </Button>
          </form>

          <div className="mt-4 text-center text-sm text-muted-foreground">
            {t('auth.loginPage.noAccount')}{' '}
            <Link to={hrefWithRedirect('/register', redirect)} className="text-primary hover:underline">{t('auth.loginPage.signUp')}</Link>
          </div>
        </div>
      </div>
    </div>
  );
}
