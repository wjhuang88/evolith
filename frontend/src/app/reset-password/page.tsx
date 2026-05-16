'use client';

import { useState, Suspense } from 'react';
import { Link, useRouter, useSearchParams } from '@/lib/router';
import { useTranslation } from 'react-i18next';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';

function ResetPasswordForm() {
  const { t } = useTranslation();
  const router = useRouter();
  const searchParams = useSearchParams();
  const token = searchParams.get('token');
  
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');

    if (password !== confirmPassword) {
      setError(t('auth.resetPasswordPage.passwordsDoNotMatch'));
      return;
    }

    if (password.length < 8) {
      setError(t('auth.resetPasswordPage.passwordTooShort'));
      return;
    }

    setIsLoading(true);

    try {
      // TODO: Call API to reset password
      // await api.post('/auth/reset-password', { token, password });
      setSuccess(true);
    } catch {
      setError(t('auth.resetPasswordPage.linkExpired'));
    } finally {
      setIsLoading(false);
    }
  };

  if (!token) {
    return (
      <div className="rounded-lg border border-border bg-card p-6 shadow-sm text-center">
        <div className="mb-4 flex justify-center">
          <div className="flex h-12 w-12 items-center justify-center rounded-full bg-error/10">
            <svg className="h-6 w-6 text-error" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </div>
        </div>
        <h2 className="text-xl font-bold text-foreground">{t('auth.resetPasswordPage.invalidLink')}</h2>
        <p className="mt-2 text-sm text-muted-foreground">
          {t('auth.resetPasswordPage.invalidLinkDesc')}
        </p>
        <div className="mt-6">
          <Link href="/forgot-password">
            <Button variant="outline" className="w-full">
              {t('auth.resetPasswordPage.requestNewLink')}
            </Button>
          </Link>
        </div>
      </div>
    );
  }

  if (success) {
    return (
      <div className="rounded-lg border border-border bg-card p-6 shadow-sm text-center">
        <div className="mb-4 flex justify-center">
          <div className="flex h-12 w-12 items-center justify-center rounded-full bg-green-100 dark:bg-green-900">
            <svg className="h-6 w-6 text-green-600 dark:text-green-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
            </svg>
          </div>
        </div>
        <h2 className="text-xl font-bold text-foreground">{t('auth.resetPasswordPage.resetComplete')}</h2>
        <p className="mt-2 text-sm text-muted-foreground">
          {t('auth.resetPasswordPage.resetCompleteDesc')}
        </p>
        <div className="mt-6">
          <Link href="/login">
            <Button className="w-full">
              {t('auth.resetPasswordPage.signIn')}
            </Button>
          </Link>
        </div>
      </div>
    );
  }

  return (
    <div className="rounded-lg border border-border bg-card p-6 shadow-sm">
      <form onSubmit={handleSubmit} className="space-y-4">
        {error && (
          <div className="rounded-md bg-error/10 p-3 text-sm text-error">
            {error}
          </div>
        )}

        <div className="space-y-2">
          <label htmlFor="password" className="text-sm font-medium">
            {t('auth.resetPasswordPage.newPassword')}
          </label>
          <Input
            id="password"
            type="password"
            placeholder="••••••••"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            required
            minLength={8}
          />
        </div>

        <div className="space-y-2">
          <label htmlFor="confirmPassword" className="text-sm font-medium">
            {t('auth.resetPasswordPage.confirmPassword')}
          </label>
          <Input
            id="confirmPassword"
            type="password"
            placeholder="••••••••"
            value={confirmPassword}
            onChange={(e) => setConfirmPassword(e.target.value)}
            required
            minLength={8}
          />
        </div>

        <Button
          type="submit"
          className="w-full"
          disabled={isLoading}
        >
          {isLoading ? t('auth.resetPasswordPage.resetting') : t('auth.resetPasswordPage.resetPassword')}
        </Button>
      </form>

      <div className="mt-4 text-center text-sm text-muted-foreground">
        <Link href="/login" className="text-primary hover:underline">
          {t('auth.resetPasswordPage.backToSignIn')}
        </Link>
      </div>
    </div>
  );
}

export default function ResetPasswordPage() {
  const { t } = useTranslation();
  
  return (
    <div className="flex min-h-screen items-center justify-center bg-background px-4">
      <div className="w-full max-w-md">
        <div className="mb-8 text-center">
          <Link href="/" className="inline-flex items-center gap-2">
            <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-gradient-to-br from-primary-500 to-purple-600">
              <span className="text-white font-bold">E</span>
            </div>
          </Link>
          <h1 className="mt-6 text-2xl font-bold text-foreground">{t('auth.resetPasswordPage.title')}</h1>
          <p className="mt-2 text-sm text-muted-foreground">
            {t('auth.resetPasswordPage.subtitle')}
          </p>
        </div>

        <Suspense fallback={
          <div className="rounded-lg border border-border bg-card p-6 shadow-sm text-center">
            <p className="text-muted-foreground">{t('common.loading')}</p>
          </div>
        }>
          <ResetPasswordForm />
        </Suspense>
      </div>
    </div>
  );
}
