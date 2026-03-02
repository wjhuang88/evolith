'use client';

import { useEffect, useState, useCallback } from 'react';
import { useParams, useRouter } from 'next/navigation';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';
import { Button } from '@/components/ui';
import { snippetsApi } from '@/lib/api/snippets';
import type { Snippet } from '@/lib/api/types';

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

export default function SnippetDetailPage() {
  const params = useParams();
  const router = useRouter();
  const snippetId = params.id as string;
  
  const [snippet, setSnippet] = useState<Snippet | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  
  // Reference state
  const [referenceFormat, setReferenceFormat] = useState<'direct' | 'inline' | 'with_deps'>('direct');
  const [referenceData, setReferenceData] = useState<ReferenceResponse | null>(null);
  const [loadingReference, setLoadingReference] = useState(false);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    async function fetchSnippet() {
      try {
        const response = await snippetsApi.get(snippetId);
        if (response.success && response.data) {
          setSnippet(response.data);
        } else {
          setError(response.error?.message || 'Snippet not found');
        }
      } catch (err) {
        setError('Failed to load snippet');
      } finally {
        setLoading(false);
      }
    }
    fetchSnippet();
  }, [snippetId]);

  const loadReference = useCallback(async () => {
    setLoadingReference(true);
    try {
      const response = await snippetsApi.getReference(snippetId, referenceFormat);
      if (response.success && response.data) {
        setReferenceData(response.data as ReferenceResponse);
      }
    } catch (err) {
      console.error('Failed to load reference');
    } finally {
      setLoadingReference(false);
    }
  }, [snippetId, referenceFormat]);

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
    if (!confirm('Are you sure you want to delete this snippet?')) return;
    
    try {
      const response = await snippetsApi.delete(snippetId);
      if (response.success) {
        router.push('/snippets');
      } else {
        alert(response.error?.message || 'Failed to delete');
      }
    } catch (err) {
      alert('Failed to delete snippet');
    }
  };

  if (loading) {
    return (
      <div className="container mx-auto py-8">
        <div className="text-center text-muted-foreground">Loading...</div>
      </div>
    );
  }

  if (error || !snippet) {
    return (
      <div className="container mx-auto py-8">
        <div className="text-center text-destructive">{error || 'Snippet not found'}</div>
        <Button className="mt-4" onClick={() => router.push('/snippets')}>Back to Snippets</Button>
      </div>
    );
  }

  return (
    <div className="container mx-auto py-8 max-w-4xl">
      <Button variant="ghost" onClick={() => router.push('/snippets')} className="mb-4">
        Back to Snippets
      </Button>

      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-3xl font-bold flex items-center gap-2">
            {snippet.title}
            {!snippet.is_public && (
              <span className="text-xs bg-muted px-2 py-0.5 rounded">Private</span>
            )}
          </h1>
          <p className="text-muted-foreground mt-1">{snippet.description}</p>
          <div className="flex gap-4 mt-2 text-sm text-muted-foreground">
            <span>Language: {snippet.language}</span>
            <span>Category: {snippet.category}</span>
          </div>
        </div>
        <Button variant="destructive" onClick={handleDelete}>Delete</Button>
      </div>

      <Card className="mb-6">
        <CardHeader>
          <CardTitle>Code</CardTitle>
        </CardHeader>
        <CardContent>
          <pre className="bg-muted p-4 rounded-md overflow-auto text-sm whitespace-pre-wrap max-h-64">
            {snippet.code}
          </pre>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <div>
            <CardTitle>Reference</CardTitle>
            <CardDescription>Get formatted code for LLM consumption</CardDescription>
          </div>
          <div className="flex gap-2">
            <Button 
              variant={referenceFormat === 'direct' ? 'primary' : 'outline'} 
              size="sm"
              onClick={() => setReferenceFormat('direct')}
            >
              Direct
            </Button>
            <Button 
              variant={referenceFormat === 'inline' ? 'primary' : 'outline'} 
              size="sm"
              onClick={() => setReferenceFormat('inline')}
            >
              Inline
            </Button>
            <Button 
              variant={referenceFormat === 'with_deps' ? 'primary' : 'outline'} 
              size="sm"
              onClick={() => setReferenceFormat('with_deps')}
            >
              With Deps
            </Button>
          </div>
        </CardHeader>
        <CardContent className="space-y-4">
          {loadingReference ? (
            <div className="text-center py-4 text-muted-foreground">Loading reference...</div>
          ) : referenceData ? (
            <>
              <div className="flex items-center justify-between text-sm text-muted-foreground">
                <span>Estimated tokens: {referenceData.estimated_tokens}</span>
                {referenceData.import_path && (
                  <span>Import: {referenceData.import_path}</span>
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
                  {copied ? 'Copied!' : 'Copy'}
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
                  <h4 className="text-sm font-medium mb-2">Usage Example</h4>
                  <pre className="bg-muted p-4 rounded-md overflow-auto text-sm whitespace-pre-wrap">
                    {referenceData.usage_example}
                  </pre>
                </div>
              )}
            </>
          ) : (
            <div className="text-center py-4 text-muted-foreground">No reference data</div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}