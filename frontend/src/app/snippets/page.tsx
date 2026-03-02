'use client';

import { useEffect, useState } from 'react';
import Link from 'next/link';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';
import { Button, Input } from '@/components/ui';
import { snippetsApi } from '@/lib/api/snippets';
import type { Snippet } from '@/lib/api/types';

export default function SnippetsPage() {
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
          setError(response.error?.message || 'Failed to load snippets');
        }
      } catch (err) {
        setError('Failed to connect to server');
      } finally {
        setLoading(false);
      }
    }
    fetchSnippets();
  }, []);

  return (
    <div className="container mx-auto py-8">
      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-3xl font-bold">Code Snippets</h1>
          <p className="text-muted-foreground mt-1">
            面向大模型的代码片段仓库，降低token消耗
          </p>
        </div>
        <Link href="/snippets/new">
          <Button>Create Snippet</Button>
        </Link>
      </div>

      <div className="mb-6">
        <Input type="search" placeholder="Search snippets..." className="max-w-md" />
      </div>

      {loading && (
        <div className="text-center py-12 text-muted-foreground">
          Loading snippets...
        </div>
      )}

      {error && (
        <div className="text-center py-12 text-destructive">
          {error}
        </div>
      )}

      {!loading && !error && snippets.length === 0 && (
        <div className="text-center py-12 text-muted-foreground">
          <p>No snippets available yet. Create your first snippet to get started.</p>
        </div>
      )}

      {!loading && !error && snippets.length > 0 && (
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
          {snippets.map((snippet) => (
            <Link key={snippet.id} href={`/snippets/${snippet.id}`}>
              <Card className="hover:shadow-md transition-shadow cursor-pointer">
                <CardHeader>
                  <CardTitle className="flex items-center gap-2">
                    {snippet.title}
                    {!snippet.is_public && (
                      <span className="text-xs bg-muted px-2 py-0.5 rounded">Private</span>
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