'use client';

import { useEffect, useState } from 'react';
import Link from 'next/link';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { useAuthStore } from '@/stores';
import { toolsApi } from '@/lib/api/tools';
import { skillsApi } from '@/lib/api/skills';
import { snippetsApi } from '@/lib/api/snippets';

export default function DashboardPage() {
  const { user, tenant } = useAuthStore();
  const [stats, setStats] = useState({
    tools: 0,
    skills: 0,
    snippets: 0,
    apiCalls: 0,
  });
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchStats() {
      try {
        const [toolsRes, skillsRes, snippetsRes] = await Promise.all([
          toolsApi.list(),
          skillsApi.list(),
          snippetsApi.list(),
        ]);

        setStats({
          tools: toolsRes.data?.length || 0,
          skills: skillsRes.data?.length || 0,
          snippets: snippetsRes.data?.length || 0,
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
          Welcome back, {user?.username || 'User'}!
        </h1>
        <p className="text-muted-foreground">
          {tenant?.name ? `Organization: ${tenant.name}` : 'Evolith - AI Agent Development Platform'}
        </p>
      </div>

      {/* Stats Grid */}
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <StatCard 
          title="Total Tools" 
          value={loading ? '...' : stats.tools.toString()} 
          description="MCP tools available" 
          href="/tools"
        />
        <StatCard 
          title="Skills" 
          value={loading ? '...' : stats.skills.toString()} 
          description="Custom skills" 
          href="/skills"
        />
        <StatCard 
          title="Snippets" 
          value={loading ? '...' : stats.snippets.toString()} 
          description="Code snippets" 
          href="/snippets"
        />
        <StatCard 
          title="API Calls" 
          value={loading ? '...' : stats.apiCalls.toLocaleString()} 
          description="This month" 
        />
      </div>

      {/* Quick Actions */}
      <Card>
        <CardHeader>
          <CardTitle>Quick Actions</CardTitle>
          <CardDescription>Common tasks to get started</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-2 gap-4 md:grid-cols-4">
            <QuickActionButton label="Add Tool" href="/tools/new" />
            <QuickActionButton label="Create Skill" href="/skills/new" />
            <QuickActionButton label="Add Snippet" href="/snippets/new" />
            <QuickActionButton label="Team Members" href="/tenant/members" />
          </div>
        </CardContent>
      </Card>

      {/* Organization Info */}
      {tenant && (
        <div className="grid gap-4 md:grid-cols-2">
          <Card>
            <CardHeader>
              <CardTitle>Organization</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-2">
                <div className="flex justify-between">
                  <span className="text-muted-foreground">Plan</span>
                  <span className="font-medium capitalize">{tenant.plan || 'Free'}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-muted-foreground">Role</span>
                  <span className="font-medium capitalize">{user?.tenant_role || 'member'}</span>
                </div>
              </div>
              <Link href="/tenant/billing">
                <Button variant="outline" className="mt-4 w-full">
                  Manage Subscription
                </Button>
              </Link>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Quick Links</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-2">
                <Link href="/tenant/api-keys" className="block text-sm text-primary hover:underline">
                  → Manage API Keys
                </Link>
                <Link href="/tenant/members" className="block text-sm text-primary hover:underline">
                  → Invite Team Members
                </Link>
                <Link href="/tenant/settings" className="block text-sm text-primary hover:underline">
                  → Organization Settings
                </Link>
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
    return <Link href={href}>{content}</Link>;
  }
  return content;
}

function QuickActionButton({ label, href }: { label: string; href: string }) {
  return (
    <Link href={href}>
      <Button variant="outline" className="w-full h-20">
        {label}
      </Button>
    </Link>
  );
}