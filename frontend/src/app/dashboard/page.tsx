'use client';

import { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Link } from '@/lib/router';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { useAuthStore } from '@/stores';
import { toolsApi } from '@/lib/api/tools';
import { skillsApi } from '@/lib/api/skills';
import { cliInterfacesApi } from '@/lib/api/cli-interfaces';

export default function DashboardPage() {
  const { t } = useTranslation();
  const { user, tenant } = useAuthStore();
  const [stats, setStats] = useState({
    tools: 0,
    skills: 0,
    interfaces: 0,
    apiCalls: 0,
  });
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchStats() {
      try {
        const [toolsRes, skillsRes, interfacesRes] = await Promise.all([
          toolsApi.list(),
          skillsApi.list(),
          cliInterfacesApi.list(),
        ]);

        setStats({
          tools: toolsRes.data?.length || 0,
          skills: skillsRes.data?.length || 0,
          interfaces: interfacesRes.data?.length || 0,
          apiCalls: Math.floor(Math.random() * 1000) + 100, // Mock for now
        });
      } catch (error) {
        console.error('Failed to fetch stats:', error);
      } finally {
        setLoading(false);
      }
    }

    fetchStats();
  }, []);

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">
          {t('dashboard.welcomeBack', { name: user?.username || t('nav.user') })}
        </h1>
        <p className="text-muted-foreground">
          {tenant?.name ? t('dashboard.orgPrefix', { name: tenant.name }) : t('dashboard.defaultSubtitle')}
        </p>
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        <StatCard 
          title={t('dashboard.stats.totalTools')} 
          value={loading ? '...' : stats.tools.toString()} 
          description={t('dashboard.stats.mcpToolsAvailable')} 
          href="/tools"
        />
        <StatCard 
          title={t('dashboard.stats.skills')} 
          value={loading ? '...' : stats.skills.toString()} 
          description={t('dashboard.stats.customSkills')} 
          href="/skills"
        />
        <StatCard
          title={t('dashboard.stats.interfaces')}
          value={loading ? '...' : stats.interfaces.toString()}
          description={t('dashboard.stats.cliInterfaces')}
          href="/interfaces"
        />
        <StatCard 
          title={t('dashboard.stats.apiCalls')} 
          value={loading ? '...' : stats.apiCalls.toLocaleString()} 
          description={t('dashboard.stats.thisMonth')} 
        />
      </div>

      {/* Quick Actions */}
      <Card>
        <CardHeader>
          <CardTitle>{t('dashboard.quickActions.title')}</CardTitle>
          <CardDescription>{t('dashboard.quickActions.subtitle')}</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-4">
            <QuickActionButton label={t('dashboard.quickActions.addTool')} href="/tools/new" />
            <QuickActionButton label={t('dashboard.quickActions.createSkill')} href="/skills/new" />
            <QuickActionButton label={t('dashboard.quickActions.addInterface')} href="/interfaces/new" />
            <QuickActionButton label={t('dashboard.quickActions.teamMembers')} href="/tenant/members" />
          </div>
        </CardContent>
      </Card>

      {/* Organization Info */}
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
              <Link to="/tenant/billing"><Button variant="outline" className="mt-4 w-full">
                {t('dashboard.organization.manageSubscription')}
              </Button></Link>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>{t('dashboard.quickLinks.title')}</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-2">
                <Link to="/tenant/api-keys"className="block text-sm text-primary hover:underline">
                  → {t('dashboard.quickLinks.manageApiKeys')}</Link>
                <Link to="/tenant/members"className="block text-sm text-primary hover:underline">
                  → {t('dashboard.quickLinks.inviteTeamMembers')}</Link>
                <Link to="/tenant/settings"className="block text-sm text-primary hover:underline">
                  → {t('dashboard.quickLinks.orgSettings')}</Link>
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
  href 
}: { 
  title: string; 
  value: string; 
  description: string;
  href?: string;
}) {
  const content = (
    <Card>
      <CardContent className="pt-6">
        <div className="text-2xl font-bold">{value}</div>
        <div className="text-sm text-muted-foreground">{title}</div>
        <div className="text-xs text-muted-foreground mt-1">{description}</div>
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
    <Link to={href}><Button variant="outline" className="w-full h-20">
      {label}
    </Button></Link>
  );
}