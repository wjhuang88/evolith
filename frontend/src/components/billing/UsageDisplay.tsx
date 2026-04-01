'use client';

import { useState, useEffect } from 'react';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';
import { billingApi, type ResourceUsage, type UsageResponse } from '@/lib/api';

interface UsageDisplayProps {
  tenantId: string;
}

function UsageBar({ used, limit, percent }: { used: number; limit: number; percent: number }) {
  const getColor = () => {
    if (percent >= 90) return 'bg-destructive';
    if (percent >= 70) return 'bg-yellow-500';
    return 'bg-green-500';
  };

  return (
    <div className="w-full">
      <div className="flex justify-between text-sm mb-1">
        <span>{used.toLocaleString()} / {limit === -1 ? '∞' : limit.toLocaleString()}</span>
        <span>{percent.toFixed(1)}%</span>
      </div>
      <div className="h-2 bg-muted rounded-full overflow-hidden">
        <div
          className={`h-full transition-all ${getColor()}`}
          style={{ width: `${Math.min(percent, 100)}%` }}
        />
      </div>
    </div>
  );
}

function ResourceTypeIcon({ type }: { type: string }) {
  const icons: Record<string, string> = {
    api_calls: '🔌',
    users: '👥',
    tools: '🔧',
    skills: '⚡',
    snippets: '📝',
    storage_mb: '💾',
  };
  return <span className="text-2xl">{icons[type] || '📊'}</span>;
}

function ResourceTypeLabel({ type }: { type: string }) {
  const labels: Record<string, string> = {
    api_calls: 'API Calls',
    users: 'Users',
    tools: 'Tools',
    skills: 'Skills',
    snippets: 'Snippets',
    storage_mb: 'Storage',
  };
  return <span className="font-medium">{labels[type] || type}</span>;
}

export function UsageDisplay({ tenantId }: UsageDisplayProps) {
  const [usage, setUsage] = useState<UsageResponse | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadUsage();
  }, [tenantId]);

  const loadUsage = async () => {
    try {
      setLoading(true);
      const response = await billingApi.getUsage(tenantId);
      if (response.success && response.data) {
        setUsage(response.data);
      }
    } catch (error) {
      console.error('Failed to load usage:', error);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return (
      <Card>
        <CardContent className="py-8 text-center text-muted-foreground">
          Loading usage statistics...
        </CardContent>
      </Card>
    );
  }

  if (!usage) {
    return (
      <Card>
        <CardContent className="py-8 text-center text-muted-foreground">
          No usage data available
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Usage This Period</CardTitle>
        <CardDescription>
          {usage.period.remaining_days} days remaining in billing cycle
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          {usage.resources.map((resource) => (
            <div key={resource.resource_type} className="space-y-2">
              <div className="flex items-center gap-2">
                <ResourceTypeIcon type={resource.resource_type} />
                <ResourceTypeLabel type={resource.resource_type} />
              </div>
              <UsageBar
                used={resource.used}
                limit={resource.limit}
                percent={resource.percent}
              />
              {resource.overage !== undefined && resource.overage > 0 && (
                <p className="text-sm text-destructive">
                  Overage: ${resource.overage.toFixed(2)}
                </p>
              )}
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  );
}