'use client';

import { useEffect, useState, useCallback } from 'react';
import { useParams, useRouter } from '@/lib/router';
import { useTranslation } from 'react-i18next';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';
import { Button } from '@/components/ui';
import { cliInterfacesApi } from '@/lib/api/cli-interfaces';
import type { CliInterface } from '@/lib/api/types';

interface ReferenceResponse {
  name: string;
  language: string;
  import_path?: string;
  export_name?: string;
  code: string;
  usage_example?: string;
  dependencies?: Array<{
    name: string;
    version: string;
    install: string;
  }>;
  estimated_tokens: number;
}

export default function InterfaceDetailPage() {
  const { t } = useTranslation();
  const params = useParams();
  const router = useRouter();
  const interfaceId = params.id as string;

  const [iface, setIface] = useState<CliInterface | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  
  // Reference state
  const [referenceFormat, setReferenceFormat] = useState<'direct' | 'inline' | 'with_deps'>('direct');
  const [referenceData, setReferenceData] = useState<ReferenceResponse | null>(null);
  const [loadingReference, setLoadingReference] = useState(false);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    async function fetchInterface() {
      try {
        const response = await cliInterfacesApi.get(interfaceId);
        if (response.success && response.data) {
          setIface(response.data);
        } else {
          setError(response.error?.message || t('interfaces.interfaceNotFound'));
        }
      } catch (err) {
        setError(t('interfaces.failedToLoadInterface'));
      } finally {
        setLoading(false);
      }
    }
    fetchInterface();
  }, [interfaceId]);

  const loadReference = useCallback(async () => {
    setLoadingReference(true);
    try {
      const response = await cliInterfacesApi.getReference(interfaceId, referenceFormat);
      if (response.success && response.data) {
        setReferenceData(response.data as ReferenceResponse);
      }
    } catch (err) {
      console.error('Failed to load reference');
    } finally {
      setLoadingReference(false);
    }
  }, [interfaceId, referenceFormat]);

  useEffect(() => {
    loadReference();
  }, [loadReference]);

  const handleCopy = () => {
    if (referenceData?.code) {
      navigator.clipboard.writeText(referenceData.code);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  const handleDelete = async () => {
    if (!confirm(t('interfaces.confirmDelete'))) return;

    try {
      const response = await cliInterfacesApi.delete(interfaceId);
      if (response.success) {
        router.push('/interfaces');
      } else {
        alert(response.error?.message || t('interfaces.failedToDelete'));
      }
    } catch (err) {
      alert(t('interfaces.failedToDeleteInterface'));
    }
  };

  if (loading) {
    return (
      <div className="container mx-auto py-8">
        <div className="text-center text-muted-foreground">{t('common.loading')}</div>
      </div>
    );
  }

  if (error || !iface) {
    return (
      <div className="container mx-auto py-8">
        <div className="text-center text-destructive">{error || t('interfaces.interfaceNotFound')}</div>
        <Button className="mt-4" onClick={() => router.push('/interfaces')}>{t('interfaces.backToInterfaces')}</Button>
      </div>
    );
  }

  return (
    <div className="container mx-auto py-8 max-w-4xl">
      <Button variant="ghost" onClick={() => router.push('/interfaces')} className="mb-4">
        {t('interfaces.backToInterfaces')}
      </Button>

      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-3xl font-bold flex items-center gap-2">
            {iface.title}
            {!iface.is_public && (
              <span className="text-xs bg-muted px-2 py-0.5 rounded">Private</span>
            )}
          </h1>
          <p className="text-muted-foreground mt-1">{iface.description}</p>
          <div className="flex gap-4 mt-2 text-sm text-muted-foreground">
            <span>{t('interfaces.languageLabel')}: {iface.language}</span>
            <span>{t('interfaces.categoryLabel')}: {iface.category}</span>
          </div>
        </div>
        <Button variant="destructive" onClick={handleDelete}>{t('interfaces.deleteInterface')}</Button>
      </div>

      <Card className="mb-6">
        <CardHeader>
          <CardTitle>{t('interfaces.codeTitle')}</CardTitle>
        </CardHeader>
        <CardContent>
          <pre className="bg-muted p-4 rounded-md overflow-auto text-sm whitespace-pre-wrap max-h-64">
            {iface.code}
          </pre>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <div>
            <CardTitle>{t('interfaces.referenceTitle')}</CardTitle>
            <CardDescription>{t('interfaces.referenceDesc')}</CardDescription>
          </div>
          <div className="flex gap-2">
            <Button 
              variant={referenceFormat === 'direct' ? 'primary' : 'outline'} 
              size="sm"
              onClick={() => setReferenceFormat('direct')}
            >
              {t('interfaces.formats.direct')}
            </Button>
            <Button 
              variant={referenceFormat === 'inline' ? 'primary' : 'outline'} 
              size="sm"
              onClick={() => setReferenceFormat('inline')}
            >
              {t('interfaces.formats.inline')}
            </Button>
            <Button 
              variant={referenceFormat === 'with_deps' ? 'primary' : 'outline'} 
              size="sm"
              onClick={() => setReferenceFormat('with_deps')}
            >
              {t('interfaces.formats.withDeps')}
            </Button>
          </div>
        </CardHeader>
        <CardContent className="space-y-4">
          {loadingReference ? (
            <div className="text-center py-4 text-muted-foreground">{t('interfaces.loadingReference')}</div>
          ) : referenceData ? (
            <>
              <div className="flex items-center justify-between text-sm text-muted-foreground">
                <span>{t('interfaces.estimatedTokensLabel', { count: referenceData.estimated_tokens })}</span>
                {referenceData.import_path && (
                  <span>{t('interfaces.importLabel', { path: referenceData.import_path })}</span>
                )}
              </div>
              <div className="relative">
                <pre className="bg-muted p-4 rounded-md overflow-auto text-sm whitespace-pre-wrap max-h-64">
                  {referenceData.code}
                </pre>
                <Button 
                  className="absolute top-2 right-2" 
                  size="sm" 
                  onClick={handleCopy}
                >
                  {copied ? t('common.copied') : t('interfaces.copyCode')}
                </Button>
              </div>
              
              {referenceFormat === 'with_deps' && referenceData.dependencies && referenceData.dependencies.length > 0 && (
                <div>
                  <h4 className="text-sm font-medium mb-2">Dependencies</h4>
                  <div className="space-y-2">
                    {referenceData.dependencies.map((dep, i) => (
                      <div key={i} className="flex items-center justify-between p-2 bg-muted rounded text-sm">
                        <span>{dep.name}@{dep.version}</span>
                        <code className="text-xs">{dep.install}</code>
                      </div>
                    ))}
                  </div>
                </div>
              )}
              
              {referenceData.usage_example && (
                <div>
                  <h4 className="text-sm font-medium mb-2">{t('interfaces.usageExample')}</h4>
                  <pre className="bg-muted p-4 rounded-md overflow-auto text-sm whitespace-pre-wrap">
                    {referenceData.usage_example}
                  </pre>
                </div>
              )}
            </>
          ) : (
            <div className="text-center py-4 text-muted-foreground">{t('interfaces.noReferenceData')}</div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
