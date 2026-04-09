'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { useTranslation } from 'react-i18next';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';
import { Button } from '@/components/ui';
import { Input } from '@/components/ui';
import { toolsApi } from '@/lib/api/tools';

interface ToolParameter {
  name: string;
  type: string;
  description: string;
  required: boolean;
  default?: string;
  enum?: string;
}

const PARAMETER_TYPES = ['string', 'number', 'boolean', 'object', 'array'];

export default function NewToolPage() {
  const router = useRouter();
  const { t } = useTranslation();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  const [formData, setFormData] = useState({
    name: '',
    description: '',
    category: 'custom',
    is_public: true,
  });
  
  const [parameters, setParameters] = useState<ToolParameter[]>([]);
  
  const categories = ['custom', 'utility', 'api', 'data', 'ai'];

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);
    
    try {
      const schema: Record<string, unknown> = {
        type: 'object',
        properties: {},
        required: [],
      };
      
      parameters.forEach(param => {
        const prop: Record<string, unknown> = {
          type: param.type,
          description: param.description,
        };
        if (param.enum && param.enum.length > 0) {
          prop.enum = param.enum.split(',').map(s => s.trim());
        }
        if (param.default) {
          prop.default = param.default;
        }
        (schema.properties as Record<string, unknown>)[param.name] = prop;
        if (param.required) {
          (schema.required as string[]).push(param.name);
        }
      });
      
      const response = await toolsApi.create({
        ...formData,
        input_schema: schema,
      });
      
      if (response.success) {
        router.push('/tools');
      } else {
        setError(response.error?.message || t('tools.newTool.failedToCreate'));
      }
    } catch (err) {
      setError(t('tools.newTool.failedToCreate'));
    } finally {
      setLoading(false);
    }
  };

  const addParameter = () => {
    setParameters([...parameters, {
      name: '',
      type: 'string',
      description: '',
      required: false,
    }]);
  };

  const removeParameter = (index: number) => {
    setParameters(parameters.filter((_, i) => i !== index));
  };

  const updateParameter = (index: number, field: keyof ToolParameter, value: string | boolean) => {
    const updated = [...parameters];
    updated[index] = { ...updated[index], [field]: value };
    setParameters(updated);
  };

  return (
    <div className="container mx-auto py-8 max-w-4xl">
      <div className="mb-8">
        <h1 className="text-3xl font-bold">{t('tools.newTool.title')}</h1>
        <p className="text-muted-foreground mt-1">{t('tools.newTool.subtitle')}</p>
      </div>

      {error && (
        <div className="mb-6 p-4 bg-destructive/10 border border-destructive rounded-md text-destructive">
          {error}
        </div>
      )}

      <form onSubmit={handleSubmit}>
        <Card className="mb-6">
          <CardHeader>
            <CardTitle>{t('tools.newTool.basicInfo')}</CardTitle>
            <CardDescription>{t('tools.newTool.basicInfoDesc')}</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div>
              <label className="text-sm font-medium">{t('tools.newTool.nameLabel')}</label>
              <Input
                value={formData.name}
                onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                placeholder={t('tools.newTool.namePlaceholder')}
                required
              />
            </div>
            <div>
              <label className="text-sm font-medium">{t('tools.newTool.descLabel')}</label>
              <Input
                value={formData.description}
                onChange={(e) => setFormData({ ...formData, description: e.target.value })}
                placeholder={t('tools.newTool.descPlaceholder')}
                required
              />
            </div>
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="text-sm font-medium">{t('tools.categoryLabel')}</label>
                <select
                  value={formData.category}
                  onChange={(e) => setFormData({ ...formData, category: e.target.value })}
                  className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                >
                  {categories.map(cat => (
                    <option key={cat} value={cat}>{t(`tools.category.${cat}`)}</option>
                  ))}
                </select>
              </div>
              <div className="flex items-center gap-2 pt-6">
                <input
                  type="checkbox"
                  id="is_public"
                  checked={formData.is_public}
                  onChange={(e) => setFormData({ ...formData, is_public: e.target.checked })}
                  className="w-4 h-4"
                />
                <label htmlFor="is_public" className="text-sm">{t('tools.newTool.publicTool')}</label>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card className="mb-6">
          <CardHeader className="flex flex-row items-center justify-between">
            <div>
              <CardTitle>{t('tools.newTool.parametersTitle')}</CardTitle>
              <CardDescription>{t('tools.newTool.parametersDesc')}</CardDescription>
            </div>
            <Button type="button" variant="outline" onClick={addParameter}>
              {t('tools.addParameter')}
            </Button>
          </CardHeader>
          <CardContent>
            {parameters.length === 0 ? (
              <div className="text-center py-8 text-muted-foreground">
                {t('tools.newTool.noParams')}
              </div>
            ) : (
              <div className="space-y-4">
                {parameters.map((param, index) => (
                  <div key={index} className="p-4 border rounded-md space-y-4">
                    <div className="flex justify-between items-start">
                      <span className="font-medium">{t('tools.newTool.paramIndex', { index: index + 1 })}</span>
                      <Button
                        type="button"
                        variant="ghost"
                        size="sm"
                        onClick={() => removeParameter(index)}
                        className="text-destructive"
                      >
                        {t('tools.removeParameter')}
                      </Button>
                    </div>
                    <div className="grid grid-cols-2 gap-4">
                      <div>
                        <label className="text-sm font-medium">{t('tools.parameterName')}</label>
                        <Input
                          value={param.name}
                          onChange={(e) => updateParameter(index, 'name', e.target.value)}
                          placeholder="parameter_name"
                          required
                        />
                      </div>
                      <div>
                        <label className="text-sm font-medium">{t('tools.parameterType')}</label>
                        <select
                          value={param.type}
                          onChange={(e) => updateParameter(index, 'type', e.target.value)}
                          className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
                        >
                          {PARAMETER_TYPES.map(type => (
                            <option key={type} value={type}>{t(`tools.types.${type}`)}</option>
                          ))}
                        </select>
                      </div>
                    </div>
                    <div>
                      <label className="text-sm font-medium">{t('tools.parameterDescription')}</label>
                      <Input
                        value={param.description}
                        onChange={(e) => updateParameter(index, 'description', e.target.value)}
                        placeholder={t('tools.parameterDescPlaceholder')}
                      />
                    </div>
                    <div className="grid grid-cols-3 gap-4">
                      <div className="flex items-center gap-2">
                        <input
                          type="checkbox"
                          id={`param-required-${index}`}
                          checked={param.required}
                          onChange={(e) => updateParameter(index, 'required', e.target.checked)}
                          className="w-4 h-4"
                        />
                        <label htmlFor={`param-required-${index}`} className="text-sm">{t('tools.parameterRequired')}</label>
                      </div>
                      <div>
                        <label className="text-sm font-medium">{t('tools.parameterDefault')}</label>
                        <Input
                          value={param.default || ''}
                          onChange={(e) => updateParameter(index, 'default', e.target.value)}
                          placeholder={t('tools.parameterDefaultPlaceholder')}
                        />
                      </div>
                      <div>
                        <label className="text-sm font-medium">{t('tools.parameterEnum')}</label>
                        <Input
                          value={param.enum || ''}
                          onChange={(e) => updateParameter(index, 'enum', e.target.value)}
                          placeholder={t('tools.parameterEnumPlaceholder')}
                        />
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}
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
