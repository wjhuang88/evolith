'use client';

import { Link, usePathname, useRouter } from '@/lib/router';
import { useState } from 'react';
import { ThemeToggle } from '@/components/ui/ThemeToggle';
import { useAuthStore } from '@/stores';
import { useTranslation } from 'react-i18next';
import { LanguageSwitcher } from '@/components/LanguageSwitcher';

interface NavItem {
  name: string;
  href: string;
  icon: (props: { className?: string }) => React.ReactElement;
}

const primaryNavigation: NavItem[] = [
  { name: 'repos', href: '/repos', icon: ReposIcon },
  { name: 'dashboard', href: '/dashboard', icon: DashboardIcon },
];

const legacyNavigation: NavItem[] = [
  { name: 'tools', href: '/tools', icon: ToolsIcon },
  { name: 'skills', href: '/skills', icon: SkillsIcon },
  { name: 'interfaces', href: '/interfaces', icon: InterfacesIcon },
];

export function Header() {
  return (
    <header className="sticky top-0 z-40 flex h-14 w-full items-center justify-between border-b border-border bg-background px-4 md:px-6">
      <div className="flex min-w-0 items-center gap-1 sm:gap-4">
        <MobileNav />
        <Link to="/" className="flex items-center gap-2">
          <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-[var(--primary)]">
            <span className="text-white font-bold text-sm">E</span>
          </div>
          <span className="hidden text-xl font-semibold text-foreground min-[480px]:inline">Evolith</span>
        </Link>
      </div>

      <div className="flex shrink-0 items-center gap-1 sm:gap-4">
        <LanguageSwitcher />
        <ThemeToggle />
        <UserMenu />
      </div>
    </header>
  );
}

function UserMenu() {
  const { t } = useTranslation();
  const [isOpen, setIsOpen] = useState(false);
  const router = useRouter();
  const { user, isAuthenticated, logout } = useAuthStore();

  const handleLogout = async () => {
    await logout();
    router.push('/login');
  };

  const getInitials = (name?: string) => {
    if (!name) return 'U';
    return name.charAt(0).toUpperCase();
  };

  if (!isAuthenticated) {
    return (
      <Link
        to="/login"
        className="rounded-[50px] border border-border bg-[var(--secondary)] px-4 py-2 text-sm font-medium text-[var(--secondary-foreground)] hover:opacity-90"
      >
        {t('nav.signIn')}
      </Link>
    );
  }

  return (
    <div className="relative">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="flex items-center gap-2 rounded-[50px] border border-border bg-muted px-3 py-2 text-sm hover:bg-muted/80"
      >
        <div className="flex h-6 w-6 items-center justify-center rounded-full bg-[var(--muted)]">
          <span className="text-xs font-medium text-[var(--foreground)]">
            {getInitials(user?.username)}
          </span>
        </div>
        <span className="hidden text-foreground sm:inline">{user?.username || t('nav.user')}</span>
      </button>

      {isOpen && (
        <div className="absolute right-0 mt-2 w-48 rounded-[8px] border border-border bg-card py-1 shadow-lg">
          <Link
            to="/settings/profile"
            className="block px-4 py-2 text-sm text-foreground hover:bg-muted"
            onClick={() => setIsOpen(false)}
          >
            {t('nav.settings')}
          </Link>
          <hr className="my-1 border-border" />
          <button
            onClick={handleLogout}
            className="block w-full px-4 py-2 text-left text-sm text-error hover:bg-muted"
          >
            {t('nav.signOut')}
          </button>
        </div>
      )}
    </div>
  );
}

function NavLink({
  item,
  isActive,
  variant = 'primary',
}: {
  item: NavItem;
  isActive: boolean;
  variant?: 'primary' | 'legacy';
}) {
  const { t } = useTranslation();
  const baseClasses =
    'flex items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium transition-colors';
  const primaryClasses = isActive
    ? 'bg-[var(--primary)] text-[var(--primary-foreground)]'
    : 'text-foreground hover:bg-muted/80';
  const legacyClasses = isActive
    ? 'bg-muted text-foreground'
    : 'text-muted-foreground hover:bg-muted/60 hover:text-foreground';
  return (
    <Link
      to={item.href}
      className={`${baseClasses} ${variant === 'legacy' ? legacyClasses : primaryClasses}`}
      aria-current={isActive ? 'page' : undefined}
    >
      <item.icon className="h-5 w-5 shrink-0" />
      <span>{t(`nav.${item.name}`)}</span>
    </Link>
  );
}

function LegacyDisclosure({
  pathname,
  isOpen,
  onToggle,
}: {
  pathname: string;
  isOpen: boolean;
  onToggle: () => void;
}) {
  const { t } = useTranslation();
  const anyActive = legacyNavigation.some(
    (item) =>
      pathname === item.href ||
      (item.href !== '/' && pathname.startsWith(`${item.href}/`))
  );
  return (
    <div className="mt-2">
      <button
        type="button"
        onClick={onToggle}
        aria-expanded={isOpen}
        className={`flex w-full items-center justify-between rounded-lg px-3 py-2 text-xs font-semibold uppercase tracking-wider ${
          anyActive
            ? 'text-foreground'
            : 'text-muted-foreground hover:text-foreground'
        }`}
      >
        <span>{t('nav.legacy')}</span>
        <ChevronIcon className={`h-4 w-4 transition-transform ${isOpen ? 'rotate-90' : ''}`} />
      </button>
      {isOpen && (
        <nav className="mt-1 space-y-1">
          {legacyNavigation.map((item) => {
            const isActive =
              pathname === item.href ||
              (item.href !== '/' && pathname.startsWith(`${item.href}/`));
            return (
              <NavLink key={item.name} item={item} isActive={isActive} variant="legacy" />
            );
          })}
        </nav>
      )}
    </div>
  );
}

export function Sidebar() {
  const { t } = useTranslation();
  const pathname = usePathname();
  const [legacyOpen, setLegacyOpen] = useState(false);
  const isOnLegacy = legacyNavigation.some(
    (item) =>
      pathname === item.href || (item.href !== '/' && pathname.startsWith(`${item.href}/`))
  );

  return (
    <aside className="hidden w-64 flex-col border-r border-border bg-muted md:flex">
      <nav className="flex-1 space-y-1 px-3 py-4" aria-label={t('nav.dashboard')}>
        {primaryNavigation.map((item) => {
          const isActive =
            pathname === item.href ||
            (item.href !== '/' && pathname.startsWith(`${item.href}/`));
          return <NavLink key={item.name} item={item} isActive={isActive} />;
        })}
        <LegacyDisclosure
          pathname={pathname}
          isOpen={legacyOpen || isOnLegacy}
          onToggle={() => setLegacyOpen((open) => !open)}
        />
      </nav>
    </aside>
  );
}

export function MobileNav() {
  const { t } = useTranslation();
  const pathname = usePathname();
  const [isOpen, setIsOpen] = useState(false);
  const [legacyOpen, setLegacyOpen] = useState(false);

  const close = () => setIsOpen(false);

  return (
    <div className="md:hidden">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="p-2 text-muted-foreground hover:bg-muted"
        aria-label={t('nav.dashboard')}
      >
        <MenuIcon className="h-6 w-6" />
      </button>

      {isOpen && (
        <div className="absolute left-0 top-16 z-50 w-full border-b border-border bg-card py-4 shadow-lg">
          <nav className="space-y-1 px-4">
            {primaryNavigation.map((item) => {
              const isActive =
                pathname === item.href ||
                (item.href !== '/' && pathname.startsWith(`${item.href}/`));
              return (
                <Link
                  key={item.name}
                  to={item.href}
                  onClick={close}
                  className={`block rounded-lg px-3 py-2 text-base font-medium ${
                    isActive
                      ? 'bg-[var(--primary)] text-[var(--primary-foreground)]'
                      : 'text-foreground hover:bg-muted'
                  }`}
                >
                  {t(`nav.${item.name}`)}
                </Link>
              );
            })}
            <div className="mt-2 border-t border-border pt-2">
              <button
                type="button"
                onClick={() => setLegacyOpen((open) => !open)}
                aria-expanded={legacyOpen}
                className="flex w-full items-center justify-between rounded-lg px-3 py-2 text-xs font-semibold uppercase tracking-wider text-muted-foreground"
              >
                <span>{t('nav.legacy')}</span>
                <ChevronIcon className={`h-4 w-4 transition-transform ${legacyOpen ? 'rotate-90' : ''}`} />
              </button>
              {legacyOpen && (
                <div className="mt-1 space-y-1">
                  {legacyNavigation.map((item) => {
                    const isActive = pathname === item.href;
                    return (
                      <Link
                        key={item.name}
                        to={item.href}
                        onClick={close}
                        className={`block rounded-lg px-3 py-2 text-base font-medium ${
                          isActive
                            ? 'bg-[var(--primary)] text-[var(--primary-foreground)]'
                            : 'text-foreground hover:bg-muted'
                        }`}
                      >
                        {t(`nav.${item.name}`)}
                      </Link>
                    );
                  })}
                </div>
              )}
            </div>
          </nav>
        </div>
      )}
    </div>
  );
}

function ReposIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" aria-hidden>
      <path strokeLinecap="round" strokeLinejoin="round" d="M2.25 12a8.25 8.25 0 108.25 8.25M2.25 12a8.25 8.25 0 0114.59-5.28M2.25 12H6m16.5 0a8.25 8.25 0 01-8.25 8.25m8.25-8.25a8.25 8.25 0 00-5.28-7.591M22.5 12h-3.75M14.59 6.72l-2.117 2.116M14.59 17.28l-2.117-2.116" />
    </svg>
  );
}

function DashboardIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" aria-hidden>
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25a2.25 2.25 0 01-2.25 2.25h-2.25A2.25 2.25 0 0113.5 6V6zm0 9.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25h-2.25A2.25 2.25 0 0113.5 15.75v-2.25z"
      />
    </svg>
  );
}

function ToolsIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" aria-hidden>
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M11.42 15.17L17.25 21A2.652 2.652 0 0021 17.25l-5.877-5.877M11.42 15.17l2.496-3.03c.317-.384.74-.626 1.208-.766M11.42 15.17l-4.655 5.653a2.548 2.548 0 11-3.586-3.586l6.837-5.63m5.108-.233c.55-.164 1.163-.188 1.743-.14a4.5 4.5 0 004.486-6.336l-3.276 3.277a3.004 3.004 0 01-2.25-2.25l3.276-3.276a4.5 4.5 0 00-6.336 4.486c.091 1.076-.071 2.264-.904 2.95l-.102.085m-1.745 1.437L5.909 7.5H4.5L2.25 3.75l1.5-1.5L7.5 4.5v1.409l4.26 4.26m-1.745 1.437l1.745-1.437m6.615 8.206L15.75 15.75M4.867 19.125h.008v.008h-.008v-.008z"
      />
    </svg>
  );
}

function SkillsIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" aria-hidden>
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M9.813 15.904L9 18.75l-.813-2.846a4.5 4.5 0 00-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 003.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 003.09 3.09L15.75 12l-.846.813a4.5 4.5 0 00-3.09 3.09L18.75 18l.813-2.846a4.5 4.5 0 00-3.09-3.09L15 9.75l-.813 2.846z"
      />
    </svg>
  );
}

function InterfacesIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" aria-hidden>
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M17.25 6.75L22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3l-4.5 16.5"
      />
    </svg>
  );
}

function MenuIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" aria-hidden>
      <path strokeLinecap="round" strokeLinejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
    </svg>
  );
}

function ChevronIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" aria-hidden>
      <path strokeLinecap="round" strokeLinejoin="round" d="M8.25 4.5l7.5 7.5-7.5 7.5" />
    </svg>
  );
}
