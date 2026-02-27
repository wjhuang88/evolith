'use client';

import { useTheme } from '@/lib/theme';
import { Moon, Sun, Monitor } from 'lucide-react';
import { Button } from '@/components/ui/Button';

export function ThemeToggle({ className }: { className?: string }) {
  const { theme, setTheme, resolvedTheme } = useTheme();

  const cycleTheme = () => {
    if (theme === 'light') {
      setTheme('dark');
    } else if (theme === 'dark') {
      setTheme('system');
    } else {
      setTheme('light');
    }
  };

  const Icon = resolvedTheme === 'dark' ? Moon : theme === 'system' ? Monitor : Sun;

  return (
    <Button
      variant="ghost"
      size="sm"
      onClick={cycleTheme}
      className={className}
      aria-label={`Current theme: ${theme}. Click to change.`}
    >
      <Icon className="h-4 w-4" />
    </Button>
  );
}
