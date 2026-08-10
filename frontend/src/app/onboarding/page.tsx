'use client';

import { useCallback, useEffect, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Button, Card, CardContent, CardDescription, CardHeader, CardTitle, Input } from '@/components/ui';
import { reposApi } from '@/lib/api/repos';
import { parseApiError } from '@/lib/api';
import type { CreateRepoRequest } from '@/lib/api/types';
import { useRouter } from '@/lib/router';
import { useAuthStore } from '@/stores/authStore';

type RepoCheckState = 'loading' | 'empty' | 'error' | 'forbidden';

const REPO_NAME_PATTERN = /^[a-zA-Z0-9._-]+$/;

function isForbidden(error: unknown): boolean {
  const status = (error as { response?: { status?: number } }).response?.status;
  const code = parseApiError(error).code.toUpperCase();
  return status === 403 || code === 'FORBIDDEN' || code === 'HTTP_403';
}

export default function OnboardingPage() {
  const { t } = useTranslation();
  const router = useRouter();
  const routerRef = useRef(router);
  routerRef.current = router;
  const { tenant } = useAuthStore();
  const [repoCheck, setRepoCheck] = useState<RepoCheckState>('loading');
  const [repoCheckError, setRepoCheckError] = useState<string | null>(null);
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [visibility, setVisibility] = useState<'public' | 'private'>('private');
  const [defaultBranch, setDefaultBranch] = useState('main');
  const [autoMerge, setAutoMerge] = useState(false);
  const [requireReview, setRequireReview] = useState(true);
  const [nameError, setNameError] = useState<string | null>(null);
  const [submitError, setSubmitError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);

  const checkRepos = useCallback(async (isCurrent: () => boolean = () => true) => {
    setRepoCheck('loading');
    setRepoCheckError(null);

    if (!tenant?.id) {
      if (isCurrent()) {
        setRepoCheck('error');
        setRepoCheckError(t('onboarding.errors.missingTenant'));
      }
      return;
    }

    try {
      const repos = await reposApi.list(tenant.id);
      if (!isCurrent()) return;
      if (repos.length > 0) {
        routerRef.current.replace('/dashboard');
        return;
      }
      setRepoCheck('empty');
    } catch (error) {
      if (!isCurrent()) return;
      setRepoCheck(isForbidden(error) ? 'forbidden' : 'error');
      setRepoCheckError(parseApiError(error).message);
    }
  }, [t, tenant?.id]);

  useEffect(() => {
    let current = true;
    void checkRepos(() => current);
    return () => {
      current = false;
    };
  }, [checkRepos]);

  const validateName = () => {
    const normalized = name.trim();
    if (!normalized) {
      return t('onboarding.form.validation.nameRequired');
    }
    if (!REPO_NAME_PATTERN.test(normalized) || normalized.length > 128) {
      return t('onboarding.form.validation.nameInvalid');
    }
    return null;
  };

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const nextNameError = validateName();
    setNameError(nextNameError);
    setSubmitError(null);
    if (nextNameError) return;
    if (!tenant?.id) {
      setSubmitError(t('onboarding.errors.missingTenant'));
      return;
    }

    setSubmitting(true);
    try {
      const payload: CreateRepoRequest = {
        name: name.trim(),
        description: description.trim() || undefined,
        visibility,
        default_branch: defaultBranch.trim() || 'main',
        auto_merge: autoMerge,
        require_review: requireReview,
        seed_template: true,
      };
      const repo = await reposApi.create(tenant.id, payload);
      routerRef.current.replace(`/repos/${repo.id}/overview`);
    } catch (error) {
      setSubmitError(parseApiError(error).message || t('onboarding.errors.createFailed'));
    } finally {
      setSubmitting(false);
    }
  };

  if (repoCheck === 'loading') {
    return (
      <main className="flex min-h-screen items-center justify-center bg-background px-4" aria-busy="true">
        <div className="text-center">
          <div className="mx-auto h-10 w-10 animate-spin rounded-full border-4 border-primary border-t-transparent" />
          <p className="mt-4 text-sm text-muted-foreground">{t('onboarding.checking')}</p>
        </div>
      </main>
    );
  }

  if (repoCheck === 'error' || repoCheck === 'forbidden') {
    const forbidden = repoCheck === 'forbidden';
    return (
      <main className="flex min-h-screen items-center justify-center bg-background px-4 py-10">
        <Card className="w-full max-w-lg">
          <CardHeader>
            <CardTitle>{t(forbidden ? 'onboarding.errors.forbiddenTitle' : 'onboarding.errors.loadTitle')}</CardTitle>
            <CardDescription>
              {t(forbidden ? 'onboarding.errors.forbiddenDescription' : 'onboarding.errors.loadDescription')}
            </CardDescription>
          </CardHeader>
          <CardContent>
            {repoCheckError && (
              <p className="rounded-[8px] border border-destructive bg-destructive/10 px-3 py-2 text-sm text-destructive" role="alert">
                {repoCheckError}
              </p>
            )}
            <Button className="mt-5" onClick={() => void checkRepos()}>
              {t('common.retry')}
            </Button>
          </CardContent>
        </Card>
      </main>
    );
  }

  return (
    <main className="min-h-screen bg-background px-4 py-8 sm:py-12">
      <div className="mx-auto w-full max-w-3xl">
        <header className="mb-8 text-center">
          <div className="mx-auto flex h-12 w-12 items-center justify-center rounded-[12px] bg-primary text-lg font-bold text-primary-foreground">
            E
          </div>
          <p className="mt-5 text-sm font-medium text-primary">{t('onboarding.eyebrow')}</p>
          <h1 className="mt-2 text-3xl font-bold text-foreground sm:text-4xl">
            {t('onboarding.title')}
          </h1>
          <p className="mx-auto mt-3 max-w-xl text-sm text-muted-foreground sm:text-base">
            {t('onboarding.subtitle')}
          </p>
        </header>

        <form onSubmit={handleSubmit} noValidate>
          <Card>
            <CardHeader>
              <CardTitle>{t('onboarding.form.title')}</CardTitle>
              <CardDescription>{t('onboarding.form.description')}</CardDescription>
            </CardHeader>
            <CardContent className="space-y-5">
              <div>
                <label htmlFor="onboarding-repo-name" className="text-sm font-medium text-foreground">
                  {t('onboarding.form.nameLabel')}
                </label>
                <Input
                  id="onboarding-repo-name"
                  name="name"
                  value={name}
                  onChange={(event) => {
                    setName(event.target.value);
                    setNameError(null);
                  }}
                  placeholder={t('onboarding.form.namePlaceholder')}
                  aria-invalid={Boolean(nameError)}
                  aria-describedby={nameError ? 'onboarding-name-error' : 'onboarding-name-hint'}
                  autoComplete="off"
                  autoFocus
                  required
                  className="mt-2"
                />
                <p id="onboarding-name-hint" className="mt-1 text-xs text-muted-foreground">
                  {t('onboarding.form.nameHint')}
                </p>
                {nameError && (
                  <p id="onboarding-name-error" className="mt-1 text-sm text-destructive" role="alert">
                    {nameError}
                  </p>
                )}
              </div>

              <div>
                <label htmlFor="onboarding-repo-description" className="text-sm font-medium text-foreground">
                  {t('onboarding.form.descriptionLabel')}
                </label>
                <Input
                  id="onboarding-repo-description"
                  name="description"
                  value={description}
                  onChange={(event) => setDescription(event.target.value)}
                  placeholder={t('onboarding.form.descriptionPlaceholder')}
                  autoComplete="off"
                  className="mt-2"
                />
              </div>

              <fieldset>
                <legend className="text-sm font-medium text-foreground">{t('onboarding.form.visibilityLabel')}</legend>
                <div className="mt-2 grid gap-3 sm:grid-cols-2">
                  {(['private', 'public'] as const).map((option) => (
                    <label key={option} className="flex cursor-pointer items-start gap-3 rounded-[8px] border border-border p-3 hover:bg-muted/40">
                      <input
                        type="radio"
                        name="visibility"
                        value={option}
                        checked={visibility === option}
                        onChange={() => setVisibility(option)}
                        className="mt-1"
                      />
                      <span>
                        <span className="block text-sm font-medium text-foreground">
                          {t(`onboarding.form.visibility.${option}.title`)}
                        </span>
                        <span className="mt-1 block text-xs text-muted-foreground">
                          {t(`onboarding.form.visibility.${option}.description`)}
                        </span>
                      </span>
                    </label>
                  ))}
                </div>
              </fieldset>

              <div>
                <label htmlFor="onboarding-default-branch" className="text-sm font-medium text-foreground">
                  {t('onboarding.form.defaultBranchLabel')}
                </label>
                <Input
                  id="onboarding-default-branch"
                  name="default_branch"
                  value={defaultBranch}
                  onChange={(event) => setDefaultBranch(event.target.value)}
                  autoComplete="off"
                  className="mt-2"
                />
              </div>

              <div className="grid gap-3 sm:grid-cols-2">
                <Toggle
                  id="onboarding-auto-merge"
                  label={t('onboarding.form.autoMerge')}
                  checked={autoMerge}
                  onChange={setAutoMerge}
                />
                <Toggle
                  id="onboarding-require-review"
                  label={t('onboarding.form.requireReview')}
                  checked={requireReview}
                  onChange={setRequireReview}
                />
              </div>

              {submitError && (
                <p id="onboarding-submit-error" className="rounded-[8px] border border-destructive bg-destructive/10 px-3 py-2 text-sm text-destructive" role="alert">
                  {submitError}
                </p>
              )}

              <div className="flex flex-col-reverse gap-3 border-t border-border pt-5 sm:flex-row sm:items-center sm:justify-between">
                <p className="text-xs text-muted-foreground">{t('onboarding.form.nextStep')}</p>
                <Button type="submit" disabled={submitting} aria-describedby={submitError ? 'onboarding-submit-error' : undefined}>
                  {submitting ? t('onboarding.form.creating') : t('onboarding.form.create')}
                </Button>
              </div>
            </CardContent>
          </Card>
        </form>
      </div>
    </main>
  );
}

function Toggle({
  id,
  label,
  checked,
  onChange,
}: {
  id: string;
  label: string;
  checked: boolean;
  onChange: (checked: boolean) => void;
}) {
  return (
    <label htmlFor={id} className="flex cursor-pointer items-center justify-between gap-4 rounded-[8px] border border-border p-3">
      <span className="text-sm font-medium text-foreground">{label}</span>
      <input
        id={id}
        type="checkbox"
        checked={checked}
        onChange={(event) => onChange(event.target.checked)}
        className="h-5 w-5 accent-[var(--primary)]"
      />
    </label>
  );
}
