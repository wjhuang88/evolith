'use client';

import { useState } from 'react';
import { Link, useRouter } from '@/lib/router';
import { useTranslation, Trans } from 'react-i18next';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { apiClient } from '@/lib/api/client';

export default function ForgotPasswordPage() {
  const { t } = useTranslation();
  const router = useRouter();
  const [email, setEmail] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [submitted, setSubmitted] = useState(false);
  const [error, setError] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);
    setError('');

    try {
      await apiClient.post('/auth/forgot-password', { email });
      setSubmitted(true);
    } catch (err) {
      setError(t('auth.forgotPasswordPage.sendFailed'));
    } finally {
      setIsLoading(false);
    }
  };

  if (submitted) {
    return (
      <div className="flex min-h-screen items-center justify-center bg-background px-4">
        <div className="w-full max-w-md">
          <div className="rounded-lg border border-border bg-card p-6 shadow-sm text-center">
            <div className="mb-4 flex justify-center">
              <div className="flex h-12 w-12 items-center justify-center rounded-full bg-green-100 dark:bg-green-900">
                <svg className="h-6 w-6 text-green-600 dark:text-green-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                </svg>
              </div>
            </div>
            <h2 className="text-xl font-bold text-foreground">{t('auth.forgotPasswordPage.checkEmail')}</h2>
            <p className="mt-2 text-sm text-muted-foreground">
              <Trans i18nKey="auth.forgotPasswordPage.sentInstructions" values={{ email }} components={{ strong: <strong /> }} />
            </p>
            <div className="mt-6">
              <Link to="/login"><Button variant="outline" className="w-full">
                {t('auth.forgotPasswordPage.backToSignIn')}
              </Button></Link>
            </div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="flex min-h-screen items-center justify-center bg-background px-4">
      <div className="w-full max-w-md">
        <div className="mb-8 text-center">
          <Link to="/"className="inline-flex items-center gap-2"><div className="flex h-10 w-10 items-center justify-center rounded-lg bg-gradient-to-br from-primary-500 to-purple-600">
            <span className="text-white font-bold">E</span>
          </div></Link>
          <h1 className="mt-6 text-2xl font-bold text-foreground">{t('auth.forgotPasswordPage.title')}</h1>
          <p className="mt-2 text-sm text-muted-foreground">
            {t('auth.forgotPasswordPage.subtitle')}
          </p>
        </div>

        <div className="rounded-lg border border-border bg-card p-6 shadow-sm">
          <form onSubmit={handleSubmit} className="space-y-4">
            {error && (
              <div className="rounded-md bg-error/10 p-3 text-sm text-error">
                {error}
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

            <Button
              type="submit"
              className="w-full"
              disabled={isLoading}
            >
              {isLoading ? t('auth.forgotPasswordPage.sending') : t('auth.forgotPasswordPage.sendResetLink')}
            </Button>
          </form>

          <div className="mt-4 text-center text-sm text-muted-foreground">
            {t('auth.forgotPasswordPage.rememberPassword')}{' '}
            <Link to="/login"className="text-primary hover:underline">{t('auth.forgotPasswordPage.signIn')}</Link>
          </div>
        </div>
      </div>
    </div>
  );
}
