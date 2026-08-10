'use client';

import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Link } from '@/lib/router';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { useAuthStore } from '@/stores';
import { reposApi } from '@/lib/api/repos';
import { membersApi } from '@/lib/api/members';
import type { Repo, Member } from '@/lib/api/types';
import { usePermission } from '@/hooks/usePermission';
import { canAccessSettingsSection } from '@/lib/settings-policy';

const THIRTY_DAYS_MS = 30 * 24 * 60 * 60 * 1000;

export default function DashboardPage() {
  const { t, i18n } = useTranslation();
  const { user, tenant } = useAuthStore();
  const { tenantRole } = usePermission();
  const [repos, setRepos] = useState<Repo[]>([]);
  const [members, setMembers] = useState<Member[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const tenantId: string | undefined = tenant?.id;
    if (!tenantId) {
      setLoading(false);
      return;
    }
    const tid: string = tenantId;
    let cancelled = false;
    async function fetchData() {
      try {
        const [reposList, membersResponse] = await Promise.all([
          reposApi.list(tid).catch(() => [] as Repo[]),
          membersApi.list(tid).catch(() => null),
        ]);
        if (cancelled) {
          return;
        }
        setRepos(reposList);
        setMembers(membersResponse?.data?.members ?? []);
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    }
    fetchData();
    return () => {
      cancelled = true;
    };
  }, [tenant?.id]);

  const recentCommits = useMemo(() => {
    const cutoff = Date.now() - THIRTY_DAYS_MS;
    return repos.filter((repo) => {
      if (!repo.last_committed_at) return false;
      const ts = new Date(repo.last_committed_at).getTime();
      return Number.isFinite(ts) && ts >= cutoff;
    }).length;
  }, [repos]);

  const recentRepos = useMemo(() => {
    return [...repos]
      .sort((a, b) => {
        const aTime = a.last_committed_at ?? a.updated_at;
        const bTime = b.last_committed_at ?? b.updated_at;
        return bTime.localeCompare(aTime);
      })
      .slice(0, 4);
  }, [repos]);

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">
          {t('dashboard.welcomeBack', { name: user?.username || t('nav.user') })}
        </h1>
        <p className="text-muted-foreground">
          {tenant?.name ? t('dashboard.orgPrefix', { name: tenant.name }) : t('dashboard.repoCentric.subtitle')}
        </p>
      </div>

      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <StatCard
          title={t('dashboard.stats.repos')}
          value={loading ? '...' : repos.length.toString()}
          description={t('dashboard.stats.reposDesc')}
          href="/repos"
        />
        <StatCard
          title={t('dashboard.stats.commitsMonth')}
          value={loading ? '...' : recentCommits.toString()}
          description={t('dashboard.stats.commitsMonthDesc')}
          href="/repos"
        />
        <StatCard
          title={t('dashboard.stats.members')}
          value={loading ? '...' : members.length.toString()}
          description={t('dashboard.stats.membersDesc')}
          href="/settings/members"
        />
        <StatCard
          title={t('dashboard.stats.withCommits')}
          value={loading ? '...' : recentRepos.length.toString()}
          description={t('dashboard.stats.reposDesc')}
          href="/repos"
        />
      </div>

      <Card>
        <CardHeader>
          <CardTitle>{t('dashboard.quickActions.title')}</CardTitle>
          <CardDescription>{t('dashboard.quickActions.subtitle')}</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-4">
            <QuickActionButton label={t('dashboard.quickActions.newRepo')} href="/repos/new" />
            <QuickActionButton label={t('dashboard.quickActions.browseRepos')} href="/repos" />
            <QuickActionButton label={t('dashboard.quickActions.teamMembers')} href="/settings/members" />
            {canAccessSettingsSection(tenantRole, 'api-keys') && (
              <QuickActionButton label={t('dashboard.quickActions.apiKeys')} href="/settings/api-keys" />
            )}
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="flex flex-row items-start justify-between gap-4">
          <div>
            <CardTitle>{t('dashboard.recentRepos.title')}</CardTitle>
            <CardDescription>{t('dashboard.recentRepos.subtitle')}</CardDescription>
          </div>
          <Link to="/repos">
            <Button variant="ghost" size="sm">{t('dashboard.recentRepos.viewAll')}</Button>
          </Link>
        </CardHeader>
        <CardContent>
          {!loading && recentRepos.length === 0 ? (
            <div className="rounded-[24px] border border-dashed border-border bg-surface-soft px-6 py-10 text-center">
              <p className="text-sm text-muted-foreground">
                {t('dashboard.recentRepos.empty')}
              </p>
              <div className="mt-4 flex justify-center">
                <Link to="/repos/new">
                  <Button>{t('dashboard.quickActions.newRepo')}</Button>
                </Link>
              </div>
            </div>
          ) : (
            <ul className="divide-y divide-border">
              {recentRepos.map((repo) => (
                <li key={repo.id} className="flex items-center justify-between py-3">
                  <div className="min-w-0">
                    <Link
                      to={`/repos/${repo.id}`}
                      className="block truncate text-sm font-medium text-foreground hover:underline"
                    >
                      {repo.name}
                    </Link>
                    <p className="truncate text-xs text-muted-foreground">
                      {repo.description || '—'}
                    </p>
                  </div>
                  <div className="ml-4 shrink-0 text-right text-xs text-muted-foreground">
                    <div className="font-mono">
                      {repo.last_commit_sha ? repo.last_commit_sha.slice(0, 7) : '—'}
                    </div>
                    <div>
                      {repo.last_committed_at
                        ? formatRelative(repo.last_committed_at, i18n.language)
                        : t('dashboard.recentRepos.neverCommitted')}
                    </div>
                  </div>
                </li>
              ))}
            </ul>
          )}
        </CardContent>
      </Card>

      {tenant && (
        <div className="grid gap-4 md:grid-cols-2">
          <Card>
            <CardHeader>
              <CardTitle>{t('dashboard.organization.title')}</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-2">
                <div className="flex justify-between">
                  <span className="text-muted-foreground">{t('dashboard.organization.plan')}</span>
                  <span className="font-medium capitalize">{tenant.plan || 'Free'}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-muted-foreground">{t('dashboard.organization.role')}</span>
                  <span className="font-medium capitalize">{user?.tenant_role || 'member'}</span>
                </div>
              </div>
              {canAccessSettingsSection(tenantRole, 'billing') && (
                <Link to="/settings/billing">
                  <Button variant="outline" className="mt-4 w-full">
                    {t('dashboard.organization.manageSubscription')}
                  </Button>
                </Link>
              )}
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>{t('dashboard.quickLinks.title')}</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-2">
                <Link to="/repos/new" className="block text-sm text-primary hover:underline">
                  → {t('dashboard.quickLinks.createRepo')}
                </Link>
                <Link to="/repos" className="block text-sm text-primary hover:underline">
                  → {t('dashboard.quickLinks.browseRepos')}
                </Link>
                {canAccessSettingsSection(tenantRole, 'api-keys') && (
                  <Link to="/settings/api-keys" className="block text-sm text-primary hover:underline">
                    → {t('dashboard.quickLinks.manageApiKeys')}
                  </Link>
                )}
                <Link to="/settings/members" className="block text-sm text-primary hover:underline">
                  → {t('dashboard.quickLinks.inviteTeamMembers')}
                </Link>
                {canAccessSettingsSection(tenantRole, 'workspace') && (
                  <Link to="/settings/workspace" className="block text-sm text-primary hover:underline">
                    → {t('dashboard.quickLinks.orgSettings')}
                  </Link>
                )}
              </div>
            </CardContent>
          </Card>
        </div>
      )}
    </div>
  );
}

function StatCard({
  title,
  value,
  description,
  href,
}: {
  title: string;
  value: string;
  description: string;
  href?: string;
}) {
  const content = (
    <Card className="h-full">
      <CardContent className="pt-6">
        <div className="text-3xl font-semibold">{value}</div>
        <div className="mt-1 text-sm font-medium text-foreground">{title}</div>
        <div className="mt-1 text-xs text-muted-foreground">{description}</div>
      </CardContent>
    </Card>
  );

  if (href) {
    return <Link to={href}>{content}</Link>;
  }
  return content;
}

function QuickActionButton({ label, href }: { label: string; href: string }) {
  return (
    <Link to={href}>
      <Button variant="outline" className="h-20 w-full">
        {label}
      </Button>
    </Link>
  );
}

function formatRelative(iso: string, locale: string): string {
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
