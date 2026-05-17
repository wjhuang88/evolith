'use client';

import { useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { Button } from '@/components/ui/Button';
import config from '@/lib/config';
import { AlertTriangle, RefreshCcw } from 'lucide-react';

interface ErrorProps {
  error: Error & { digest?: string };
  reset: () => void;
}

export default function Error({ error, reset }: ErrorProps) {
  const { t } = useTranslation();
  useEffect(() => {
    // Log error to error reporting service
    console.error('Application error:', error);
  }, [error]);

  return (
    <div className="flex min-h-screen flex-col items-center justify-center bg-background px-4">
      <div className="text-center max-w-md">
        {/* Error Icon */}
        <div className="flex justify-center mb-6">
          <div className="rounded-full bg-error/10 p-4">
            <AlertTriangle className="h-12 w-12 text-error" />
          </div>
        </div>

        {/* Heading */}
        <h1 className="text-3xl font-bold tracking-tight text-foreground">
          {t('errors.somethingWentWrong')}
        </h1>

        {/* Description */}
        <p className="mt-4 text-muted-foreground">
          {t('errors.errorDescription')}
        </p>

        {/* Error Details (for debugging) */}
        {config.nodeEnv === 'development' && (
          <div className="mt-6 p-4 rounded-lg bg-muted text-left">
            <p className="text-sm font-mono text-muted-foreground break-all">
              {error.message}
            </p>
            {error.digest && (
              <p className="mt-2 text-xs text-muted-foreground">
                 {t('errors.errorId', { id: error.digest })}
              </p>
            )}
          </div>
        )}

        {/* Action Buttons */}
        <div className="mt-8 flex flex-col sm:flex-row gap-4 justify-center">
          <Button onClick={reset} size="lg" className="w-full sm:w-auto">
            <RefreshCcw className="mr-2 h-4 w-4" />
            {t('errors.tryAgain')}
          </Button>
        </div>

        {/* Error digest for debugging (always shown, but small) */}
        {error.digest && (
          <p className="mt-8 text-xs text-muted-foreground">
             {t('errors.referenceId', { id: error.digest })}
          </p>
        )}
      </div>
    </div>
  );
}
