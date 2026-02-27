'use client';

import { useEffect, useState } from 'react';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';
import { Button } from '@/components/ui';
import { skillsApi } from '@/lib/api/skills';
import type { Skill } from '@/lib/api/types';

export default function SkillsPage() {
  const [skills, setSkills] = useState<Skill[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function fetchSkills() {
      try {
        const response = await skillsApi.list();
        if (response.success && response.data) {
          setSkills(response.data);
        } else {
          setError(response.error?.message || 'Failed to load skills');
        }
      } catch (err) {
        setError('Failed to connect to server');
      } finally {
        setLoading(false);
      }
    }
    fetchSkills();
  }, []);

  return (
    <div className="container mx-auto py-8">
      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-3xl font-bold">Skills</h1>
          <p className="text-muted-foreground mt-1">
            兼容Claude Skills格式的混合型技能系统
          </p>
        </div>
        <Button>Create Skill</Button>
      </div>

      {loading && (
        <div className="text-center py-12 text-muted-foreground">
          Loading skills...
        </div>
      )}

      {error && (
        <div className="text-center py-12 text-destructive">
          {error}
        </div>
      )}

      {!loading && !error && skills.length === 0 && (
        <div className="text-center py-12 text-muted-foreground">
          <p>No skills available yet. Create your first skill to get started.</p>
        </div>
      )}

      {!loading && !error && skills.length > 0 && (
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
          {skills.map((skill) => (
            <Card key={skill.id} className="hover:shadow-md transition-shadow cursor-pointer">
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  {skill.name}
                  {!skill.is_public && (
                    <span className="text-xs bg-muted px-2 py-0.5 rounded">Private</span>
                  )}
                </CardTitle>
                <CardDescription>{skill.description}</CardDescription>
              </CardHeader>
              <CardContent>
                <div className="flex items-center justify-between text-sm text-muted-foreground">
                  <span className="capitalize">{skill.category}</span>
                  <span>{skill.version}</span>
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      )}
    </div>
  );
}
