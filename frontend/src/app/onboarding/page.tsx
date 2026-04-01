'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { useTranslation } from 'react-i18next';
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from '@/components/ui';
import { Button } from '@/components/ui';
import { Input } from '@/components/ui';

import { toolsApi } from '@/lib/api/tools';
import { useAuthStore } from '@/stores/authStore';

type OnboardingStep = 1 | 2 | 3;

export default function OnboardingPage() {
  const router = useRouter();
  const { t } = useTranslation();
  const { user } = useAuthStore();
  const [step, setStep] = useState<OnboardingStep>(1);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  const [toolData, setToolData] = useState({
    name: '',
    description: '',
    schema: '{}',
  });

  const handleSkip = () => {
    router.push('/dashboard');
  };

  const handleNext = () => {
    if (step < 3) {
      setStep((step + 1) as OnboardingStep);
    }
  };

  const handleBack = () => {
    if (step > 1) {
      setStep((step - 1) as OnboardingStep);
    }
  };

  const handleCreateTool = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);
    
    try {
      let parsedSchema: Record<string, unknown>;
      try {
        parsedSchema = JSON.parse(toolData.schema);
      } catch {
        setError(t('onboarding.step2.invalidSchema'));
        setLoading(false);
        return;
      }

      const response = await toolsApi.create({
        name: toolData.name,
        description: toolData.description,
        category: 'custom',
        schema: parsedSchema,
        is_public: true,
      });
      
      if (response.success) {
        setStep(3);
      } else {
        setError(response.error?.message || t('onboarding.step2.failedToCreate'));
      }
    } catch {
      setError(t('onboarding.step2.failedToCreate'));
    } finally {
      setLoading(false);
    }
  };

  const renderProgress = () => {
    return (
      <div className="flex items-center justify-center gap-2 mb-8">
        {[1, 2, 3].map((s) => (
          <div key={s} className="flex items-center">
            <div
              className={`w-8 h-8 rounded-full flex items-center justify-center text-sm font-medium transition-colors ${
                s <= step
                  ? 'bg-primary-600 text-white'
                  : 'bg-muted text-muted-foreground'
              }`}
            >
              {s}
            </div>
            {s < 3 && (
              <div
                className={`w-12 h-0.5 mx-2 transition-colors ${
                  s < step ? 'bg-primary-600' : 'bg-muted'
                }`}
              />
            )}
          </div>
        ))}
      </div>
    );
  };

  const renderStep1 = () => {
    return (
      <Card>
        <CardHeader className="text-center">
          <CardTitle className="text-3xl">{t('onboarding.step1.title')}</CardTitle>
          <CardDescription className="text-lg mt-2">
            {t('onboarding.step1.subtitle', { username: user?.username || '' })}
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          <div className="grid gap-4 md:grid-cols-3">
            <div className="p-4 rounded-lg border bg-card text-center">
              <div className="text-3xl mb-2">🔧</div>
              <h3 className="font-semibold mb-1">{t('onboarding.step1.feature1.title')}</h3>
              <p className="text-sm text-muted-foreground">
                {t('onboarding.step1.feature1.description')}
              </p>
            </div>
            <div className="p-4 rounded-lg border bg-card text-center">
              <div className="text-3xl mb-2">⚡</div>
              <h3 className="font-semibold mb-1">{t('onboarding.step1.feature2.title')}</h3>
              <p className="text-sm text-muted-foreground">
                {t('onboarding.step1.feature2.description')}
              </p>
            </div>
            <div className="p-4 rounded-lg border bg-card text-center">
              <div className="text-3xl mb-2">📚</div>
              <h3 className="font-semibold mb-1">{t('onboarding.step1.feature3.title')}</h3>
              <p className="text-sm text-muted-foreground">
                {t('onboarding.step1.feature3.description')}
              </p>
            </div>
          </div>
          
          {error && (
            <div className="p-4 bg-destructive/10 border border-destructive rounded-md text-destructive text-sm">
              {error}
            </div>
          )}
        </CardContent>
        <CardFooter className="flex justify-between">
          <Button variant="ghost" onClick={handleSkip}>
            {t('onboarding.skip')}
          </Button>
          <Button onClick={handleNext}>
            {t('onboarding.getStarted')}
          </Button>
        </CardFooter>
      </Card>
    );
  };

  const renderStep2 = () => {
    return (
      <Card>
        <CardHeader>
          <CardTitle>{t('onboarding.step2.title')}</CardTitle>
          <CardDescription>{t('onboarding.step2.subtitle')}</CardDescription>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleCreateTool} className="space-y-4">
            <div className="space-y-2">
              <label className="text-sm font-medium">
                {t('onboarding.step2.nameLabel')}
                <span className="text-red-500 ml-1">*</span>
              </label>
              <Input
                value={toolData.name}
                onChange={(e) => setToolData({ ...toolData, name: e.target.value })}
                placeholder={t('onboarding.step2.namePlaceholder')}
                required
              />
            </div>
            
            <div className="space-y-2">
              <label className="text-sm font-medium">
                {t('onboarding.step2.descriptionLabel')}
                <span className="text-red-500 ml-1">*</span>
              </label>
              <Input
                value={toolData.description}
                onChange={(e) => setToolData({ ...toolData, description: e.target.value })}
                placeholder={t('onboarding.step2.descriptionPlaceholder')}
                required
              />
            </div>
            
            <div className="space-y-2">
              <label className="text-sm font-medium">{t('onboarding.step2.schemaLabel')}</label>
              <textarea
                value={toolData.schema}
                onChange={(e) => setToolData({ ...toolData, schema: e.target.value })}
                placeholder={t('onboarding.step2.schemaPlaceholder')}
                className="flex min-h-[120px] w-full rounded-md border border-border bg-background px-3 py-2 text-sm font-mono"
                rows={5}
              />
            </div>
            
            {error && (
              <div className="p-3 bg-destructive/10 border border-destructive rounded-md text-destructive text-sm">
                {error}
              </div>
            )}
          </form>
        </CardContent>
        <CardFooter className="flex justify-between">
          <Button variant="ghost" onClick={handleBack}>
            {t('onboarding.back')}
          </Button>
          <div className="flex gap-2">
            <Button variant="ghost" onClick={handleSkip}>
              {t('onboarding.skip')}
            </Button>
            <Button onClick={handleCreateTool} disabled={loading || !toolData.name || !toolData.description}>
              {loading ? t('onboarding.step2.creating') : t('onboarding.step2.createTool')}
            </Button>
          </div>
        </CardFooter>
      </Card>
    );
  };

  const renderStep3 = () => {
    return (
      <Card>
        <CardHeader className="text-center">
          <div className="text-5xl mb-4">🎉</div>
          <CardTitle className="text-3xl">{t('onboarding.step3.title')}</CardTitle>
          <CardDescription className="text-lg mt-2">
            {t('onboarding.step3.subtitle')}
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="p-4 rounded-lg border bg-muted/50">
            <h3 className="font-semibold mb-2">{t('onboarding.step3.nextSteps.title')}</h3>
            <ul className="space-y-2 text-sm text-muted-foreground">
              <li className="flex items-center gap-2">
                <span className="text-primary-600">→</span>
                {t('onboarding.step3.nextSteps.exploreTools')}
              </li>
              <li className="flex items-center gap-2">
                <span className="text-primary-600">→</span>
                {t('onboarding.step3.nextSteps.readDocs')}
              </li>
              <li className="flex items-center gap-2">
                <span className="text-primary-600">→</span>
                {t('onboarding.step3.nextSteps.inviteTeam')}
              </li>
            </ul>
          </div>
        </CardContent>
        <CardFooter className="flex justify-center gap-4">
          <Button variant="outline" onClick={() => router.push('/docs')}>
            {t('onboarding.step3.viewDocs')}
          </Button>
          <Button onClick={() => router.push('/dashboard')}>
            {t('onboarding.step3.goToDashboard')}
          </Button>
        </CardFooter>
      </Card>
    );
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-background p-4">
      <div className="w-full max-w-2xl">
        {renderProgress()}
        {step === 1 && renderStep1()}
        {step === 2 && renderStep2()}
        {step === 3 && renderStep3()}
      </div>
    </div>
  );
}
