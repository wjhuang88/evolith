'use client';

import { useEffect, useState } from 'react';
import { Link } from '@/lib/router';
import { useTranslation } from 'react-i18next';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';
import { Button, Input } from '@/components/ui';
import { cliInterfacesApi } from '@/lib/api/cli-interfaces';
import type { CliInterface } from '@/lib/api/types';

export default function InterfacesPage() {
  const { t } = useTranslation();
  const [interfaces, setInterfaces] = useState<CliInterface[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function fetchInterfaces() {
      try {
        const response = await cliInterfacesApi.list();
        if (response.success && response.data) {
          setInterfaces(response.data);
        } else {
          setError(response.error?.message || t('interfaces.failedToLoad'));
        }
      } catch (err) {
        setError(t('interfaces.failedToConnect'));
      } finally {
        setLoading(false);
      }
    }
    fetchInterfaces();
  }, [t]);

  return (
    <div className="container mx-auto py-8">
      <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 mb-8">
        <div>
          <h1 className="text-3xl font-bold">{t('interfaces.title')}</h1>
          <p className="text-muted-foreground mt-1">
            {t('interfaces.subtitle')}
          </p>
        </div>
        <Link to="/interfaces/new"><Button>{t('interfaces.createInterface')}</Button></Link>
      </div>

      <div className="mb-6">
        <Input type="search" placeholder={t('interfaces.searchPlaceholder')} className="w-full max-w-md" />
      </div>

      {loading && (
        <div className="text-center py-12 text-muted-foreground">
          {t('interfaces.loadingInterfaces')}
        </div>
      )}

      {error && (
        <div className="text-center py-12 text-destructive">
          {error}
        </div>
      )}

      {!loading && !error && interfaces.length === 0 && (
        <div className="text-center py-12 text-muted-foreground">
          <p>{t('interfaces.noInterfaces')}</p>
        </div>
      )}

      {!loading && !error && interfaces.length > 0 && (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          {interfaces.map((iface) => (
            <Link key={iface.id} to={`/interfaces/${iface.id}`}>
              <Card className="hover:shadow-md transition-shadow cursor-pointer">
                <CardHeader>
                  <CardTitle className="flex items-center gap-2">
                    {iface.title}
                    {!iface.is_public && (
                      <span className="text-xs bg-muted px-2 py-0.5 rounded">{t('common.private')}</span>
                    )}
                  </CardTitle>
                  <CardDescription>{iface.description}</CardDescription>
                </CardHeader>
                <CardContent>
                  <div className="flex items-center justify-between text-sm text-muted-foreground">
                    <span>{iface.language}</span>
                    <span className="capitalize">{iface.category}</span>
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