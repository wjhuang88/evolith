'use client';

import { useState } from 'react';
import { useRouter } from '@/lib/router';
import { useTranslation } from 'react-i18next';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui';
import { Button } from '@/components/ui';
import { Input } from '@/components/ui';
import { Form, FormField, FormLabel, FormControl, FormGroup, FormActions } from '@/components/ui';
import { useAuthStore } from '@/stores';
import { reposApi } from '@/lib/api/repos';
import type { CreateRepoRequest } from '@/lib/api/types';

const REPO_NAME_PATTERN = /^[a-zA-Z0-9._-]+$/;

export default function NewRepoPage() {
  const { t } = useTranslation();
  const router = useRouter();
  const { tenant } = useAuthStore();

  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [visibility, setVisibility] = useState<'public' | 'private'>('private');
  const [defaultBranch, setDefaultBranch] = useState('main');
  const [autoMerge, setAutoMerge] = useState(false);
  const [requireReview, setRequireReview] = useState(true);
  const [errors, setErrors] = useState<Record<string, string>>({});
  const [submitting, setSubmitting] = useState(false);
  const [submitError, setSubmitError] = useState<string | null>(null);

  const validate = (): Record<string, string> => {
    const next: Record<string, string> = {};
    if (!name.trim()) {
      next.name = t('repos.newRepo.validation.nameRequired');
    } else if (!REPO_NAME_PATTERN.test(name.trim()) || name.length > 128) {
      next.name = t('repos.newRepo.validation.nameInvalid');
    }
    if (defaultBranch.trim().length === 0) {
      next.default_branch = t('repos.newRepo.validation.nameRequired');
    }
    return next;
  };

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setSubmitError(null);
    const validation = validate();
    if (Object.keys(validation).length > 0) {
      setErrors(validation);
      return;
    }
    if (!tenant?.id) {
      setSubmitError(t('repos.newRepo.failedToCreate'));
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
      };
      await reposApi.create(tenant.id, payload);
      router.push('/repos');
    } catch (err) {
      const message =
        err instanceof Error ? err.message : t('repos.newRepo.failedToCreate');
      setSubmitError(message);
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <div className="container mx-auto max-w-3xl py-8">
      <div className="mb-6">
        <h1 className="text-3xl font-bold">{t('repos.newRepo.title')}</h1>
        <p className="mt-1 text-muted-foreground">{t('repos.newRepo.subtitle')}</p>
      </div>

      <Form onSubmit={handleSubmit} errors={errors}>
        <Card>
          <CardHeader>
            <CardTitle>{t('repos.newRepo.basicInfo')}</CardTitle>
            <CardDescription>{t('repos.newRepo.basicInfoDesc')}</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <FormGroup>
              <FormField name="name">
                <FormLabel htmlFor="repo-name">{t('repos.newRepo.nameLabel')}</FormLabel>
                <FormControl>
                  <Input
                    id="repo-name"
                    name="name"
                    value={name}
                    onChange={(event) => setName(event.target.value)}
                    placeholder={t('repos.newRepo.namePlaceholder')}
                    autoComplete="off"
                    autoFocus
                    required
                  />
                </FormControl>
                <p className="mt-1 text-xs text-muted-foreground">
                  {t('repos.newRepo.nameHint')}
                </p>
              </FormField>

              <FormField name="description">
                <FormLabel htmlFor="repo-desc">{t('repos.newRepo.descLabel')}</FormLabel>
                <FormControl>
                  <Input
                    id="repo-desc"
                    name="description"
                    value={description}
                    onChange={(event) => setDescription(event.target.value)}
                    placeholder={t('repos.newRepo.descPlaceholder')}
                    autoComplete="off"
                  />
                </FormControl>
              </FormField>
            </FormGroup>

            <FormField name="visibility">
              <FormLabel>{t('repos.newRepo.visibilityLabel')}</FormLabel>
              <FormControl>
                <div className="space-y-2">
                  <label className="flex cursor-pointer items-start gap-3 rounded-[8px] border border-border p-3 hover:bg-muted/40">
                    <input
                      type="radio"
                      name="visibility"
                      value="private"
                      checked={visibility === 'private'}
                      onChange={() => setVisibility('private')}
                      className="mt-1"
                    />
                    <span className="text-sm text-foreground">
                      {t('repos.newRepo.visibilityPrivate')}
                    </span>
                  </label>
                  <label className="flex cursor-pointer items-start gap-3 rounded-[8px] border border-border p-3 hover:bg-muted/40">
                    <input
                      type="radio"
                      name="visibility"
                      value="public"
                      checked={visibility === 'public'}
                      onChange={() => setVisibility('public')}
                      className="mt-1"
                    />
                    <span className="text-sm text-foreground">
                      {t('repos.newRepo.visibilityPublic')}
                    </span>
                  </label>
                </div>
              </FormControl>
            </FormField>

            <FormField name="default_branch">
              <FormLabel htmlFor="repo-branch">
                {t('repos.newRepo.defaultBranchLabel')}
              </FormLabel>
              <FormControl>
                <Input
                  id="repo-branch"
                  name="default_branch"
                  value={defaultBranch}
                  onChange={(event) => setDefaultBranch(event.target.value)}
                  placeholder="main"
                  autoComplete="off"
                />
              </FormControl>
              <p className="mt-1 text-xs text-muted-foreground">
                {t('repos.newRepo.defaultBranchHint')}
              </p>
            </FormField>
          </CardContent>
        </Card>

        <Card className="mt-6">
          <CardHeader>
            <CardTitle>{t('repos.newRepo.policiesTitle')}</CardTitle>
            <CardDescription>{t('repos.newRepo.policiesDesc')}</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <ToggleRow
              id="repo-auto-merge"
              label={t('repos.newRepo.autoMergeLabel')}
              description={t('repos.newRepo.autoMergeDesc')}
              checked={autoMerge}
              onChange={setAutoMerge}
            />
            <ToggleRow
              id="repo-require-review"
              label={t('repos.newRepo.requireReviewLabel')}
              description={t('repos.newRepo.requireReviewDesc')}
              checked={requireReview}
              onChange={setRequireReview}
            />
          </CardContent>
        </Card>

        {submitError && (
          <p className="mt-4 rounded-[8px] border border-destructive bg-destructive/10 px-3 py-2 text-sm text-destructive">
            {submitError}
          </p>
        )}

        <FormActions className="mt-6">
          <Button
            type="button"
            variant="outline"
            onClick={() => router.push('/repos')}
            disabled={submitting}
          >
            {t('common.cancel')}
          </Button>
          <Button type="submit" disabled={submitting}>
            {submitting ? t('repos.newRepo.creating') : t('repos.newRepo.createButton')}
          </Button>
        </FormActions>
      </Form>

      <div className="mt-6 rounded-[24px] border border-border bg-surface-soft p-6">
        <h3 className="text-base font-semibold">{t('repos.newRepo.nextTitle')}</h3>
        <p className="mt-1 text-sm text-muted-foreground">
          {t('repos.newRepo.nextDesc')}
        </p>
      </div>
    </div>
  );
}

function ToggleRow({
  id,
  label,
  description,
  checked,
  onChange,
}: {
  id: string;
  label: string;
  description: string;
  checked: boolean;
  onChange: (next: boolean) => void;
}) {
  return (
    <label className="flex items-start justify-between gap-4 rounded-[8px] border border-border p-3">
      <span>
        <span className="text-sm font-medium text-foreground">{label}</span>
        <span className="mt-1 block text-xs text-muted-foreground">{description}</span>
      </span>
      <input
        id={id}
        type="checkbox"
        checked={checked}
        onChange={(event) => onChange(event.target.checked)}
        className="mt-1 h-5 w-5 accent-[var(--primary)]"
      />
    </label>
  );
}
