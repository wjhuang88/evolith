'use client';

import { useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { Button } from '@/components/ui/Button';
import config from '@/lib/config';
import { AlertCircle, RefreshCw } from 'lucide-react';

interface GlobalErrorProps {
  error: Error & { digest?: string };
  reset: () => void;
}

export default function GlobalError({ error, reset }: GlobalErrorProps) {
  const { t } = useTranslation();
  useEffect(() => {
    // Log error to error reporting service
    console.error('Global application error:', error);
  }, [error]);

  return (
    <html lang="en">
      <body>
        <div className="flex min-h-screen flex-col items-center justify-center bg-background px-4">
          <div className="text-center max-w-md">
            {/* Error Icon */}
            <div className="flex justify-center mb-6">
              <div className="rounded-full bg-error/10 p-4">
                <AlertCircle className="h-12 w-12 text-error" />
              </div>
            </div>

            {/* Heading */}
            <h1 className="text-3xl font-bold tracking-tight text-foreground">
               {t('errors.globalError.title')}
            </h1>

            {/* Description */}
            <p className="mt-4 text-muted-foreground">
               {t('errors.globalError.description')}
            </p>

            {/* Error Details (development only) */}
            {config.nodeEnv === 'development' && (
              <div className="mt-6 p-4 rounded-lg bg-muted text-left">
                <p className="text-sm font-mono text-muted-foreground break-all">
                  {error.message}
                </p>
                {error.digest && (
                  <p className="mt-2 text-xs text-muted-foreground">
                    Error ID: {error.digest}
                  </p>
                )}
              </div>
            )}

            {/* Action Button */}
            <div className="mt-8">
              <Button onClick={() => window.location.reload()} size="lg">
                <RefreshCw className="mr-2 h-4 w-4" />
                 {t('errors.globalError.reloadPage')}
              </Button>
            </div>

            {/* Error digest for debugging */}
            {error.digest && (
              <p className="mt-8 text-xs text-muted-foreground">
                Reference ID: {error.digest}
              </p>
            )}
          </div>
        </div>
      </body>
    </html>
  );
}
