'use client';

import { useEffect, useState } from 'react';
import Link from 'next/link';
import { useTranslation } from 'react-i18next';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';
import { Button } from '@/components/ui';
import { toolsApi } from '@/lib/api/tools';
import type { Tool } from '@/lib/api/types';

export default function ToolsPage() {
  const { t } = useTranslation();
  const [tools, setTools] = useState<Tool[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function fetchTools() {
      try {
        const response = await toolsApi.list();
        if (response.success && response.data) {
          setTools(response.data);
        } else {
          setError(response.error?.message || t('tools.failedToLoad'));
        }
      } catch (err) {
        setError(t('tools.failedToConnect'));
      } finally {
        setLoading(false);
      }
    }
    fetchTools();
  }, [t]);

  return (
    <div className="container mx-auto py-8">
      <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 mb-8">
        <div>
          <h1 className="text-3xl font-bold">{t('tools.title')}</h1>
          <p className="text-muted-foreground mt-1">
            {t('tools.subtitle')}
          </p>
        </div>
        <Link href="/tools/new">
          <Button>{t('tools.createTool')}</Button>
        </Link>
      </div>

      {loading && (
        <div className="text-center py-12 text-muted-foreground">
          {t('tools.loadingTools')}
        </div>
      )}

      {error && (
        <div className="text-center py-12 text-destructive">
          {error}
        </div>
      )}

      {!loading && !error && tools.length === 0 && (
        <div className="text-center py-12 text-muted-foreground">
          <p>{t('tools.noTools')}</p>
        </div>
      )}

      {!loading && !error && tools.length > 0 && (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          {tools.map((tool) => (
            <Link key={tool.id} href={`/tools/${tool.id}`}>
              <Card className="hover:shadow-md transition-shadow cursor-pointer">
                <CardHeader>
                  <CardTitle className="flex items-center gap-2">
                    {tool.name}
                    {!tool.is_public && (
                      <span className="text-xs bg-muted px-2 py-0.5 rounded">{t('common.private')}</span>
                    )}
                  </CardTitle>
                  <CardDescription>{tool.description}</CardDescription>
                </CardHeader>
                <CardContent>
                  <div className="flex items-center justify-between text-sm text-muted-foreground">
                    <span className="capitalize">{tool.category}</span>
                  </div>
                </CardContent>
              </Card>
            </Link>
          ))}
        </div>
      )}
    </div>
  );
}