'use client';

import { useEffect, useState } from 'react';
import Link from 'next/link';
import { useTranslation } from 'react-i18next';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';
import { Button, Input } from '@/components/ui';
import { snippetsApi } from '@/lib/api/snippets';
import type { Snippet } from '@/lib/api/types';

export default function SnippetsPage() {
  const { t } = useTranslation();
  const [snippets, setSnippets] = useState<Snippet[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function fetchSnippets() {
      try {
        const response = await snippetsApi.list();
        if (response.success && response.data) {
          setSnippets(response.data);
        } else {
          setError(response.error?.message || t('snippets.failedToLoad'));
        }
      } catch (err) {
        setError(t('snippets.failedToConnect'));
      } finally {
        setLoading(false);
      }
    }
    fetchSnippets();
  }, [t]);

  return (
    <div className="container mx-auto py-8">
      <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 mb-8">
        <div>
          <h1 className="text-3xl font-bold">{t('snippets.title')}</h1>
          <p className="text-muted-foreground mt-1">
            {t('snippets.subtitle')}
          </p>
        </div>
        <Link href="/snippets/new">
          <Button>{t('snippets.createSnippet')}</Button>
        </Link>
      </div>

      <div className="mb-6">
        <Input type="search" placeholder={t('snippets.searchPlaceholder')} className="w-full max-w-md" />
      </div>

      {loading && (
        <div className="text-center py-12 text-muted-foreground">
          {t('snippets.loadingSnippets')}
        </div>
      )}

      {error && (
        <div className="text-center py-12 text-destructive">
          {error}
        </div>
      )}

      {!loading && !error && snippets.length === 0 && (
        <div className="text-center py-12 text-muted-foreground">
          <p>{t('snippets.noSnippets')}</p>
        </div>
      )}

      {!loading && !error && snippets.length > 0 && (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          {snippets.map((snippet) => (
            <Link key={snippet.id} href={`/snippets/${snippet.id}`}>
              <Card className="hover:shadow-md transition-shadow cursor-pointer">
                <CardHeader>
                  <CardTitle className="flex items-center gap-2">
                    {snippet.title}
                    {!snippet.is_public && (
                      <span className="text-xs bg-muted px-2 py-0.5 rounded">{t('common.private')}</span>
                    )}
                  </CardTitle>
                  <CardDescription>{snippet.description}</CardDescription>
                </CardHeader>
                <CardContent>
                  <div className="flex items-center justify-between text-sm text-muted-foreground">
                    <span>{snippet.language}</span>
                    <span className="capitalize">{snippet.category}</span>
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