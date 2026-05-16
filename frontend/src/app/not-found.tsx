'use client';
import { Link } from '@/lib/router';
import { Home, ArrowLeft } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import { Button } from '@/components/ui/Button';

export default function NotFound() {
  const { t } = useTranslation();
  return (
    <div className="flex min-h-screen flex-col items-center justify-center bg-background px-4">
      <div className="text-center">
        {/* 404 Heading */}
        <h1 className="text-9xl font-bold text-primary-600 dark:text-primary-400">{t('errors.notFound.title')}</h1>
        
        {/* Subtitle */}
        <h2 className="mt-4 text-3xl font-semibold tracking-tight text-foreground sm:text-4xl">
          {t('errors.notFound.subtitle')}
        </h2>
        
        {/* Description */}
        <p className="mt-4 text-lg text-muted-foreground max-w-md mx-auto">
          {t('errors.notFound.description')}
        </p>

        {/* Action Buttons */}
        <div className="mt-8 flex flex-col sm:flex-row gap-4 justify-center">
          <Link href="/">
            <Button variant="outline" size="lg" className="w-full sm:w-auto">
              <ArrowLeft className="mr-2 h-4 w-4" />
               {t('errors.notFound.goBack')}
            </Button>
          </Link>
          <Link href="/dashboard">
            <Button size="lg" className="w-full sm:w-auto">
              <Home className="mr-2 h-4 w-4" />
               {t('errors.notFound.goToDashboard')}
            </Button>
          </Link>
        </div>

        {/* Decorative Element */}
        <div className="mt-16 relative">
          <div className="absolute inset-0 flex items-center justify-center">
            <div className="w-32 h-32 rounded-full bg-primary-100 dark:bg-primary-900/20 blur-3xl" />
          </div>
          <div className="relative text-6xl opacity-20">🔍</div>
        </div>
      </div>
    </div>
  );
}
