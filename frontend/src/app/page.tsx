'use client';

import { Link } from '@/lib/router';
import { useTranslation } from 'react-i18next';
import { Button } from '@/components/ui/Button';
import { LanguageSwitcher } from '@/components/LanguageSwitcher';

export default function LandingPage() {
  const { t } = useTranslation();

  // Always render the full page - don't show loading state
  // This prevents the blank "Evolith" flash on first load
  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900">
      {/* Navigation */}
      <nav className="fixed top-0 left-0 right-0 z-50 border-b border-white/10 bg-black/20 backdrop-blur-lg">
        <div className="mx-auto flex h-16 max-w-7xl items-center justify-between px-6">
          <Link to="/"className="flex items-center gap-2"><div className="flex h-9 w-9 items-center justify-center rounded-lg bg-gradient-to-br from-purple-500 to-pink-500">
            <span className="text-lg font-bold text-white">E</span>
          </div>
          <span className="text-xl font-bold text-white">Evolith</span></Link>
          
          <div className="flex items-center gap-6">
            <LanguageSwitcher />
            <Link to="/login"><Button variant="ghost" className="text-white hover:bg-white/10">
              {t('auth.login')}
            </Button></Link>
            <Link to="/register"><Button className="bg-gradient-to-r from-purple-500 to-pink-500 hover:from-purple-600 hover:to-pink-600">
              {t('auth.register')}
            </Button></Link>
          </div>
        </div>
      </nav>

      {/* Hero Section */}
      <section className="relative overflow-hidden pt-32 pb-20">
        <div className="absolute inset-0 overflow-hidden">
          <div className="absolute -top-40 -right-40 h-80 w-80 rounded-full bg-purple-500/20 blur-3xl" />
          <div className="absolute -bottom-40 -left-40 h-80 w-80 rounded-full bg-pink-500/20 blur-3xl" />
        </div>

        <div className="relative mx-auto max-w-7xl px-6">
          <div className="text-center">
            <h1 className="bg-gradient-to-r from-white via-purple-200 to-pink-200 bg-clip-text text-5xl font-bold tracking-tight text-transparent sm:text-7xl">
              {t('landing.hero.title')}
            </h1>
            <p className="mx-auto mt-6 max-w-2xl text-xl text-purple-200/80">
              {t('landing.hero.subtitle')}
            </p>
            <div className="mt-10 flex items-center justify-center gap-4">
              <Link to="/register"><Button size="lg" className="bg-gradient-to-r from-purple-500 to-pink-500 px-8 hover:from-purple-600 hover:to-pink-600">
                {t('landing.hero.getStarted')}
              </Button></Link>
              <Link to="/docs"><Button variant="outline" size="lg" className="border-white/20 text-white hover:bg-white/10">
                {t('landing.hero.learnMore')}
              </Button></Link>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-20">
        <div className="mx-auto max-w-7xl px-6">
          <div className="text-center">
            <h2 className="text-3xl font-bold text-white sm:text-4xl">
              {t('landing.features.title')}
            </h2>
            <p className="mt-4 text-lg text-purple-200/70">
              {t('landing.features.subtitle')}
            </p>
          </div>

          <div className="mt-16 grid gap-8 md:grid-cols-3">
            <Link to="/tools"className="group block rounded-2xl border border-white/10 bg-white/5 p-8 backdrop-blur-sm transition-all hover:border-purple-500/50 hover:bg-white/10"><div className="flex h-12 w-12 items-center justify-center rounded-lg bg-gradient-to-br from-purple-500/20 to-pink-500/20 text-purple-400">
              <ToolsIcon />
            </div>
            <h3 className="mt-4 text-xl font-semibold text-white">{t('landing.features.tools.title')}</h3>
            <p className="mt-2 text-purple-200/60">{t('landing.features.tools.description')}</p></Link>

            <Link to="/skills"className="group block rounded-2xl border border-white/10 bg-white/5 p-8 backdrop-blur-sm transition-all hover:border-purple-500/50 hover:bg-white/10"><div className="flex h-12 w-12 items-center justify-center rounded-lg bg-gradient-to-br from-purple-500/20 to-pink-500/20 text-purple-400">
              <SkillsIcon />
            </div>
            <h3 className="mt-4 text-xl font-semibold text-white">{t('landing.features.skills.title')}</h3>
            <p className="mt-2 text-purple-200/60">{t('landing.features.skills.description')}</p></Link>

            <Link to="/interfaces"className="group block rounded-2xl border border-white/10 bg-white/5 p-8 backdrop-blur-sm transition-all hover:border-purple-500/50 hover:bg-white/10"><div className="flex h-12 w-12 items-center justify-center rounded-lg bg-gradient-to-br from-purple-500/20 to-pink-500/20 text-purple-400">
              <InterfacesIcon />
            </div>
            <h3 className="mt-4 text-xl font-semibold text-white">{t('landing.features.interfaces.title')}</h3>
            <p className="mt-2 text-purple-200/60">{t('landing.features.interfaces.description')}</p></Link>
          </div>
        </div>
      </section>

      {/* Stats Section */}
      <section className="border-y border-white/10 bg-black/20 py-16">
        <div className="mx-auto max-w-7xl px-6">
          <div className="grid gap-8 text-center md:grid-cols-4">
            <div>
              <div className="text-4xl font-bold text-white">Git-Compatible</div>
              <div className="mt-1 text-sm text-purple-200/60">Zero migration cost</div>
            </div>
            <div>
              <div className="text-4xl font-bold text-white">MCP-Native</div>
              <div className="mt-1 text-sm text-purple-200/60">Industry-standard protocol</div>
            </div>
            <div>
              <div className="text-4xl font-bold text-white">Enterprise-Ready</div>
              <div className="mt-1 text-sm text-purple-200/60">RBAC · Audit · Multi-tenant</div>
            </div>
            <div>
              <div className="text-4xl font-bold text-white">Open Format</div>
              <div className="mt-1 text-sm text-purple-200/60">Claude Skills compatible</div>
            </div>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-20">
        <div className="mx-auto max-w-4xl px-6 text-center">
          <h2 className="text-3xl font-bold text-white sm:text-4xl">
            {t('landing.cta.title')}
          </h2>
          <p className="mt-4 text-lg text-purple-200/70">
            {t('landing.cta.subtitle')}
          </p>
          <div className="mt-8">
            <Link to="/register"><Button size="lg" className="bg-gradient-to-r from-purple-500 to-pink-500 px-10 hover:from-purple-600 hover:to-pink-600">
              {t('landing.cta.button')}
            </Button></Link>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="border-t border-white/10 bg-black/30 py-12">
        <div className="mx-auto max-w-7xl px-6">
          <div className="flex flex-col items-center justify-between gap-4 md:flex-row">
            <div className="flex items-center gap-2">
              <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-gradient-to-br from-purple-500 to-pink-500">
                <span className="text-sm font-bold text-white">E</span>
              </div>
              <span className="text-lg font-semibold text-white">Evolith</span>
            </div>
            <div className="flex gap-6 text-sm text-purple-200/60">
              <Link to="/docs"className="hover:text-white">{t('landing.footer.docs')}</Link>
              <Link to="/privacy"className="hover:text-white">{t('landing.footer.privacy')}</Link>
              <Link to="/terms"className="hover:text-white">{t('landing.footer.terms')}</Link>
            </div>
            <p className="text-sm text-purple-200/40">
              © 2026 Evolith. All rights reserved.
            </p>
          </div>
        </div>
      </footer>
    </div>
  );
}

function ToolsIcon() {
  return (
    <svg className="h-6 w-6" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" d="M11.42 15.17L17.25 21A2.652 2.652 0 0021 17.25l-5.877-5.877M11.42 15.17l2.496-3.03c.317-.384.74-.626 1.208-.766M11.42 15.17l-4.655 5.653a2.548 2.548 0 11-3.586-3.586l6.837-5.63m5.108-.233c.55-.164 1.163-.188 1.743-.14a4.5 4.5 0 004.486-6.336l-3.276 3.277a3.004 3.004 0 01-2.25-2.25l3.276-3.276a4.5 4.5 0 00-6.336 4.486c.091 1.076-.071 2.264-.904 2.95l-.102.085m-1.745 1.437L5.909 7.5H4.5L2.25 3.75l1.5-1.5L7.5 4.5v1.409l4.26 4.26m-1.745 1.437l1.745-1.437m6.615 8.206L15.75 15.75M4.867 19.125h.008v.008h-.008v-.008z" />
    </svg>
  );
}

function SkillsIcon() {
  return (
    <svg className="h-6 w-6" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" d="M9.813 15.904L9 18.75l-.813-2.846a4.5 4.5 0 00-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 003.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 003.09 3.09L15.75 12l-.846.813a4.5 4.5 0 00-3.09 3.09L18.75 18l.813-2.846a4.5 4.5 0 00-3.09-3.09L15 9.75l-.813 2.846z" />
    </svg>
  );
}

function InterfacesIcon() {
  return (
    <svg className="h-6 w-6" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" d="M17.25 6.75L22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3l-4.5 16.5" />
    </svg>
  );
}