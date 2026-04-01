'use client';

import Link from 'next/link';
import { usePathname, useRouter } from 'next/navigation';
import { useState, useEffect } from 'react';
import { ThemeToggle } from '@/components/ui/ThemeToggle';
import { useAuthStore } from '@/stores';
import { useTranslation } from 'react-i18next';
import { LanguageSwitcher } from '@/components/LanguageSwitcher';

const navigation = [
  { name: 'Dashboard', href: '/dashboard', icon: DashboardIcon },
  { name: 'Tools', href: '/tools', icon: ToolsIcon },
  { name: 'Skills', href: '/skills', icon: SkillsIcon },
  { name: 'Snippets', href: '/snippets', icon: SnippetsIcon },
]; // name is used as key, display localized in JSX

export function Header() {
  const { t } = useTranslation();
  return (
    <header className="sticky top-0 z-40 flex h-16 w-full items-center justify-between border-b border-border bg-background px-4 md:px-6 shadow-sm">
      <div className="flex items-center gap-4">
        <MobileNav />
        <Link href="/" className="flex items-center gap-2">
          <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-gradient-to-br from-primary-500 to-purple-600">
            <span className="text-white font-bold text-sm">E</span>
          </div>
          <span className="text-xl font-semibold text-foreground">Evolith</span>
        </Link>
      </div>
      
      <div className="flex items-center gap-4">
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
        href="/login"
        className="rounded-full border border-border bg-muted px-4 py-2 text-sm font-medium hover:bg-muted/80"
      >
        {t('nav.signIn')}
      </Link>
    );
  }

  return (
    <div className="relative">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="flex items-center gap-2 rounded-full border border-border bg-muted px-3 py-2 text-sm hover:bg-muted/80"
      >
        <div className="flex h-6 w-6 items-center justify-center rounded-full bg-primary-100 dark:bg-primary-900">
          <span className="text-xs font-medium text-primary-600 dark:text-primary-400">
            {getInitials(user?.username)}
          </span>
        </div>
        <span className="text-foreground">{user?.username || t('nav.user')}</span>
      </button>
      
      {isOpen && (
        <div className="absolute right-0 mt-2 w-48 rounded-md border border-border bg-card py-1 shadow-lg">
          <Link
            href="/profile"
            className="block px-4 py-2 text-sm text-foreground hover:bg-muted"
            onClick={() => setIsOpen(false)}
          >
            {t('nav.profile')}
          </Link>
          <Link
            href="/settings"
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

export function Sidebar() {
  const { t } = useTranslation();
  const pathname = usePathname();
  
  return (
    <aside className="hidden w-64 flex-col border-r border-border bg-muted md:flex">
      <nav className="flex-1 space-y-1 px-3 py-4">
{navigation.map((item) => {
  const isActive = pathname === item.href || (item.href !== '/' && pathname.startsWith(item.href));
  return (
    <Link
      key={item.name}
      href={item.href}
      className={`flex items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium transition-colors ${
        isActive
          ? 'bg-primary-50 text-primary-600 dark:bg-primary-900/50 dark:text-primary-400'
          : 'text-foreground hover:bg-muted/80'
      }`}
    >
      <item.icon className="h-5 w-5" />
      {t('nav.' + item.name.toLowerCase())}
    </Link>
  );
})}
      </nav>
    </aside>
  );
}

export function MobileNav() {
  const { t } = useTranslation();
  const pathname = usePathname();
  const [isOpen, setIsOpen] = useState(false);
  
  return (
    <div className="md:hidden">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="p-2 text-muted-foreground hover:bg-muted"
      >
        <MenuIcon className="h-6 w-6" />
      </button>
      
      {isOpen && (
        <div className="absolute left-0 top-16 z-50 w-full border-b border-border bg-card py-4 shadow-lg">
          <nav className="space-y-1 px-4">
{navigation.map((item) => {
  const isActive = pathname === item.href;
  return (
    <Link
      key={item.name}
      href={item.href}
      onClick={() => setIsOpen(false)}
      className={`block rounded-lg px-3 py-2 text-base font-medium ${
        isActive
          ? 'bg-primary-50 text-primary-600 dark:bg-primary-900/50 dark:text-primary-400'
          : 'text-foreground hover:bg-muted'
      }`}
    >
      {t('nav.' + item.name.toLowerCase())}
    </Link>
  );
})}
          </nav>
        </div>
      )}
    </div>
  );
}

// Icons
function DashboardIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" d="M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25a2.25 2.25 0 01-2.25 2.25h-2.25A2.25 2.25 0 0113.5 6V6zm0 9.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25h-2.25A2.25 2.25 0 0113.5 15.75v-2.25z" />
    </svg>
  );
}

function ToolsIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" d="M11.42 15.17L17.25 21A2.652 2.652 0 0021 17.25l-5.877-5.877M11.42 15.17l2.496-3.03c.317-.384.74-.626 1.208-.766M11.42 15.17l-4.655 5.653a2.548 2.548 0 11-3.586-3.586l6.837-5.63m5.108-.233c.55-.164 1.163-.188 1.743-.14a4.5 4.5 0 004.486-6.336l-3.276 3.277a3.004 3.004 0 01-2.25-2.25l3.276-3.276a4.5 4.5 0 00-6.336 4.486c.091 1.076-.071 2.264-.904 2.95l-.102.085m-1.745 1.437L5.909 7.5H4.5L2.25 3.75l1.5-1.5L7.5 4.5v1.409l4.26 4.26m-1.745 1.437l1.745-1.437m6.615 8.206L15.75 15.75M4.867 19.125h.008v.008h-.008v-.008z" />
    </svg>
  );
}

function SkillsIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" d="M9.813 15.904L9 18.75l-.813-2.846a4.5 4.5 0 00-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 003.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 003.09 3.09L15.75 12l-.846.813a4.5 4.5 0 00-3.09 3.09L18.75 18l.813-2.846a4.5 4.5 0 00-3.09-3.09L15 9.75l-.813 2.846z" />
    </svg>
  );
}

function SnippetsIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" d="M17.25 6.75L22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3l-4.5 16.5" />
    </svg>
  );
}

function MenuIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
    </svg>
  );
}
