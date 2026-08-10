'use client';

import { useEffect, useMemo, useState } from 'react';
import { Link, useParams, useRouter } from '@/lib/router';
import { useTranslation } from 'react-i18next';
import {
  Check,
  ChevronDown,
  ChevronRight,
  Clipboard,
  File,
  Folder,
  FolderOpen,
  GitCommitHorizontal,
  Trash2,
} from 'lucide-react';
import { parse } from 'yaml';
import { Button, Card, CardContent, CardHeader, CardTitle, Input, Modal } from '@/components/ui';
import { useAuthStore } from '@/stores';
import { reposApi } from '@/lib/api/repos';
import config from '@/lib/config';
import type { CommitInfo, FileTreeEntry, Repo, UpdateRepoRequest } from '@/lib/api/types';

type Tab = 'overview' | 'files' | 'commits' | 'settings';

interface PolicySummary {
  defaultAction: string;
  protectedPaths: string[];
  agents: string[];
}

const tabs: Tab[] = ['overview', 'files', 'commits', 'settings'];
const REPO_NAME_PATTERN = /^[a-zA-Z0-9._-]+$/;

function isTab(value: string | undefined): value is Tab {
  return value !== undefined && tabs.includes(value as Tab);
}

function cloneUrlFor(repoId: string): string {
  if (typeof window === 'undefined') return `/repos/${repoId}`;
  const apiUrl = new URL(config.apiBaseUrl, window.location.origin);
  const basePath = apiUrl.pathname.replace(/\/api\/v1\/?$/, '').replace(/\/$/, '');
  return `${apiUrl.origin}${basePath}/repos/${repoId}`;
}

function formatRelative(timestamp: number, locale: string): string {
  const diffSeconds = Math.round((Date.now() - timestamp * 1000) / 1000);
  const rtf = new Intl.RelativeTimeFormat(locale, { numeric: 'auto' });
  if (diffSeconds < 60) return rtf.format(-diffSeconds, 'second');
  const minutes = Math.round(diffSeconds / 60);
  if (minutes < 60) return rtf.format(-minutes, 'minute');
  const hours = Math.round(minutes / 60);
  if (hours < 24) return rtf.format(-hours, 'hour');
  const days = Math.round(hours / 24);
  if (days < 30) return rtf.format(-days, 'day');
  return new Date(timestamp * 1000).toLocaleDateString(locale);
}

function parsePolicy(content: string): PolicySummary | null {
  const value = parse(content) as unknown;
  if (!value || typeof value !== 'object' || Array.isArray(value)) return null;
  const record = value as Record<string, unknown>;
  const paths = Array.isArray(record.protected_paths)
    ? record.protected_paths.filter((item): item is string => typeof item === 'string')
    : [];
  const rawAgents = Array.isArray(record.agents) ? record.agents : [];
  const agents = rawAgents.flatMap((item) => {
    if (typeof item === 'string') return [item];
    if (item && typeof item === 'object' && 'name' in item && typeof item.name === 'string') {
      return [item.name];
    }
    return [];
  });
  return {
    defaultAction: typeof record.default_action === 'string' ? record.default_action : 'require_review',
    protectedPaths: paths,
    agents,
  };
}

export default function RepoDetailPage() {
  const { t } = useTranslation();
  const { id, tab: requestedTab } = useParams<{ id: string; tab?: string }>();
  const { tenant } = useAuthStore();
  const router = useRouter();
  const tab: Tab = isTab(requestedTab) ? requestedTab : 'overview';
  const [repo, setRepo] = useState<Repo | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<'forbidden' | 'load' | null>(null);

  useEffect(() => {
    if (requestedTab !== tab && id) router.replace(`/repos/${id}/${tab}`);
  }, [id, requestedTab, router, tab]);

  useEffect(() => {
    if (!tenant?.id || !id) {
      setLoading(false);
      return;
    }
    let cancelled = false;
    setLoading(true);
    setError(null);
    reposApi.get(tenant.id, id)
      .then((value) => { if (!cancelled) setRepo(value); })
      .catch((reason) => {
        if (cancelled) return;
        const status = (reason as { response?: { status?: number } }).response?.status;
        setError(status === 403 ? 'forbidden' : 'load');
      })
      .finally(() => { if (!cancelled) setLoading(false); });
    return () => { cancelled = true; };
  }, [tenant?.id, id]);

  if (loading) {
    return <div className="container mx-auto py-8 text-muted-foreground">{t('repos.detail.loading')}</div>;
  }

  if (error || !repo) {
    return (
      <div className="container mx-auto max-w-3xl py-8">
        <Card>
          <CardContent className="space-y-4 py-12 text-center">
            <h1 className="text-2xl font-bold">
              {error === 'forbidden'
                ? t('repos.forbiddenTitle')
                : error === 'load'
                  ? t('repos.detail.loadFailed')
                  : t('repos.detail.notFound')}
            </h1>
            {error === 'forbidden' && (
              <p className="text-muted-foreground">{t('repos.forbiddenDescription')}</p>
            )}
            <Link to="/repos"><Button variant="outline">{t('repos.detail.back')}</Button></Link>
          </CardContent>
        </Card>
      </div>
    );
  }

  const cloneUrl = cloneUrlFor(repo.id);
  return (
    <div className="container mx-auto py-8">
      <div className="mb-6 flex flex-col gap-4 lg:flex-row lg:items-start lg:justify-between">
        <div className="min-w-0">
          <div className="flex flex-wrap items-center gap-3">
            <h1 className="min-w-0 break-words text-3xl font-bold">{repo.name}</h1>
            <span className="rounded-[6px] bg-muted px-2 py-1 text-xs">{repo.visibility}</span>
          </div>
          <p className="mt-2 text-muted-foreground">{repo.description || t('repos.detail.noDescription')}</p>
          <p className="mt-2 font-mono text-xs text-muted-foreground">{repo.default_branch}</p>
        </div>
        <div className="flex min-w-0 flex-col gap-2 sm:flex-row">
          <CloneUrl value={cloneUrl} compact />
          <Link to="/repos"><Button variant="outline" className="w-full sm:w-auto">{t('repos.detail.back')}</Button></Link>
        </div>
      </div>

      <nav className="mb-6 flex gap-1 overflow-x-auto border-b border-border" aria-label={t('repos.detail.tabs')}>
        {tabs.map((name) => (
          <Link
            key={name}
            to={`/repos/${repo.id}/${name}`}
            className={`border-b-2 px-4 py-3 text-sm ${tab === name ? 'border-primary font-semibold text-foreground' : 'border-transparent text-muted-foreground hover:text-foreground'}`}
          >
            {t(`repos.detail.${name}`)}
          </Link>
        ))}
      </nav>

      {tab === 'overview' && <Overview repo={repo} tenantId={tenant!.id} />}
      {tab === 'files' && <Files repo={repo} tenantId={tenant!.id} />}
      {tab === 'commits' && <Commits repo={repo} tenantId={tenant!.id} />}
      {tab === 'settings' && (
        <Settings
          repo={repo}
          tenantId={tenant!.id}
          cloneUrl={cloneUrl}
          onSaved={setRepo}
          onDeleted={() => router.push('/repos')}
        />
      )}
    </div>
  );
}

function CloneUrl({ value, compact = false }: { value: string; compact?: boolean }) {
  const { t } = useTranslation();
  const [copied, setCopied] = useState(false);
  const copy = async () => {
    await navigator.clipboard.writeText(value);
    setCopied(true);
    window.setTimeout(() => setCopied(false), 1600);
  };
  return (
    <div className={`flex min-w-0 items-center gap-2 ${compact ? 'sm:max-w-md' : ''}`}>
      <code className="min-w-0 flex-1 truncate rounded-[6px] border border-border bg-muted px-3 py-2 text-xs" title={value}>{value}</code>
      <Button type="button" variant="outline" size="sm" onClick={copy} title={t('repos.detail.copyCloneUrl')} aria-label={t('repos.detail.copyCloneUrl')}>
        {copied ? <Check className="h-4 w-4" aria-hidden="true" /> : <Clipboard className="h-4 w-4" aria-hidden="true" />}
        {!compact && <span className="ml-2">{copied ? t('common.copied') : t('common.copy')}</span>}
      </Button>
    </div>
  );
}

function Overview({ repo, tenantId }: { repo: Repo; tenantId: string }) {
  const { t } = useTranslation();
  const [readme, setReadme] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(false);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    setError(false);
    reposApi.fileTree(tenantId, repo.id, repo.default_branch)
      .then(async (entries) => {
        const entry = entries.find((item) => !item.is_tree && /(^|\/)readme\.md$/i.test(item.name));
        if (!entry) return null;
        return reposApi.blob(tenantId, repo.id, entry.oid);
      })
      .then((blob) => { if (!cancelled) setReadme(blob?.encoding === 'utf-8' ? blob.content : null); })
      .catch(() => { if (!cancelled) setError(true); })
      .finally(() => { if (!cancelled) setLoading(false); });
    return () => { cancelled = true; };
  }, [tenantId, repo.id, repo.default_branch]);

  return (
    <div className="space-y-4">
      <Card>
        <CardHeader><CardTitle>{t('repos.detail.overviewTitle')}</CardTitle></CardHeader>
        <CardContent className="grid gap-4 sm:grid-cols-3">
          <Summary label={t('repos.detail.branch')} value={repo.default_branch} mono />
          <Summary label={t('repos.detail.latestCommit')} value={repo.last_commit_sha?.slice(0, 7) ?? '-'} mono />
          <Summary label={t('repos.detail.policy')} value={repo.require_review ? t('repos.detail.reviewRequired') : t('repos.detail.autoMerge')} />
        </CardContent>
      </Card>
      <Card>
        <CardHeader><CardTitle>README.md</CardTitle></CardHeader>
        <CardContent>
          {loading && <p className="text-sm text-muted-foreground">{t('common.loading')}</p>}
          {error && <p className="text-sm text-destructive">{t('repos.detail.loadFailed')}</p>}
          {!loading && !error && readme && <pre className="max-h-[560px] overflow-auto whitespace-pre-wrap break-words text-sm leading-6">{readme}</pre>}
          {!loading && !error && !readme && <p className="text-sm text-muted-foreground">{t('repos.detail.noReadme')}</p>}
        </CardContent>
      </Card>
      <Link to={`/repos/${repo.id}/files`}><Button>{t('repos.detail.browseCode')}</Button></Link>
    </div>
  );
}

function Summary({ label, value, mono = false }: { label: string; value: string; mono?: boolean }) {
  return <div><p className="text-sm text-muted-foreground">{label}</p><p className={mono ? 'font-mono' : ''}>{value}</p></div>;
}

function Files({ repo, tenantId }: { repo: Repo; tenantId: string }) {
  const { t } = useTranslation();
  const [entries, setEntries] = useState<FileTreeEntry[]>([]);
  const [collapsed, setCollapsed] = useState<Set<string>>(new Set());
  const [selected, setSelected] = useState<string | null>(null);
  const [content, setContent] = useState<string | null>(null);
  const [binary, setBinary] = useState(false);
  const [loading, setLoading] = useState(true);
  const [blobLoading, setBlobLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    setError(null);
    setEntries([]);
    setCollapsed(new Set());
    setSelected(null);
    setContent(null);
    setBinary(false);
    setBlobLoading(false);
    reposApi.fileTree(tenantId, repo.id, repo.default_branch)
      .then((items) => { if (!cancelled) setEntries(items); })
      .catch(() => { if (!cancelled) setError(t('repos.detail.loadFailed')); })
      .finally(() => { if (!cancelled) setLoading(false); });
    return () => { cancelled = true; };
  }, [tenantId, repo.id, repo.default_branch, t]);

  const visibleEntries = useMemo(() => entries.filter((entry) => {
    const segments = entry.name.split('/');
    segments.pop();
    let path = '';
    return segments.every((segment) => {
      path = path ? `${path}/${segment}` : segment;
      return !collapsed.has(path);
    });
  }), [collapsed, entries]);

  const open = async (entry: FileTreeEntry) => {
    if (entry.is_tree) {
      setCollapsed((current) => {
        const next = new Set(current);
        if (next.has(entry.name)) next.delete(entry.name); else next.add(entry.name);
        return next;
      });
      return;
    }
    setSelected(entry.name);
    setContent(null);
    setBinary(false);
    setBlobLoading(true);
    setError(null);
    try {
      const blob = await reposApi.blob(tenantId, repo.id, entry.oid);
      setBinary(blob.encoding === 'base64');
      setContent(blob.encoding === 'utf-8' ? blob.content : null);
    } catch {
      setError(t('repos.detail.loadFailed'));
    } finally {
      setBlobLoading(false);
    }
  };

  return (
    <div className="grid min-h-[480px] gap-4 lg:grid-cols-[minmax(240px,0.36fr)_minmax(0,1fr)]">
      <Card>
        <CardHeader><CardTitle>{t('repos.detail.files')}</CardTitle></CardHeader>
        <CardContent>
          {loading && <p className="text-sm text-muted-foreground">{t('common.loading')}</p>}
          {!loading && visibleEntries.length === 0 && !error && <p className="text-sm text-muted-foreground">{t('repos.detail.emptyFiles')}</p>}
          <div className="space-y-1">
            {visibleEntries.map((entry) => {
              const depth = entry.name.split('/').length - 1;
              const label = entry.name.split('/').at(-1) ?? entry.name;
              const isCollapsed = collapsed.has(entry.name);
              return (
                <button
                  key={`${entry.oid}:${entry.name}`}
                  type="button"
                  onClick={() => open(entry)}
                  className={`flex w-full min-w-0 items-center gap-2 rounded-[6px] py-1.5 pr-2 text-left text-sm hover:bg-muted ${selected === entry.name ? 'bg-muted font-medium' : ''}`}
                  style={{ paddingLeft: `${8 + depth * 16}px` }}
                >
                  {entry.is_tree ? (
                    <>{isCollapsed ? <ChevronRight className="h-4 w-4 shrink-0" /> : <ChevronDown className="h-4 w-4 shrink-0" />}{isCollapsed ? <Folder className="h-4 w-4 shrink-0" /> : <FolderOpen className="h-4 w-4 shrink-0" />}</>
                  ) : <><span className="w-4 shrink-0" /><File className="h-4 w-4 shrink-0" /></>}
                  <span className="truncate">{label}</span>
                </button>
              );
            })}
          </div>
        </CardContent>
      </Card>
      <Card className="min-w-0">
        <CardHeader><CardTitle className="break-all">{selected ? `${repo.name} / ${selected}` : t('repos.detail.selectFile')}</CardTitle></CardHeader>
        <CardContent>
          {error && <p className="text-sm text-destructive">{error}</p>}
          {blobLoading && <p className="text-sm text-muted-foreground">{t('common.loading')}</p>}
          {!error && !blobLoading && binary && <p className="text-sm text-muted-foreground">{t('repos.detail.binaryFile')}</p>}
          {!error && !blobLoading && content !== null && <pre className="max-h-[560px] overflow-auto whitespace-pre-wrap break-words text-sm leading-6">{content || t('repos.detail.emptyFile')}</pre>}
          {!error && !blobLoading && !binary && content === null && <p className="text-sm text-muted-foreground">{t('repos.detail.selectFile')}</p>}
        </CardContent>
      </Card>
    </div>
  );
}

function Commits({ repo, tenantId }: { repo: Repo; tenantId: string }) {
  const { t, i18n } = useTranslation();
  const [commits, setCommits] = useState<CommitInfo[]>([]);
  const [visibleCount, setVisibleCount] = useState(20);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(false);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    setError(false);
    setCommits([]);
    setVisibleCount(20);
    reposApi.commits(tenantId, repo.id, repo.default_branch, 50)
      .then((items) => { if (!cancelled) setCommits(items); })
      .catch(() => { if (!cancelled) setError(true); })
      .finally(() => { if (!cancelled) setLoading(false); });
    return () => { cancelled = true; };
  }, [tenantId, repo.id, repo.default_branch]);

  return (
    <Card>
      <CardHeader><CardTitle>{t('repos.detail.commits')}</CardTitle></CardHeader>
      <CardContent>
        {loading && <p className="text-muted-foreground">{t('common.loading')}</p>}
        {error && <p className="text-destructive">{t('repos.detail.loadFailed')}</p>}
        <div className="space-y-3">
          {commits.slice(0, visibleCount).map((commit) => (
            <div key={commit.sha} className="flex gap-3 border-b border-border pb-3">
              <GitCommitHorizontal className="mt-1 h-4 w-4 shrink-0 text-muted-foreground" aria-hidden="true" />
              <div className="min-w-0">
                <p className="font-medium break-words">{commit.message.split('\n')[0]}</p>
                <p className="mt-1 text-xs text-muted-foreground">
                  <Link
                    to={`/repos/${repo.id}/commits/${commit.sha}?ref=${encodeURIComponent(repo.default_branch)}`}
                    className="font-mono text-foreground underline-offset-4 hover:underline focus-visible:rounded-sm focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                  >
                    {commit.sha.slice(0, 7)}
                  </Link>
                  {' '}{t('repos.detail.committedBy', { author: commit.author_name, when: formatRelative(commit.timestamp, i18n.language) })}
                </p>
              </div>
            </div>
          ))}
          {!loading && !error && commits.length === 0 && <p className="text-muted-foreground">{t('repos.detail.noCommits')}</p>}
        </div>
        {visibleCount < commits.length && <Button className="mt-4" variant="outline" onClick={() => setVisibleCount((count) => count + 20)}>{t('repos.detail.loadMore')}</Button>}
      </CardContent>
    </Card>
  );
}

function Settings({ repo, tenantId, cloneUrl, onSaved, onDeleted }: { repo: Repo; tenantId: string; cloneUrl: string; onSaved: (repo: Repo) => void; onDeleted: () => void }) {
  const { t } = useTranslation();
  const [name, setName] = useState(repo.name);
  const [description, setDescription] = useState(repo.description);
  const [visibility, setVisibility] = useState(repo.visibility);
  const [autoMerge, setAutoMerge] = useState(repo.auto_merge);
  const [requireReview, setRequireReview] = useState(repo.require_review);
  const [policy, setPolicy] = useState<PolicySummary | null>(null);
  const [policyError, setPolicyError] = useState(false);
  const [saving, setSaving] = useState(false);
  const [message, setMessage] = useState<string | null>(null);
  const [deleteOpen, setDeleteOpen] = useState(false);
  const [deleteName, setDeleteName] = useState('');
  const [deleting, setDeleting] = useState(false);

  useEffect(() => {
    setName(repo.name);
    setDescription(repo.description);
    setVisibility(repo.visibility);
    setAutoMerge(repo.auto_merge);
    setRequireReview(repo.require_review);
    setMessage(null);
    setDeleteOpen(false);
    setDeleteName('');
  }, [repo.id, repo.name, repo.description, repo.visibility, repo.auto_merge, repo.require_review]);

  useEffect(() => {
    let cancelled = false;
    setPolicy(null);
    setPolicyError(false);
    reposApi.fileTree(tenantId, repo.id, repo.default_branch)
      .then(async (entries) => {
        const entry = entries.find((item) => !item.is_tree && item.name === '.evolith/policy.yaml');
        if (!entry) return null;
        const blob = await reposApi.blob(tenantId, repo.id, entry.oid);
        return blob.encoding === 'utf-8' ? parsePolicy(blob.content) : null;
      })
      .then((summary) => { if (!cancelled) setPolicy(summary); })
      .catch(() => { if (!cancelled) setPolicyError(true); });
    return () => { cancelled = true; };
  }, [tenantId, repo.id, repo.default_branch]);

  const save = async () => {
    const trimmedName = name.trim();
    if (!trimmedName || !REPO_NAME_PATTERN.test(trimmedName) || trimmedName.length > 128) {
      setMessage(t('repos.detail.invalidName'));
      return;
    }
    setSaving(true);
    setMessage(null);
    try {
      const update: UpdateRepoRequest = { name: trimmedName, description: description.trim(), visibility, auto_merge: autoMerge, require_review: requireReview };
      const saved = await reposApi.update(tenantId, repo.id, update);
      onSaved(saved);
      setName(saved.name);
      setDescription(saved.description);
      setMessage(t('repos.detail.saved'));
    } catch {
      setMessage(t('repos.detail.saveFailed'));
    } finally {
      setSaving(false);
    }
  };

  const remove = async () => {
    if (deleteName !== repo.name) return;
    setDeleting(true);
    setMessage(null);
    try {
      await reposApi.delete(tenantId, repo.id);
      onDeleted();
    } catch {
      setMessage(t('repos.detail.deleteFailed'));
      setDeleteOpen(false);
    } finally {
      setDeleting(false);
    }
  };

  return (
    <div className="space-y-4">
      <Card>
        <CardHeader><CardTitle>{t('repos.detail.settings')}</CardTitle></CardHeader>
        <CardContent className="space-y-5">
          <div><label className="mb-1 block text-sm font-medium" htmlFor="repo-name">{t('common.name')}</label><Input id="repo-name" value={name} onChange={(event) => setName(event.target.value)} /></div>
          <div><label className="mb-1 block text-sm font-medium" htmlFor="repo-description">{t('repos.detail.description')}</label><Input id="repo-description" value={description} onChange={(event) => setDescription(event.target.value)} /></div>
          <div>
            <label className="mb-1 block text-sm font-medium" htmlFor="repo-visibility">{t('repos.detail.visibility')}</label>
            <select id="repo-visibility" value={visibility} onChange={(event) => setVisibility(event.target.value as Repo['visibility'])} className="flex h-10 w-full rounded-[8px] border border-border bg-background px-3 py-2 text-sm">
              <option value="private">{t('common.private')}</option><option value="public">{t('common.public')}</option>
            </select>
          </div>
          <SettingToggle label={t('repos.detail.autoMergeSetting')} checked={autoMerge} onChange={setAutoMerge} />
          <SettingToggle label={t('repos.detail.requireReviewSetting')} checked={requireReview} onChange={setRequireReview} />
          <div><p className="mb-1 text-sm font-medium">{t('repos.detail.cloneUrl')}</p><CloneUrl value={cloneUrl} /></div>
          <div className="flex flex-wrap items-center gap-3"><Button onClick={save} disabled={saving}>{saving ? t('repos.detail.saving') : t('repos.detail.save')}</Button>{message && <span className="text-sm text-muted-foreground">{message}</span>}</div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader><CardTitle>{t('repos.detail.policy')}</CardTitle></CardHeader>
        <CardContent>
          {policyError && <p className="text-sm text-destructive">{t('repos.detail.policyLoadFailed')}</p>}
          {!policyError && !policy && <p className="text-sm text-muted-foreground">{t('repos.detail.noPolicy')}</p>}
          {policy && <div className="grid gap-4 sm:grid-cols-3"><Summary label={t('repos.detail.defaultAction')} value={policy.defaultAction} /><Summary label={t('repos.detail.protectedPaths')} value={policy.protectedPaths.join(', ') || '-'} /><Summary label={t('repos.detail.agents')} value={policy.agents.join(', ') || '-'} /></div>}
        </CardContent>
      </Card>

      <Card className="border-destructive">
        <CardContent className="flex flex-col gap-4 py-6 sm:flex-row sm:items-center sm:justify-between">
          <div><p className="font-medium">{t('repos.detail.dangerZone')}</p><p className="text-sm text-muted-foreground">{t('repos.detail.deleteHint', { name: repo.name })}</p></div>
          <Button variant="destructive" onClick={() => { setDeleteName(''); setDeleteOpen(true); }}><Trash2 className="mr-2 h-4 w-4" />{t('repos.detail.delete')}</Button>
        </CardContent>
      </Card>

      <Modal isOpen={deleteOpen} onClose={() => setDeleteOpen(false)} title={t('repos.detail.confirmDelete')} size="sm" footer={<><Button variant="outline" onClick={() => setDeleteOpen(false)}>{t('common.cancel')}</Button><Button variant="destructive" onClick={remove} disabled={deleteName !== repo.name || deleting}>{deleting ? t('common.processing') : t('repos.detail.delete')}</Button></>}>
        <label className="mb-2 block text-sm" htmlFor="delete-repo-name">{t('repos.detail.typeNameToDelete', { name: repo.name })}</label>
        <Input id="delete-repo-name" value={deleteName} onChange={(event) => setDeleteName(event.target.value)} autoComplete="off" autoFocus />
      </Modal>
    </div>
  );
}

function SettingToggle({ label, checked, onChange }: { label: string; checked: boolean; onChange: (next: boolean) => void }) {
  return <label className="flex items-center justify-between gap-4 rounded-[8px] border border-border p-3 text-sm font-medium"><span>{label}</span><input type="checkbox" checked={checked} onChange={(event) => onChange(event.target.checked)} className="h-5 w-5 accent-[var(--primary)]" /></label>;
}
