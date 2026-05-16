'use client';

import { useState } from 'react';
import { useRouter } from '@/lib/router';
import { useTranslation } from 'react-i18next';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';
import { Button } from '@/components/ui';
import { Input } from '@/components/ui';
import { snippetsApi } from '@/lib/api/snippets';

export default function NewSnippetPage() {
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
      const response = await snippetsApi.create({
        ...formData,
        tags: formData.tags.split(',').map(t => t.trim()).filter(Boolean),
      });
      
      if (response.success) {
        router.push('/snippets');
      } else {
        setError(response.error?.message || t('snippets.newSnippet.failedToCreate'));
      }
    } catch (err) {
      setError(t('snippets.newSnippet.failedToCreate'));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="container mx-auto py-8 max-w-4xl">
      <div className="mb-8">
        <h1 className="text-3xl font-bold">{t('snippets.newSnippet.title')}</h1>
        <p className="text-muted-foreground mt-1">{t('snippets.newSnippet.subtitle')}</p>
      </div>

      {error && (
        <div className="mb-6 p-4 bg-destructive/10 border border-destructive rounded-md text-destructive">
          {error}
        </div>
      )}

      <form onSubmit={handleSubmit}>
        <Card className="mb-6">
          <CardHeader>
            <CardTitle>{t('snippets.newSnippet.basicInfo')}</CardTitle>
            <CardDescription>{t('snippets.newSnippet.basicInfoDesc')}</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div>
              <label className="text-sm font-medium">{t('snippets.newSnippet.titleLabel')}</label>
              <Input
                value={formData.title}
                onChange={(e) => setFormData({ ...formData, title: e.target.value })}
                placeholder={t('snippets.newSnippet.titlePlaceholder')}
                required
              />
            </div>
            <div>
              <label className="text-sm font-medium">{t('snippets.newSnippet.descLabel')}</label>
              <Input
                value={formData.description}
                onChange={(e) => setFormData({ ...formData, description: e.target.value })}
                placeholder={t('snippets.newSnippet.descPlaceholder')}
                required
              />
            </div>
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="text-sm font-medium">{t('snippets.newSnippet.languageLabel')}</label>
                <select
                  value={formData.language}
                  onChange={(e) => setFormData({ ...formData, language: e.target.value })}
                  className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                >
                  {languages.map(lang => (
                    <option key={lang} value={lang}>{t(`snippets.languages.${lang}`)}</option>
                  ))}
                </select>
              </div>
              <div>
                <label className="text-sm font-medium">{t('snippets.newSnippet.categoryLabel')}</label>
                <select
                  value={formData.category}
                  onChange={(e) => setFormData({ ...formData, category: e.target.value })}
                  className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                >
                  {categories.map(cat => (
                    <option key={cat} value={cat}>{t(`snippets.category.${cat}`)}</option>
                  ))}
                </select>
              </div>
            </div>
            <div>
              <label className="text-sm font-medium">{t('snippets.newSnippet.tagsLabel')}</label>
              <Input
                value={formData.tags}
                onChange={(e) => setFormData({ ...formData, tags: e.target.value })}
                placeholder={t('snippets.newSnippet.tagsPlaceholder')}
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
              <label htmlFor="is_public" className="text-sm">{t('snippets.newSnippet.publicSnippet')}</label>
            </div>
          </CardContent>
        </Card>

        <Card className="mb-6">
          <CardHeader>
            <CardTitle>{t('snippets.newSnippet.codeTitle')}</CardTitle>
            <CardDescription>{t('snippets.newSnippet.codeDesc')}</CardDescription>
          </CardHeader>
          <CardContent>
            <textarea
              value={formData.code}
              onChange={(e) => setFormData({ ...formData, code: e.target.value })}
              className="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm font-mono min-h-[300px]"
              placeholder={t('snippets.newSnippet.codePlaceholder')}
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
