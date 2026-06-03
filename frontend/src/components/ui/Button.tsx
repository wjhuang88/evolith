import { cn } from '@/lib/utils';

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'outline' | 'ghost' | 'destructive';
  size?: 'sm' | 'md' | 'lg';
}

export function Button({
  className,
  variant = 'primary',
  size = 'md',
  children,
  ...props
}: ButtonProps) {
  return (
    <button
      className={cn(
        'inline-flex items-center justify-center rounded-[50px] font-medium transition-colors',
        'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2',
        'disabled:pointer-events-none disabled:opacity-50',
        variant === 'primary' && 'bg-[var(--primary)] text-[var(--primary-foreground)] hover:opacity-90',
        variant === 'secondary' && 'bg-[var(--secondary)] text-[var(--secondary-foreground)] hover:opacity-90',
        variant === 'outline' &&
          'border border-[var(--border)] bg-transparent hover:bg-[var(--muted)] text-[var(--foreground)]',
        variant === 'ghost' && 'hover:bg-[var(--muted)] hover:text-[var(--foreground)]',
        variant === 'destructive' && 'bg-red-600 text-white hover:bg-red-700',
        size === 'sm' && 'h-8 px-4 text-sm',
        size === 'md' && 'h-10 px-5 text-sm',
        size === 'lg' && 'h-12 px-6 text-base',
        className
      )}
      {...props}
    >
      {children}
    </button>
  );
}
