'use client';

import { useState } from 'react';
import Link from 'next/link';
import { useTranslation, Trans } from 'react-i18next';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';

export default function RegisterPage() {
  const { t } = useTranslation();
  const [formData, setFormData] = useState({
    email: '',
    username: '',
    password: '',
    confirmPassword: '',
    tenantName: '',
    tenantSlug: '',
  });
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState('');
  const [validationError, setValidationError] = useState('');
  const [isSuccess, setIsSuccess] = useState(false);
  const [verificationLink, setVerificationLink] = useState('');

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({ ...prev, [name]: value }));
    setError('');
    setValidationError('');
  };

  const validateForm = () => {
    if (!formData.email || !formData.username || !formData.password) {
      return t('auth.registerPage.validation.fillRequired');
    }
    if (formData.password !== formData.confirmPassword) {
      return t('auth.registerPage.validation.passwordsDoNotMatch');
    }
    if (formData.password.length < 8) {
      return t('auth.registerPage.validation.passwordTooShort');
    }
    if (formData.tenantSlug && !/^[a-z0-9-]+$/.test(formData.tenantSlug)) {
      return t('auth.registerPage.validation.invalidSlug');
    }
    return '';
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    const validation = validateForm();
    if (validation) {
      setValidationError(validation);
      return;
    }

    setIsLoading(true);
    setError('');

    try {
      const response = await fetch('/api/v1/auth/register', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          email: formData.email,
          username: formData.username,
          password: formData.password,
          tenant_name: formData.tenantName || undefined,
          tenant_slug: formData.tenantSlug || undefined,
        }),
      });

      const data = await response.json();

      if (data.success) {
        setIsSuccess(true);
        // In development, the API returns the verification link
        if (data.verification_link) {
          setVerificationLink(data.verification_link);
        }
      } else {
        setError(data.error?.message || t('auth.registerPage.registrationFailed'));
      }
    } catch {
      setError(t('auth.registerPage.errorOccurred'));
    } finally {
      setIsLoading(false);
    }
  };

  if (isSuccess) {
    return (
      <div className="flex min-h-screen items-center justify-center bg-background px-4 py-12">
        <div className="w-full max-w-md">
          <div className="mb-8 text-center">
            <Link href="/" className="inline-flex items-center gap-2">
              <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-gradient-to-br from-primary-500 to-purple-600">
                <span className="text-white font-bold">E</span>
              </div>
            </Link>
            <h1 className="mt-6 text-2xl font-bold text-foreground">{t('auth.registerPage.checkEmail')}</h1>
          </div>

          <div className="rounded-lg border border-border bg-card p-6 shadow-sm">
            <div className="text-center">
              <div className="mb-4 rounded-full bg-green-100 p-3 dark:bg-green-900/20">
                <svg
                  className="mx-auto h-8 w-8 text-green-600 dark:text-green-400"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
                  />
                </svg>
              </div>

              <p className="mb-4 text-muted-foreground">
                <Trans i18nKey="auth.registerPage.verificationSent" values={{ email: formData.email }} components={{ strong: <strong /> }} />
              </p>

              {verificationLink && (
                <div className="mb-4 rounded-md bg-muted p-3">
                  <p className="mb-2 text-sm text-muted-foreground">{t('auth.registerPage.devVerificationLink')}</p>
                  <Link
                    href={verificationLink}
                    className="break-all text-sm text-primary hover:underline"
                  >
                    {verificationLink}
                  </Link>
                </div>
              )}

              <div className="mt-6 space-y-3">
                <Link href="/login">
                  <Button className="w-full">{t('auth.registerPage.goToLogin')}</Button>
                </Link>
                <p className="text-sm text-muted-foreground">
                  {t('auth.registerPage.didntReceive')}{' '}
                  <button
                    onClick={() => setIsSuccess(false)}
                    className="text-primary hover:underline"
                  >
                    {t('auth.registerPage.tryAgain')}
                  </button>
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="flex min-h-screen items-center justify-center bg-background px-4 py-12">
      <div className="w-full max-w-md">
        <div className="mb-8 text-center">
          <Link href="/" className="inline-flex items-center gap-2">
            <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-gradient-to-br from-primary-500 to-purple-600">
              <span className="text-white font-bold">E</span>
            </div>
          </Link>
          <h1 className="mt-6 text-2xl font-bold text-foreground">{t('auth.registerPage.title')}</h1>
          <p className="mt-2 text-sm text-muted-foreground">
            {t('auth.registerPage.subtitle')}
          </p>
        </div>

        <div className="rounded-lg border border-border bg-card p-6 shadow-sm">
          <form onSubmit={handleSubmit} className="space-y-4">
            {(error || validationError) && (
              <div className="rounded-md bg-destructive/10 p-3 text-sm text-destructive">
                {error || validationError}
              </div>
            )}

            <div>
              <label htmlFor="email" className="block text-sm font-medium text-foreground">
                {t('auth.email')} *
              </label>
              <Input
                id="email"
                name="email"
                type="email"
                value={formData.email}
                onChange={handleChange}
                placeholder="you@company.com"
                required
                className="mt-1"
              />
            </div>

            <div>
              <label htmlFor="username" className="block text-sm font-medium text-foreground">
                {t('auth.username')} *
              </label>
              <Input
                id="username"
                name="username"
                type="text"
                value={formData.username}
                onChange={handleChange}
                placeholder="john"
                required
                className="mt-1"
              />
            </div>

            <div>
              <label htmlFor="password" className="block text-sm font-medium text-foreground">
                {t('auth.password')} *
              </label>
              <Input
                id="password"
                name="password"
                type="password"
                value={formData.password}
                onChange={handleChange}
                placeholder="••••••••"
                required
                className="mt-1"
              />
              <p className="mt-1 text-xs text-muted-foreground">
                {t('auth.registerPage.passwordHint')}
              </p>
            </div>

            <div>
              <label htmlFor="confirmPassword" className="block text-sm font-medium text-foreground">
                {t('auth.registerPage.confirmPassword')} *
              </label>
              <Input
                id="confirmPassword"
                name="confirmPassword"
                type="password"
                value={formData.confirmPassword}
                onChange={handleChange}
                placeholder="••••••••"
                required
                className="mt-1"
              />
            </div>

            <hr className="my-4 border-border" />

            <div>
              <label htmlFor="tenantName" className="block text-sm font-medium text-foreground">
                {t('auth.registerPage.orgName')}
              </label>
              <Input
                id="tenantName"
                name="tenantName"
                type="text"
                value={formData.tenantName}
                onChange={handleChange}
                placeholder={t('auth.registerPage.orgNamePlaceholder')}
                className="mt-1"
              />
              <p className="mt-1 text-xs text-muted-foreground">
                {t('auth.registerPage.orgNameHint')}
              </p>
            </div>

            <div>
              <label htmlFor="tenantSlug" className="block text-sm font-medium text-foreground">
                {t('auth.registerPage.orgUrl')}
              </label>
              <div className="mt-1 flex rounded-md">
                <span className="inline-flex items-center rounded-l-md border border-r-0 border-input bg-muted px-3 text-sm text-muted-foreground">
                  evolith.io/
                </span>
                <Input
                  id="tenantSlug"
                  name="tenantSlug"
                  type="text"
                  value={formData.tenantSlug}
                  onChange={handleChange}
                  placeholder="my-company"
                  className="rounded-l-none"
                />
              </div>
            </div>

            <Button type="submit" className="w-full" disabled={isLoading}>
              {isLoading ? t('auth.registerPage.creatingAccount') : t('auth.registerPage.createAccount')}
            </Button>
          </form>

          <p className="mt-4 text-center text-sm text-muted-foreground">
            {t('auth.registerPage.alreadyHaveAccount')}{' '}
            <Link href="/login" className="font-medium text-primary hover:underline">
              {t('auth.registerPage.signIn')}
            </Link>
          </p>
        </div>
      </div>
    </div>
  );
}
