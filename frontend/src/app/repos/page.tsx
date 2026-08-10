'use client';

import { useEffect, useMemo, useState } from 'react';
import { Link } from '@/lib/router';
import { useTranslation } from 'react-i18next';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';
import { Button } from '@/components/ui';
import { Input } from '@/components/ui';
import { useAuthStore } from '@/stores';
import { reposApi } from '@/lib/api/repos';
import type { Repo } from '@/lib/api/types';

type SortKey = 'updated' | 'name' | 'created';

function formatRelative(iso: string | null, locale: string): string {
  if (!iso) return '';
  const then = new Date(iso).getTime();
  if (Number.isNaN(then)) return '';
  const diffMs = Date.now() - then;
  const seconds = Math.round(diffMs / 1000);
  const minutes = Math.round(seconds / 60);
  const hours = Math.round(minutes / 60);
  const days = Math.round(hours / 24);
  const rtf = new Intl.RelativeTimeFormat(locale, { numeric: 'auto' });
  if (seconds < 60) return rtf.format(-seconds, 'second');
  if (minutes < 60) return rtf.format(-minutes, 'minute');
  if (hours < 24) return rtf.format(-hours, 'hour');
  if (days < 30) return rtf.format(-days, 'day');
  return new Date(iso).toLocaleDateString(locale);
}

function sortRepos(repos: Repo[], key: SortKey): Repo[] {
  const arr = [...repos];
  if (key === 'name') {
    arr.sort((a, b) => a.name.localeCompare(b.name));
  } else if (key === 'created') {
    arr.sort((a, b) => b.created_at.localeCompare(a.created_at));
  } else {
    arr.sort((a, b) => {
      const aTime = a.last_committed_at ?? a.updated_at;
      const bTime = b.last_committed_at ?? b.updated_at;
      return bTime.localeCompare(aTime);
    });
  }
  return arr;
}

export default function ReposPage() {
  const { t, i18n } = useTranslation();
  const { user, tenant } = useAuthStore();
  const [repos, setRepos] = useState<Repo[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [search, setSearch] = useState('');
  const [sortKey, setSortKey] = useState<SortKey>('updated');

  useEffect(() => {
    const tenantId: string | undefined = tenant?.id;
    if (!tenantId) {
      setLoading(false);
      return;
    }
    const tid: string = tenantId;
    let cancelled = false;
    async function fetchRepos() {
      try {
        const list = await reposApi.list(tid);
        if (!cancelled) {
          setRepos(list);
        }
      } catch (err) {
        if (!cancelled) {
          setError(t('repos.failedToConnect'));
        }
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    }
    fetchRepos();
    return () => {
      cancelled = true;
    };
  }, [tenant?.id, t]);

  const filtered = useMemo(() => {
    const needle = search.trim().toLowerCase();
    const matched = needle
      ? repos.filter(
          (r) =>
            r.name.toLowerCase().includes(needle) ||
            r.description.toLowerCase().includes(needle)
        )
      : repos;
    return sortRepos(matched, sortKey);
  }, [repos, search, sortKey]);

  const isEmpty = !loading && !error && repos.length === 0;
  const noMatches = !loading && !error && repos.length > 0 && filtered.length === 0;

  return (
    <div className="container mx-auto py-8">
      <div className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-3xl font-bold">{t('repos.title')}</h1>
          <p className="mt-1 text-muted-foreground">{t('repos.subtitle')}</p>
        </div>
        <Link to="/repos/new">
          <Button>{t('repos.createRepo')}</Button>
        </Link>
      </div>

      <div className="mt-6 flex flex-col gap-3 sm:flex-row sm:items-center">
        <div className="flex-1">
          <Input
            type="search"
            value={search}
            onChange={(event) => setSearch(event.target.value)}
            placeholder={t('repos.searchPlaceholder')}
            aria-label={t('repos.searchPlaceholder')}
          />
        </div>
        <div className="flex items-center gap-2">
          <label htmlFor="repo-sort" className="text-sm text-muted-foreground">
            {t('repos.sortBy')}
          </label>
          <select
            id="repo-sort"
            value={sortKey}
            onChange={(event) => setSortKey(event.target.value as SortKey)}
            className="flex h-10 rounded-[8px] border border-border bg-background px-3 py-2 text-sm"
          >
            <option value="updated">{t('repos.sortUpdated')}</option>
            <option value="name">{t('repos.sortName')}</option>
            <option value="created">{t('repos.sortCreated')}</option>
          </select>
        </div>
      </div>

      <div className="mt-6">
        {loading && (
          <div className="rounded-[24px] border border-border bg-card py-16 text-center text-muted-foreground">
            {t('repos.loading')}
          </div>
        )}

        {error && (
          <div className="rounded-[24px] border border-destructive bg-card py-12 text-center text-destructive">
            {error}
          </div>
        )}

        {isEmpty && <EmptyState />}

        {noMatches && (
          <div className="rounded-[24px] border border-border bg-card py-12 text-center text-muted-foreground">
            {t('repos.searchPlaceholder')}
          </div>
        )}

        {!loading && !error && filtered.length > 0 && (
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
            {filtered.map((repo) => (
              <RepoCard
                key={repo.id}
                repo={repo}
                locale={i18n.language}
              />
            ))}
          </div>
        )}
      </div>

      {!user?.tenant_id && !loading && (
        <p className="mt-6 text-xs text-muted-foreground">tenant_id unavailable</p>
      )}
    </div>
  );
}

function RepoCard({ repo, locale }: { repo: Repo; locale: string }) {
  const { t } = useTranslation();
  const lastCommitRelative = repo.last_committed_at
    ? formatRelative(repo.last_committed_at, locale)
    : null;
  return (
    <Link to={`/repos/${repo.id}`} aria-label={repo.name}>
      <Card className="flex h-full flex-col transition-shadow hover:shadow-md">
        <CardHeader>
          <CardTitle className="flex items-center justify-between gap-2">
            <span className="truncate">{repo.name}</span>
            <span
              className={`shrink-0 rounded px-2 py-0.5 text-xs font-medium ${
                repo.visibility === 'public'
                  ? 'bg-[var(--block-mint)] text-foreground'
                  : 'bg-muted text-foreground'
              }`}
            >
              {repo.visibility === 'public'
                ? t('repos.repo.public')
                : t('repos.repo.private')}
            </span>
          </CardTitle>
          <CardDescription className="line-clamp-2 min-h-[2.5em]">
            {repo.description || '—'}
          </CardDescription>
        </CardHeader>
        <CardContent className="mt-auto space-y-1 text-sm text-muted-foreground">
          <div className="flex items-center justify-between">
            <span>{t('repos.repo.branch')}</span>
            <span className="font-mono text-foreground">{repo.default_branch}</span>
          </div>
          <div className="flex items-center justify-between">
            <span className="truncate">
              {repo.last_commit_sha ? repo.last_commit_sha.slice(0, 7) : '—'}
            </span>
            <span>
              {lastCommitRelative
                ? t('repos.repo.lastCommit', { when: lastCommitRelative })
                : t('repos.repo.neverCommitted')}
            </span>
          </div>
        </CardContent>
      </Card>
    </Link>
  );
}

function EmptyState() {
  const { t } = useTranslation();
  return (
    <div className="rounded-[24px] border border-dashed border-border bg-surface-soft px-8 py-16 text-center">
      <h2 className="text-2xl font-semibold">{t('repos.noReposTitle')}</h2>
      <p className="mt-2 text-muted-foreground">{t('repos.noRepos')}</p>
      <div className="mt-6 flex justify-center">
        <Link to="/repos/new">
          <Button size="lg">{t('repos.createRepo')}</Button>
        </Link>
      </div>
    </div>
  );
}
