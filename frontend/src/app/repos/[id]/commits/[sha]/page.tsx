'use client';

import { useEffect, useState } from 'react';
import { GitCommitHorizontal } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import { Button, Card, CardContent, CardHeader, CardTitle } from '@/components/ui';
import { reposApi } from '@/lib/api/repos';
import type { CommitDetail, Repo } from '@/lib/api/types';
import { Link, useParams, useSearchParams } from '@/lib/router';
import { useAuthStore } from '@/stores';

type LoadState = 'loading' | 'ready' | 'not-found' | 'error';

function absoluteTime(timestamp: number, locale: string): string {
  return new Intl.DateTimeFormat(locale, {
    dateStyle: 'medium',
    timeStyle: 'long',
  }).format(new Date(timestamp * 1000));
}

export default function CommitEvidencePage() {
  const { t, i18n } = useTranslation();
  const { id, sha } = useParams<{ id: string; sha: string }>();
  const [searchParams] = useSearchParams();
  const { tenant } = useAuthStore();
  const [state, setState] = useState<LoadState>('loading');
  const [repo, setRepo] = useState<Repo | null>(null);
  const [detail, setDetail] = useState<CommitDetail | null>(null);

  useEffect(() => {
    if (!tenant?.id || !id || !sha) {
      setState('error');
      return;
    }
    let cancelled = false;
    setState('loading');
    setRepo(null);
    setDetail(null);

    reposApi.get(tenant.id, id)
      .then(async (loadedRepo) => {
        if (!loadedRepo) {
          if (!cancelled) setState('not-found');
          return;
        }
        const requestedRef = searchParams.get('ref') || loadedRepo.default_branch;
        const loadedDetail = await reposApi.commitDetail(tenant.id, id, sha, requestedRef);
        if (!cancelled) {
          setRepo(loadedRepo);
          setDetail(loadedDetail);
          setState('ready');
        }
      })
      .catch((error: unknown) => {
        if (cancelled) return;
        const status = (error as { response?: { status?: number } }).response?.status;
        setState(status === 404 ? 'not-found' : 'error');
      });
    return () => { cancelled = true; };
  }, [tenant?.id, id, sha, searchParams]);

  if (state === 'loading') {
    return <main className="container mx-auto py-8 text-muted-foreground" aria-busy="true">{t('repos.commitEvidence.loading')}</main>;
  }

  if (state !== 'ready' || !repo || !detail) {
    return (
      <main className="container mx-auto max-w-3xl py-8">
        <Card>
          <CardContent className="space-y-4 py-12 text-center">
            <h1 className="text-2xl font-bold">{t(state === 'not-found' ? 'repos.commitEvidence.notFound' : 'repos.commitEvidence.loadFailed')}</h1>
            <Link to={id ? `/repos/${id}/commits` : '/repos'}><Button variant="outline">{t('repos.commitEvidence.back')}</Button></Link>
          </CardContent>
        </Card>
      </main>
    );
  }

  const refQuery = `?ref=${encodeURIComponent(detail.ref_name)}`;
  return (
    <main className="container mx-auto space-y-6 py-8">
      <div className="flex flex-col gap-4 lg:flex-row lg:items-start lg:justify-between">
        <div className="min-w-0">
          <p className="text-sm font-medium text-primary">{repo.name}</p>
          <h1 className="mt-2 flex items-center gap-2 text-3xl font-bold">
            <GitCommitHorizontal className="h-7 w-7 shrink-0" aria-hidden="true" />
            {t('repos.commitEvidence.title')}
          </h1>
          <code className="mt-3 block break-all text-xs text-muted-foreground">{detail.sha}</code>
        </div>
        <Link to={`/repos/${repo.id}/commits`}><Button variant="outline">{t('repos.commitEvidence.back')}</Button></Link>
      </div>

      <Card>
        <CardHeader><CardTitle>{t('repos.commitEvidence.message')}</CardTitle></CardHeader>
        <CardContent>
          <pre className="whitespace-pre-wrap break-words font-sans text-sm leading-6">{detail.message.trimEnd()}</pre>
        </CardContent>
      </Card>

      <Card>
        <CardHeader><CardTitle>{t('repos.commitEvidence.identity')}</CardTitle></CardHeader>
        <CardContent className="grid gap-5 md:grid-cols-2">
          <Identity label={t('repos.commitEvidence.author')} name={detail.author_name} email={detail.author_email} time={absoluteTime(detail.authored_at, i18n.language)} />
          <Identity label={t('repos.commitEvidence.committer')} name={detail.committer_name} email={detail.committer_email} time={absoluteTime(detail.committed_at, i18n.language)} />
          <Evidence label={t('repos.commitEvidence.ref')} value={detail.ref_name} />
          <div>
            <p className="text-sm text-muted-foreground">{t('repos.commitEvidence.parents')}</p>
            {detail.parents.length === 0 ? (
              <p className="mt-1 text-sm">{t('repos.commitEvidence.rootCommit')}</p>
            ) : (
              <div className="mt-1 space-y-1">
                {detail.parents.map((parent) => (
                  <Link key={parent} to={`/repos/${repo.id}/commits/${parent}${refQuery}`} className="block break-all font-mono text-xs underline-offset-4 hover:underline">
                    {parent}
                  </Link>
                ))}
              </div>
            )}
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader><CardTitle>{t('repos.commitEvidence.changes', { count: detail.changes.length })}</CardTitle></CardHeader>
        <CardContent>
          {detail.changes.length === 0 ? (
            <p className="text-sm text-muted-foreground">{t('repos.commitEvidence.noChanges')}</p>
          ) : (
            <div className="overflow-x-auto rounded-[8px] border border-border">
              <table className="w-full min-w-[720px] text-left text-sm">
                <thead className="bg-muted/60 text-xs uppercase text-muted-foreground">
                  <tr><th className="px-3 py-2">{t('repos.commitEvidence.status')}</th><th className="px-3 py-2">{t('repos.commitEvidence.path')}</th><th className="px-3 py-2">{t('repos.commitEvidence.oldObject')}</th><th className="px-3 py-2">{t('repos.commitEvidence.newObject')}</th></tr>
                </thead>
                <tbody>
                  {detail.changes.map((change) => (
                    <tr key={`${change.path}:${change.old_oid}:${change.new_oid}`} className="border-t border-border">
                      <td className="px-3 py-3"><span className="rounded-[6px] bg-muted px-2 py-1 text-xs">{t(`repos.commitEvidence.changeType.${change.change_type}`)}</span></td>
                      <td className="px-3 py-3 font-mono text-xs">{change.path}</td>
                      <td className="px-3 py-3 font-mono text-xs text-muted-foreground">{change.old_oid ? change.old_oid.slice(0, 12) : '—'}</td>
                      <td className="px-3 py-3 font-mono text-xs text-muted-foreground">{change.new_oid ? change.new_oid.slice(0, 12) : '—'}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </CardContent>
      </Card>
    </main>
  );
}

function Identity({ label, name, email, time }: { label: string; name: string; email: string; time: string }) {
  return <div><p className="text-sm text-muted-foreground">{label}</p><p className="mt-1 font-medium">{name}</p><p className="break-all text-xs text-muted-foreground">{email}</p><p className="mt-1 text-xs text-muted-foreground">{time}</p></div>;
}

function Evidence({ label, value }: { label: string; value: string }) {
  return <div><p className="text-sm text-muted-foreground">{label}</p><code className="mt-1 block break-all text-xs">{value}</code></div>;
}
