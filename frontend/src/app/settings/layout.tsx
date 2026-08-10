'use client';

import type { ReactNode } from 'react';
import { Outlet } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { Link, usePathname } from '@/lib/router';
import { usePermission } from '@/hooks/usePermission';
import {
  canAccessSettingsSection,
  settingsNavigationFor,
  type SettingsSection,
} from '@/lib/settings-policy';

export function SettingsLayout() {
  const { t } = useTranslation();
  const pathname = usePathname();
  const { tenantRole } = usePermission();
  const navigation = settingsNavigationFor(tenantRole);

  return (
    <div className="mx-auto w-full max-w-7xl">
      <div className="border-b border-border pb-4">
        <h1 className="text-2xl font-semibold text-foreground">{t('settings.title')}</h1>
        <p className="mt-1 text-sm text-muted-foreground">{t('settings.subtitle')}</p>
      </div>

      <div className="grid min-w-0 gap-6 py-4 md:grid-cols-[13rem_minmax(0,1fr)] md:py-6">
        <nav
          aria-label={t('settings.navigationLabel')}
          className="flex gap-1 overflow-x-auto border-b border-border pb-3 md:block md:space-y-1 md:overflow-visible md:border-b-0 md:border-r md:pb-0 md:pr-4"
        >
          {navigation.map((item) => {
            const isActive = pathname === item.href;
            return (
              <Link
                key={item.section}
                to={item.href}
                aria-current={isActive ? 'page' : undefined}
                className={`shrink-0 rounded-md px-3 py-2 text-sm font-medium transition-colors md:block ${
                  isActive
                    ? 'bg-primary text-primary-foreground'
                    : 'text-muted-foreground hover:bg-muted hover:text-foreground'
                }`}
              >
                {t(item.labelKey)}
              </Link>
            );
          })}
        </nav>

        <div className="min-w-0">
          <Outlet />
        </div>
      </div>
    </div>
  );
}

export function SettingsAccessGuard({
  section,
  children,
}: {
  section: SettingsSection;
  children: ReactNode;
}) {
  const { t } = useTranslation();
  const { tenantRole } = usePermission();

  if (!canAccessSettingsSection(tenantRole, section)) {
    return (
      <section
        aria-labelledby="settings-forbidden-title"
        className="rounded-lg border border-border bg-card p-6"
      >
        <p className="text-sm font-medium text-destructive">403</p>
        <h2 id="settings-forbidden-title" className="mt-2 text-2xl font-semibold text-foreground">
          {t('settings.forbidden.title')}
        </h2>
        <p className="mt-2 max-w-xl text-sm text-muted-foreground">
          {t('settings.forbidden.description')}
        </p>
        <Link
          to="/settings/profile"
          className="mt-5 inline-flex rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:opacity-90 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
        >
          {t('settings.forbidden.action')}
        </Link>
      </section>
    );
  }

  return <>{children}</>;
}
