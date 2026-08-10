'use client';

import { useEffect, useState } from 'react';
import { useParams, useRouter } from '@/lib/router';
import { useTranslation } from 'react-i18next';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';
import { Button } from '@/components/ui';
import { skillsApi } from '@/lib/api/skills';
import type { Skill } from '@/lib/api/types';

interface ExecutionResult {
  status: string;
  skill_id: string;
  skill_name: string;
  runtime: string;
  output: string;
  execution_time_ms: number;
}

export default function SkillDetailPage() {
  const { t } = useTranslation();
  const params = useParams();
  const router = useRouter();
  const skillId = params.id as string;
  
  const [skill, setSkill] = useState<Skill | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  
  // Execution state
  const [executeModalOpen, setExecuteModalOpen] = useState(false);
  const [argumentsJson, setArgumentsJson] = useState('{}');
  const [executing, setExecuting] = useState(false);
  const [executionResult, setExecutionResult] = useState<ExecutionResult | null>(null);
  const [executionError, setExecutionError] = useState<string | null>(null);

  useEffect(() => {
    async function fetchSkill() {
      try {
        const response = await skillsApi.get(skillId);
        if (response.success && response.data) {
          setSkill(response.data);
        } else {
          setError(response.error?.message || t('skills.skillNotFound'));
        }
      } catch (err) {
        setError(t('skills.failedToLoadSkill'));
      } finally {
        setLoading(false);
      }
    }
    fetchSkill();
  }, [skillId]);

  const handleExecute = async () => {
    setExecuting(true);
    setExecutionError(null);
    setExecutionResult(null);
    
    let parsedArgs: Record<string, unknown>;
    try {
      parsedArgs = JSON.parse(argumentsJson);
    } catch {
      setExecutionError(t('skills.invalidJson'));
      setExecuting(false);
      return;
    }
    
    try {
      const response = await skillsApi.execute(skillId, parsedArgs);
      if (response.success && response.data) {
        setExecutionResult(response.data as ExecutionResult);
      } else {
        setExecutionError(response.error?.message || t('skills.executionFailed'));
      }
    } catch (err) {
      setExecutionError(t('skills.failedToExecute'));
    } finally {
      setExecuting(false);
    }
  };

  const handleDelete = async () => {
    if (!confirm(t('skills.confirmDelete'))) return;
    
    try {
      const response = await skillsApi.delete(skillId);
      if (response.success) {
        router.push('/skills');
      } else {
        alert(response.error?.message || t('skills.failedToDelete'));
      }
    } catch (err) {
      alert(t('skills.failedToDeleteSkill'));
    }
  };

  if (loading) {
    return (
      <div className="container mx-auto py-8">
        <div className="text-center text-muted-foreground">{t('common.loading')}</div>
      </div>
    );
  }

  if (error || !skill) {
    return (
      <div className="container mx-auto py-8">
        <div className="text-center text-destructive">{error || t('skills.skillNotFound')}</div>
        <Button className="mt-4" onClick={() => router.push('/skills')}>{t('skills.backToSkills')}</Button>
      </div>
    );
  }

  return (
    <div className="container mx-auto py-8 max-w-4xl">
      <Button variant="ghost" onClick={() => router.push('/skills')} className="mb-4">
        ← {t('skills.backToSkills')}
      </Button>

      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-3xl font-bold flex items-center gap-2">
            {skill.name}
            {!skill.is_public && (
              <span className="text-xs bg-muted px-2 py-0.5 rounded">Private</span>
            )}
          </h1>
          <p className="text-muted-foreground mt-1">{skill.description}</p>
          <div className="flex gap-4 mt-2 text-sm text-muted-foreground">
            <span>{t('skills.version')}: {skill.version}</span>
            <span>{t('skills.category')}: {skill.category}</span>
            <span>Runtime: javascript</span>
          </div>
        </div>
        <div className="flex gap-2">
          <Button onClick={() => setExecuteModalOpen(true)}>{t('skills.execute')}</Button>
          <Button variant="destructive" onClick={handleDelete}>{t('skills.deleteSkill')}</Button>
        </div>
      </div>

      <Card className="mb-6">
        <CardHeader>
          <CardTitle>{t('skills.contentTitle')}</CardTitle>
        </CardHeader>
        <CardContent>
          <pre className="bg-muted p-4 rounded-md overflow-auto text-sm whitespace-pre-wrap max-h-96">
            {skill.content}
          </pre>
        </CardContent>
      </Card>

      {/* Execute Modal */}
      {executeModalOpen && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
          <Card className="w-full max-w-2xl max-h-[90vh] overflow-auto">
            <CardHeader>
              <CardTitle>{t('skills.executeSkill', { name: skill.name })}</CardTitle>
              <CardDescription>{t('skills.enterArgsJson')}</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div>
                <label className="text-sm font-medium">{t('skills.argsLabel')}</label>
                <textarea
                  value={argumentsJson}
                  onChange={(e) => setArgumentsJson(e.target.value)}
                  className="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm font-mono min-h-[120px]"
                  placeholder='{"key": "value"}'
                />
              </div>
              
              {executionError && (
                <div className="p-4 bg-destructive/10 border border-destructive rounded-md text-destructive text-sm">
                  {executionError}
                </div>
              )}
              
              {executionResult && (
                <div className="space-y-4">
                  <div className="flex gap-4 text-sm text-muted-foreground">
                    <span>{t('skills.statusLabel')}: <span className={executionResult.status === 'success' ? 'text-green-500' : 'text-red-500'}>{executionResult.status}</span></span>
                    <span>{t('skills.runtimeLabel')}: {executionResult.runtime}</span>
                    <span>{t('skills.timeLabel')}: {executionResult.execution_time_ms}ms</span>
                  </div>
                  <div>
                    <label className="text-sm font-medium">{t('skills.output')}</label>
                    <pre className="bg-muted p-4 rounded-md overflow-auto text-sm whitespace-pre-wrap max-h-64">
                      {executionResult.output}
                    </pre>
                  </div>
                </div>
              )}
              
              <div className="flex gap-2 justify-end">
                <Button variant="outline" onClick={() => {
                  setExecuteModalOpen(false);
                  setExecutionResult(null);
                  setExecutionError(null);
                }}>
                  Close
                </Button>
                <Button onClick={handleExecute} disabled={executing}>
                  {executing ? t('skills.executing') : t('skills.execute')}
                </Button>
              </div>
            </CardContent>
          </Card>
        </div>
      )}
    </div>
  );
}
