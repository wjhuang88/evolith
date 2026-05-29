'use client';

import { useState } from 'react';
import { useRouter } from '@/lib/router';
import { useTranslation } from 'react-i18next';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';
import { Button } from '@/components/ui';
import { Input } from '@/components/ui';
import { cliInterfacesApi } from '@/lib/api/cli-interfaces';

export default function NewInterfacePage() {
  const router = useRouter();
  const { t } = useTranslation();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  const [formData, setFormData] = useState({
    title: '',
    description: '',
    code: '',
    language: 'typescript',
    category: 'custom',
    tags: '',
    is_public: true,
  });
  
  const categories = ['custom', 'utility', 'api', 'data', 'ai'];
  const languages = ['typescript', 'javascript', 'python', 'rust', 'go', 'java', 'csharp', 'sql', 'bash', 'json', 'yaml', 'markdown'];

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);
    
    try {
      const response = await cliInterfacesApi.create({
        ...formData,
        tags: formData.tags.split(',').map(t => t.trim()).filter(Boolean),
      });

      if (response.success) {
        router.push('/interfaces');
      } else {
        setError(response.error?.message || t('interfaces.newInterface.failedToCreate'));
      }
    } catch (err) {
      setError(t('interfaces.newInterface.failedToCreate'));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="container mx-auto py-8 max-w-4xl">
      <div className="mb-8">
        <h1 className="text-3xl font-bold">{t('interfaces.newInterface.title')}</h1>
        <p className="text-muted-foreground mt-1">{t('interfaces.newInterface.subtitle')}</p>
      </div>

      {error && (
        <div className="mb-6 p-4 bg-destructive/10 border border-destructive rounded-md text-destructive">
          {error}
        </div>
      )}

      <form onSubmit={handleSubmit}>
        <Card className="mb-6">
          <CardHeader>
            <CardTitle>{t('interfaces.newInterface.basicInfo')}</CardTitle>
            <CardDescription>{t('interfaces.newInterface.basicInfoDesc')}</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div>
              <label className="text-sm font-medium">{t('interfaces.newInterface.titleLabel')}</label>
              <Input
                value={formData.title}
                onChange={(e) => setFormData({ ...formData, title: e.target.value })}
                placeholder={t('interfaces.newInterface.titlePlaceholder')}
                required
              />
            </div>
            <div>
              <label className="text-sm font-medium">{t('interfaces.newInterface.descLabel')}</label>
              <Input
                value={formData.description}
                onChange={(e) => setFormData({ ...formData, description: e.target.value })}
                placeholder={t('interfaces.newInterface.descPlaceholder')}
                required
              />
            </div>
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="text-sm font-medium">{t('interfaces.newInterface.languageLabel')}</label>
                <select
                  value={formData.language}
                  onChange={(e) => setFormData({ ...formData, language: e.target.value })}
                  className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                >
                  {languages.map(lang => (
                    <option key={lang} value={lang}>{t(`interfaces.languages.${lang}`)}</option>
                  ))}
                </select>
              </div>
              <div>
                <label className="text-sm font-medium">{t('interfaces.newInterface.categoryLabel')}</label>
                <select
                  value={formData.category}
                  onChange={(e) => setFormData({ ...formData, category: e.target.value })}
                  className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                >
                  {categories.map(cat => (
                    <option key={cat} value={cat}>{t(`interfaces.category.${cat}`)}</option>
                  ))}
                </select>
              </div>
            </div>
            <div>
              <label className="text-sm font-medium">{t('interfaces.newInterface.tagsLabel')}</label>
              <Input
                value={formData.tags}
                onChange={(e) => setFormData({ ...formData, tags: e.target.value })}
                placeholder={t('interfaces.newInterface.tagsPlaceholder')}
              />
            </div>
            <div className="flex items-center gap-2">
              <input
                type="checkbox"
                id="is_public"
                checked={formData.is_public}
                onChange={(e) => setFormData({ ...formData, is_public: e.target.checked })}
                className="w-4 h-4"
              />
              <label htmlFor="is_public" className="text-sm">{t('interfaces.newInterface.publicInterface')}</label>
            </div>
          </CardContent>
        </Card>

        <Card className="mb-6">
          <CardHeader>
            <CardTitle>{t('interfaces.newInterface.codeTitle')}</CardTitle>
            <CardDescription>{t('interfaces.newInterface.codeDesc')}</CardDescription>
          </CardHeader>
          <CardContent>
            <textarea
              value={formData.code}
              onChange={(e) => setFormData({ ...formData, code: e.target.value })}
              className="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm font-mono min-h-[300px]"
              placeholder={t('interfaces.newInterface.codePlaceholder')}
              required
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
