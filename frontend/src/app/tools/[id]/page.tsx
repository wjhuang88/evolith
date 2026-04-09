'use client';

import { useEffect, useState } from 'react';
import { useParams, useRouter } from 'next/navigation';
import { useTranslation } from 'react-i18next';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';
import { Button } from '@/components/ui';
import { Input } from '@/components/ui';
import { toolsApi } from '@/lib/api/tools';
import type { Tool } from '@/lib/api/types';

export default function ToolDetailPage() {
  const { t } = useTranslation();
  const params = useParams();
  const router = useRouter();
  const toolId = params.id as string;
  
  const [tool, setTool] = useState<Tool | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [invoking, setInvoking] = useState(false);
  const [result, setResult] = useState<string | null>(null);
  const [invokeError, setInvokeError] = useState<string | null>(null);
  const [params_, setParams_] = useState<Record<string, unknown>>({});

  useEffect(() => {
    async function fetchTool() {
      try {
        const response = await toolsApi.get(toolId);
        if (response.success && response.data) {
          setTool(response.data);
          // Initialize params from schema
          const schema = response.data.input_schema as { properties?: Record<string, unknown> };
          if (schema.properties) {
            const initialParams: Record<string, unknown> = {};
            Object.keys(schema.properties).forEach(key => {
              const prop = schema.properties![key] as { default?: unknown };
              if (prop.default !== undefined) {
                initialParams[key] = prop.default;
              }
            });
            setParams_(initialParams);
          }
        } else {
          setError(response.error?.message || t('tools.toolNotFound'));
        }
      } catch (err) {
        setError(t('tools.failedToLoadTool'));
      } finally {
        setLoading(false);
      }
    }
    fetchTool();
  }, [toolId]);

  const handleInvoke = async () => {
    setInvoking(true);
    setResult(null);
    setInvokeError(null);
    
    try {
      const response = await toolsApi.execute(toolId, params_);
      if (response.success && response.data) {
        setResult(JSON.stringify(response.data, null, 2));
      } else {
        setInvokeError(response.error?.message || t('tools.executionFailed'));
      }
    } catch (err) {
      setInvokeError(t('tools.failedToExecute'));
    } finally {
      setInvoking(false);
    }
  };

  const handleDelete = async () => {
    if (!confirm(t('tools.confirmDelete'))) return;
    
    try {
      const response = await toolsApi.delete(toolId);
      if (response.success) {
        router.push('/tools');
      } else {
        alert(response.error?.message || t('tools.failedToDelete'));
      }
    } catch (err) {
      alert(t('tools.failedToDeleteTool'));
    }
  };

  if (loading) {
    return (
      <div className="container mx-auto py-8">
        <div className="text-center text-muted-foreground">{t('common.loading')}</div>
      </div>
    );
  }

  if (error || !tool) {
    return (
      <div className="container mx-auto py-8">
        <div className="text-center text-destructive">{error || t('tools.toolNotFound')}</div>
        <Button className="mt-4" onClick={() => router.push('/tools')}>{t('tools.backToTools')}</Button>
      </div>
    );
  }

  const schema = tool.input_schema as { properties?: Record<string, unknown>; required?: string[] };
  const properties = schema.properties || {};

  return (
    <div className="container mx-auto py-8 max-w-4xl">
      <Button variant="ghost" onClick={() => router.push('/tools')} className="mb-4">
        ← {t('tools.backToTools')}
      </Button>

      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-3xl font-bold flex items-center gap-2">
            {tool.name}
            {!tool.is_public && (
              <span className="text-xs bg-muted px-2 py-0.5 rounded">Private</span>
            )}
          </h1>
          <p className="text-muted-foreground mt-1">{tool.description}</p>
            <p className="text-sm text-muted-foreground mt-2">{t('tools.categoryLabel')}: {tool.category}</p>
        </div>
        <Button variant="destructive" onClick={handleDelete}>{t('tools.deleteTool')}</Button>
      </div>

      {Object.keys(properties).length > 0 && (
        <Card className="mb-6">
          <CardHeader>
            <CardTitle>{t('tools.parameters')}</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            {Object.entries(properties).map(([key, prop]) => {
              const schemaProp = prop as { type: string; description?: string; default?: unknown; enum?: string[] };
              const isRequired = schema.required?.includes(key);
              
              return (
                <div key={key} className="grid grid-cols-2 gap-4 items-start">
                  <div>
                    <label className="text-sm font-medium">
                      {key}
                      {isRequired && <span className="text-destructive ml-1">*</span>}
                    </label>
                    <p className="text-xs text-muted-foreground">{schemaProp.description}</p>
                    <p className="text-xs text-muted-foreground">Type: {schemaProp.type}</p>
                  </div>
                  {schemaProp.enum ? (
                    <select
                      value={(params_[key] as string) || ''}
                      onChange={(e) => setParams_({ ...params_, [key]: e.target.value })}
                      className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                    >
                      <option value="">Select...</option>
                      {schemaProp.enum.map((val) => (
                        <option key={val} value={val}>{val}</option>
                      ))}
                    </select>
                  ) : (
                    <Input
                      type={schemaProp.type === 'number' ? 'number' : 'text'}
                      value={(params_[key] as string) || ''}
                      onChange={(e) => setParams_({ 
                        ...params_, 
                        [key]: schemaProp.type === 'number' ? Number(e.target.value) : e.target.value 
                      })}
                      placeholder={t('tools.enterParam', { name: key })}
                    />
                  )}
                </div>
              );
            })}
            <Button onClick={handleInvoke} disabled={invoking} className="mt-4">
              {invoking ? t('tools.invoking') : t('tools.executeTool')}
            </Button>
          </CardContent>
        </Card>
      )}

      {invokeError && (
        <Card className="mb-6 border-destructive">
          <CardHeader>
            <CardTitle className="text-destructive">Error</CardTitle>
          </CardHeader>
          <CardContent>
            <pre className="text-sm text-destructive whitespace-pre-wrap">{invokeError}</pre>
          </CardContent>
        </Card>
      )}

      {result && (
        <Card>
          <CardHeader>
            <CardTitle>{t('tools.result')}</CardTitle>
          </CardHeader>
          <CardContent>
            <pre className="text-sm bg-muted p-4 rounded-md overflow-auto whitespace-pre-wrap">
              {result}
            </pre>
          </CardContent>
        </Card>
      )}
    </div>
  );
}