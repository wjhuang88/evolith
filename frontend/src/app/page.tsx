'use client';

import { useCallback, useEffect, useState } from 'react';
import { Link, useRouter } from '@/lib/router';
import { useTranslation } from 'react-i18next';
import { Button } from '@/components/ui/Button';
import { LanguageSwitcher } from '@/components/LanguageSwitcher';
import { getToken, parseApiError } from '@/lib/api';
import { resolveAuthenticatedEntry } from '@/lib/entry';
import { useAuthStore } from '@/stores/authStore';

type EntryState = 'public' | 'resolving' | 'error' | 'forbidden' | 'missing-tenant';

export default function RootEntryPage() {
  const { t } = useTranslation();
  const router = useRouter();
  const fetchUser = useAuthStore((state) => state.fetchUser);
  const [entryState, setEntryState] = useState<EntryState>(() => getToken() ? 'resolving' : 'public');

  const resolveEntry = useCallback(async () => {
    if (!getToken()) {
      setEntryState('public');
      return;
    }

    setEntryState('resolving');
    let auth = useAuthStore.getState();
    if (!auth.user) {
      await fetchUser();
      auth = useAuthStore.getState();
    }
    if (!getToken() || !auth.isAuthenticated) {
      setEntryState('public');
      return;
    }
    if (!auth.tenant?.id) {
      setEntryState('missing-tenant');
      return;
    }

    try {
      router.replace(await resolveAuthenticatedEntry(auth.tenant.id));
    } catch (error) {
      const code = parseApiError(error).code;
      setEntryState(code === 'HTTP_403' || code === 'FORBIDDEN' ? 'forbidden' : 'error');
    }
  }, [fetchUser, router]);

  useEffect(() => {
    void resolveEntry();
  }, [resolveEntry]);

  if (entryState === 'public') return <PublicLanding />;

  return (
    <main className="flex min-h-screen items-center justify-center bg-[#f7f5ef] px-5 text-[#151515]">
      <section className="w-full max-w-lg rounded-[24px] border-2 border-[#151515] bg-white p-8 shadow-[8px_8px_0_#151515]">
        <p className="text-xs font-bold uppercase tracking-[0.2em]">Evolith / entry</p>
        <h1 className="mt-4 text-3xl font-black tracking-tight">
          {entryState === 'resolving' ? t('auth.entry.loadingTitle') : t(`auth.entry.${entryState}Title`)}
        </h1>
        <p className="mt-3 text-muted-foreground">
          {entryState === 'resolving' ? t('auth.entry.loadingDescription') : t(`auth.entry.${entryState}Description`)}
        </p>
        {entryState !== 'resolving' && (
          <div className="mt-6 flex flex-wrap gap-3">
            <Button type="button" onClick={resolveEntry}>{t('common.retry')}</Button>
            <Link to="/login"><Button variant="outline">{t('auth.login')}</Button></Link>
          </div>
        )}
      </section>
    </main>
  );
}

function PublicLanding() {
  const { t } = useTranslation();
  const statuses = [
    { key: 'available', tone: 'bg-[#dceeb1]', tag: 'available' },
    { key: 'development', tone: 'bg-[#c5b0f4]', tag: 'development' },
    { key: 'planned', tone: 'bg-[#f4c3d8]', tag: 'planned' },
  ] as const;

  return (
    <div className="min-h-screen overflow-x-hidden bg-[#f7f5ef] text-[#151515]">
      <nav className="border-b-2 border-[#151515] bg-[#f7f5ef]">
        <div className="mx-auto flex h-20 max-w-7xl items-center justify-between px-5 sm:px-8">
          <Link to="/" className="flex items-center gap-3 font-black tracking-tight">
            <span className="flex h-10 w-10 items-center justify-center rounded-full border-2 border-[#151515] bg-[#dceeb1]">E</span>
            <span className="text-xl">Evolith</span>
          </Link>
          <div className="flex items-center gap-2 sm:gap-4">
            <LanguageSwitcher />
            <Link to="/login" className="hidden text-sm font-bold underline-offset-4 hover:underline sm:block">{t('auth.login')}</Link>
            <Link to="/register"><Button className="rounded-full border-2 border-[#151515] bg-[#151515] px-5 text-white hover:bg-[#333]">{t('auth.register')}</Button></Link>
          </div>
        </div>
      </nav>

      <main>
        <section className="mx-auto grid max-w-7xl gap-10 px-5 py-16 sm:px-8 lg:grid-cols-[1.05fr_0.95fr] lg:items-center lg:py-24">
          <div>
            <p className="inline-flex rounded-full border-2 border-[#151515] bg-[#dceeb1] px-4 py-2 text-xs font-black uppercase tracking-[0.16em]">
              {t('landing.hero.eyebrow')}
            </p>
            <h1 className="mt-7 max-w-3xl text-5xl font-black leading-[0.96] tracking-[-0.055em] sm:text-7xl">
              {t('landing.hero.title')}
            </h1>
            <p className="mt-7 max-w-2xl text-lg leading-8 text-[#464646] sm:text-xl">
              {t('landing.hero.subtitle')}
            </p>
            <div className="mt-9 flex flex-wrap gap-3">
              <Link to="/register"><Button size="lg" className="rounded-full border-2 border-[#151515] bg-[#151515] px-7 text-white hover:bg-[#333]">{t('landing.hero.getStarted')}</Button></Link>
              <a href="#product-status"><Button variant="outline" size="lg" className="rounded-full border-2 border-[#151515] bg-transparent px-7 hover:bg-white">{t('landing.hero.learnMore')}</Button></a>
            </div>
          </div>

          <div className="relative mx-auto w-full max-w-xl">
            <div className="absolute -left-5 -top-5 h-full w-full rounded-[28px] border-2 border-[#151515] bg-[#c5b0f4]" />
            <div className="relative rounded-[28px] border-2 border-[#151515] bg-white p-5 shadow-[10px_10px_0_#151515] sm:p-7">
              <div className="flex items-center justify-between border-b-2 border-[#151515] pb-4">
                <div>
                  <p className="text-xs font-black uppercase tracking-[0.15em]">repo / evolith</p>
                  <p className="mt-1 font-mono text-sm">main · 8f31c2a</p>
                </div>
                <span className="rounded-full border-2 border-[#151515] bg-[#dceeb1] px-3 py-1 text-xs font-bold">{t('landing.demo.verified')}</span>
              </div>
              <div className="py-6">
                <p className="text-sm text-[#666]">{t('landing.demo.commit')}</p>
                <h2 className="mt-2 text-2xl font-black">feat(repo): add commit evidence view</h2>
                <div className="mt-5 grid grid-cols-3 gap-2 text-center text-sm font-bold">
                  <div className="rounded-xl border-2 border-[#151515] bg-[#dceeb1] px-2 py-3">+84</div>
                  <div className="rounded-xl border-2 border-[#151515] bg-[#f4c3d8] px-2 py-3">−12</div>
                  <div className="rounded-xl border-2 border-[#151515] bg-[#b9ddf2] px-2 py-3">3 files</div>
                </div>
              </div>
              <div className="rounded-2xl bg-[#151515] p-4 font-mono text-xs leading-6 text-white">
                <p><span className="text-[#dceeb1]">+</span> commit identity and metadata</p>
                <p><span className="text-[#dceeb1]">+</span> parent and branch context</p>
                <p><span className="text-[#dceeb1]">+</span> changed-file patch evidence</p>
              </div>
            </div>
          </div>
        </section>

        <section id="product-status" className="border-y-2 border-[#151515] bg-white py-16 sm:py-20">
          <div className="mx-auto max-w-7xl px-5 sm:px-8">
            <div className="max-w-3xl">
              <p className="text-xs font-black uppercase tracking-[0.18em]">{t('landing.status.eyebrow')}</p>
              <h2 className="mt-4 text-4xl font-black tracking-tight sm:text-5xl">{t('landing.status.title')}</h2>
              <p className="mt-4 text-lg text-[#555]">{t('landing.status.subtitle')}</p>
            </div>
            <div className="mt-10 grid gap-5 lg:grid-cols-3">
              {statuses.map(({ key, tone, tag }) => (
                <article key={key} className={`rounded-[24px] border-2 border-[#151515] p-6 ${tone}`}>
                  <span className="rounded-full border-2 border-[#151515] bg-white px-3 py-1 text-xs font-black uppercase tracking-wide">{t(`landing.status.tags.${tag}`)}</span>
                  <h3 className="mt-7 text-2xl font-black">{t(`landing.status.${key}.title`)}</h3>
                  <p className="mt-3 leading-7 text-[#3f3f3f]">{t(`landing.status.${key}.description`)}</p>
                </article>
              ))}
            </div>
          </div>
        </section>

        <section className="mx-auto max-w-5xl px-5 py-16 text-center sm:px-8 sm:py-24">
          <h2 className="text-4xl font-black tracking-tight sm:text-6xl">{t('landing.cta.title')}</h2>
          <p className="mx-auto mt-5 max-w-2xl text-lg text-[#555]">{t('landing.cta.subtitle')}</p>
          <div className="mt-8 flex flex-wrap justify-center gap-3">
            <Link to="/register"><Button size="lg" className="rounded-full border-2 border-[#151515] bg-[#151515] px-8 text-white hover:bg-[#333]">{t('landing.cta.button')}</Button></Link>
            <Link to="/login"><Button variant="outline" size="lg" className="rounded-full border-2 border-[#151515] bg-transparent px-8 hover:bg-white">{t('auth.login')}</Button></Link>
          </div>
        </section>
      </main>

      <footer className="border-t-2 border-[#151515] bg-[#dceeb1] py-8">
        <div className="mx-auto flex max-w-7xl flex-col gap-3 px-5 text-sm sm:flex-row sm:items-center sm:justify-between sm:px-8">
          <span className="font-black">Evolith</span>
          <span>{t('landing.footer.status')}</span>
          <span>© 2026 Evolith</span>
        </div>
      </footer>
    </div>
  );
}
