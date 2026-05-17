'use client';

import { useEffect, useState } from 'react';
import { Link } from '@/lib/router';
import { useTranslation } from 'react-i18next';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';
import { Button } from '@/components/ui';
import { skillsApi } from '@/lib/api/skills';
import type { Skill } from '@/lib/api/types';

export default function SkillsPage() {
  const { t } = useTranslation();
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
          setError(response.error?.message || t('skills.failedToLoad'));
        }
      } catch (err) {
        setError(t('skills.failedToConnect'));
      } finally {
        setLoading(false);
      }
    }
    fetchSkills();
  }, [t]);

  return (
    <div className="container mx-auto py-8">
      <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 mb-8">
        <div>
          <h1 className="text-3xl font-bold">{t('skills.title')}</h1>
          <p className="text-muted-foreground mt-1">
            {t('skills.subtitle')}
          </p>
        </div>
        <Link to="/skills/new"><Button>{t('skills.createSkill')}</Button></Link>
      </div>

      {loading && (
        <div className="text-center py-12 text-muted-foreground">
          {t('skills.loadingSkills')}
        </div>
      )}

      {error && (
        <div className="text-center py-12 text-destructive">
          {error}
        </div>
      )}

      {!loading && !error && skills.length === 0 && (
        <div className="text-center py-12 text-muted-foreground">
          <p>{t('skills.noSkills')}</p>
        </div>
      )}

      {!loading && !error && skills.length > 0 && (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          {skills.map((skill) => (
            <Link key={skill.id} to={`/skills/${skill.id}`}>
              <Card className="hover:shadow-md transition-shadow cursor-pointer">
                <CardHeader>
                  <CardTitle className="flex items-center gap-2">
                    {skill.name}
                    {!skill.is_public && (
                      <span className="text-xs bg-muted px-2 py-0.5 rounded">{t('common.private')}</span>
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
            </Link>
          ))}
        </div>
      )}
    </div>
  );
}