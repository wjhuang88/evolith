'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { useTranslation } from 'react-i18next';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';
import { Button } from '@/components/ui';
import { Input } from '@/components/ui';
import { skillsApi } from '@/lib/api/skills';

export default function NewSkillPage() {
  const router = useRouter();
  const { t } = useTranslation();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  const [formData, setFormData] = useState({
    name: '',
    description: '',
    version: '1.0.0',
    content: '',
    category: 'custom',
    tags: '',
    is_public: true,
  });
  
  const categories = ['custom', 'utility', 'api', 'data', 'ai'];

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);
    
    try {
      const response = await skillsApi.create({
        ...formData,
        tags: formData.tags.split(',').map(t => t.trim()).filter(Boolean),
      });
      
      if (response.success) {
        router.push('/skills');
      } else {
        setError(response.error?.message || t('skills.newSkill.failedToCreate'));
      }
    } catch (err) {
      setError(t('skills.newSkill.failedToCreate'));
    } finally {
      setLoading(false);
    }
  };

  const defaultSkillContent = `---
name: ${formData.name || 'skill-name'}
description: A brief description of what this skill does
version: ${formData.version}
tags:
  - utility
---

# SKILL.md

## Description
Describe what this skill does and when to use it.

## Usage
Explain how to use this skill with arguments.

## Examples
Provide examples of how to use this skill.

## Notes
Any additional notes or considerations.
`;

  return (
    <div className="container mx-auto py-8 max-w-4xl">
      <div className="mb-8">
        <h1 className="text-3xl font-bold">{t('skills.newSkill.title')}</h1>
        <p className="text-muted-foreground mt-1">{t('skills.newSkill.subtitle')}</p>
      </div>

      {error && (
        <div className="mb-6 p-4 bg-destructive/10 border border-destructive rounded-md text-destructive">
          {error}
        </div>
      )}

      <form onSubmit={handleSubmit}>
        <Card className="mb-6">
          <CardHeader>
            <CardTitle>{t('skills.newSkill.basicInfo')}</CardTitle>
            <CardDescription>{t('skills.newSkill.basicInfoDesc')}</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="text-sm font-medium">{t('skills.newSkill.nameLabel')}</label>
                <Input
                  value={formData.name}
                  onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                  placeholder={t('skills.newSkill.namePlaceholder')}
                  required
                />
              </div>
              <div>
                <label className="text-sm font-medium">{t('skills.newSkill.versionLabel')}</label>
                <Input
                  value={formData.version}
                  onChange={(e) => setFormData({ ...formData, version: e.target.value })}
                  placeholder={t('skills.newSkill.versionPlaceholder')}
                />
              </div>
            </div>
            <div>
              <label className="text-sm font-medium">{t('skills.newSkill.descLabel')}</label>
              <Input
                value={formData.description}
                onChange={(e) => setFormData({ ...formData, description: e.target.value })}
                placeholder={t('skills.newSkill.descPlaceholder')}
                required
              />
            </div>
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="text-sm font-medium">{t('skills.newSkill.categoryLabel')}</label>
                <select
                  value={formData.category}
                  onChange={(e) => setFormData({ ...formData, category: e.target.value })}
                  className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                >
                  {categories.map(cat => (
                    <option key={cat} value={cat}>{t(`skills.category.${cat}`)}</option>
                  ))}
                </select>
              </div>
              <div>
                <label className="text-sm font-medium">{t('skills.newSkill.tagsLabel')}</label>
                <Input
                  value={formData.tags}
                  onChange={(e) => setFormData({ ...formData, tags: e.target.value })}
                  placeholder={t('skills.newSkill.tagsPlaceholder')}
                />
              </div>
            </div>
            <div className="flex items-center gap-2">
              <input
                type="checkbox"
                id="is_public"
                checked={formData.is_public}
                onChange={(e) => setFormData({ ...formData, is_public: e.target.checked })}
                className="w-4 h-4"
              />
              <label htmlFor="is_public" className="text-sm">{t('skills.newSkill.publicSkill')}</label>
            </div>
          </CardContent>
        </Card>

        <Card className="mb-6">
          <CardHeader>
            <CardTitle>{t('skills.newSkill.contentTitle')}</CardTitle>
            <CardDescription>{t('skills.newSkill.contentDesc')}</CardDescription>
          </CardHeader>
          <CardContent>
            <textarea
              value={formData.content}
              onChange={(e) => setFormData({ ...formData, content: e.target.value })}
              className="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm font-mono min-h-[400px]"
              placeholder={defaultSkillContent}
            />
          </CardContent>
        </Card>

        <div className="flex gap-4">
          <Button type="submit" disabled={loading}>
            {loading ? t('common.creating') : t('common.create')}
          </Button>
          <Button type="button" variant="outline" onClick={() => router.back()}>
            {t('common.back')}
          </Button>
        </div>
      </form>
    </div>
  );
}
