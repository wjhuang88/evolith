'use client';

import Link from 'next/link';
import { useTranslation } from 'react-i18next';
import { useState, useEffect } from 'react';
import { Button } from '@/components/ui/Button';
import { LanguageSwitcher } from '@/components/LanguageSwitcher';

export default function HomePage() {
  const { t } = useTranslation();
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  if (!mounted) {
    return (
      <main className="flex min-h-screen flex-col items-center justify-center bg-gradient-to-b from-background to-background/80">
        <div className="text-center">
          <h1 className="text-5xl font-bold">Evolith</h1>
        </div>
      </main>
    );
  }

  return (
    <main className="flex min-h-screen flex-col items-center justify-center bg-gradient-to-b from-background to-background/80">
      {/* Language Switcher */}
      <div className="fixed top-4 right-4">
        <LanguageSwitcher />
      </div>

      <div className="container flex flex-col items-center justify-center gap-8 px-4 py-16">
        {/* Hero Section */}
        <div className="text-center">
          <h1 className="text-5xl font-bold tracking-tight text-foreground sm:text-6xl">
            {t('home.title')}
          </h1>
          <p className="mt-4 text-xl text-muted-foreground">
            {t('home.subtitle')}
          </p>
          <p className="mt-2 text-lg text-muted-foreground/80">
            {t('home.description')}
          </p>
        </div>

        {/* Features Grid */}
        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          <FeatureCard
            title={t('nav.tools')}
            description={t('tools.subtitle')}
            href="/tools"
          />
          <FeatureCard
            title={t('nav.skills')}
            description={t('skills.subtitle')}
            href="/skills"
          />
          <FeatureCard
            title={t('nav.snippets')}
            description={t('snippets.subtitle')}
            href="/snippets"
          />
        </div>

        {/* CTA Section */}
        <div className="flex gap-4">
          <Link href="/tools">
            <Button variant="primary" size="lg">
              {t('home.getStarted')}
            </Button>
          </Link>
          <Link href="/docs">
            <Button variant="outline" size="lg">
              {t('home.viewDocs')}
            </Button>
          </Link>
        </div>
      </div>
    </main>
  );
}

function FeatureCard({
  title,
  description,
  href,
}: {
  title: string;
  description: string;
  href: string;
}) {
  return (
    <Link
      href={href}
      className="group rounded-lg border border-border bg-card p-6 transition-colors hover:border-primary hover:bg-card/80"
    >
      <h2 className="mb-2 text-xl font-semibold">{title}</h2>
      <p className="text-sm text-muted-foreground">{description}</p>
    </Link>
  );
}
