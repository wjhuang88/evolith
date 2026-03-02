'use client';

import { useEffect, useState } from 'react';
import Link from 'next/link';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';
import { Button } from '@/components/ui';
import { toolsApi } from '@/lib/api/tools';
import type { Tool } from '@/lib/api/types';

export default function ToolsPage() {
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
          setError(response.error?.message || 'Failed to load tools');
        }
      } catch (err) {
        setError('Failed to connect to server');
      } finally {
        setLoading(false);
      }
    }
    fetchTools();
  }, []);

  return (
    <div className="container mx-auto py-8">
      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-3xl font-bold">MCP Tools</h1>
          <p className="text-muted-foreground mt-1">
            标准化的工具封装，通过MCP协议提供服务
          </p>
        </div>
        <Link href="/tools/new">
          <Button>Create Tool</Button>
        </Link>
      </div>

      {loading && (
        <div className="text-center py-12 text-muted-foreground">
          Loading tools...
        </div>
      )}

      {error && (
        <div className="text-center py-12 text-destructive">
          {error}
        </div>
      )}

      {!loading && !error && tools.length === 0 && (
        <div className="text-center py-12 text-muted-foreground">
          <p>No tools available yet. Create your first tool to get started.</p>
        </div>
      )}

      {!loading && !error && tools.length > 0 && (
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
          {tools.map((tool) => (
            <Link key={tool.id} href={`/tools/${tool.id}`}>
              <Card className="hover:shadow-md transition-shadow cursor-pointer">
                <CardHeader>
                  <CardTitle className="flex items-center gap-2">
                    {tool.name}
                    {!tool.is_public && (
                      <span className="text-xs bg-muted px-2 py-0.5 rounded">Private</span>
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