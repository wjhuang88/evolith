import * as React from 'react';
import { cn } from '@/lib/utils';

// ============================================
// Form Context
// ============================================

interface FormContextValue {
  errors: Record<string, string>;
}

const FormContext = React.createContext<FormContextValue | null>(null);

function useFormContext() {
  const context = React.useContext(FormContext);
  if (!context) {
    throw new Error('Form components must be used within a Form');
  }
  return context;
}

// ============================================
// Form Component
// ============================================

interface FormProps extends React.FormHTMLAttributes<HTMLFormElement> {
  errors?: Record<string, string>;
  onSubmit?: (e: React.FormEvent<HTMLFormElement>) => void;
}

export function Form({ className, errors = {}, onSubmit, children, ...props }: FormProps) {
  const [formErrors, setFormErrors] = React.useState<Record<string, string>>(errors);

  React.useEffect(() => {
    setFormErrors(errors);
  }, [errors]);

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    onSubmit?.(e);
  };

  return (
    <FormContext.Provider value={{ errors: formErrors }}>
      <form
        className={cn('space-y-4', className)}
        onSubmit={handleSubmit}
        {...props}
      >
        {children}
      </form>
    </FormContext.Provider>
  );
}

// ============================================
// FormField Component
// ============================================

interface FormFieldProps {
  name: string;
  children: React.ReactNode;
}

export function FormField({ name, children }: FormFieldProps) {
  const { errors } = useFormContext();
  const error = errors[name];

  return (
    <div className="space-y-2">
      {React.Children.map(children, (child) => {
        if (React.isValidElement(child)) {
          return React.cloneElement(child as React.ReactElement<any>, { name, error });
        }
        return child;
      })}
      {error && <p className="text-sm text-red-500">{error}</p>}
    </div>
  );
}

// ============================================
// FormLabel Component
// ============================================

interface FormLabelProps extends React.LabelHTMLAttributes<HTMLLabelElement> {
  required?: boolean;
}

export const FormLabel = React.forwardRef<HTMLLabelElement, FormLabelProps>(
  ({ className, required, children, ...props }, ref) => {
    return (
      <label
        ref={ref}
        className={cn(
          'text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70',
          className
        )}
        {...props}
      >
        {children}
        {required && <span className="text-red-500 ml-1">*</span>}
      </label>
    );
  }
);
FormLabel.displayName = 'FormLabel';

// ============================================
// FormControl Component
// ============================================

interface FormControlProps {
  children: React.ReactNode;
}

export function FormControl({ children }: FormControlProps) {
  return <div>{children}</div>;
}

// ============================================
// FormMessage Component
// ============================================

interface FormMessageProps {
  error?: string;
  success?: string;
  className?: string;
}

export function FormMessage({ error, success, className }: FormMessageProps) {
  if (!error && !success) return null;

  return (
    <p
      className={cn(
        'text-sm',
        error ? 'text-red-500' : 'text-green-500',
        className
      )}
    >
      {error || success}
    </p>
  );
}

// ============================================
// FormGroup Component (for inline fields)
// ============================================

interface FormGroupProps {
  children: React.ReactNode;
  className?: string;
}

export function FormGroup({ children, className }: FormGroupProps) {
  return <div className={cn('grid gap-4', className)}>{children}</div>;
}

// ============================================
// FormActions Component (submit buttons)
// ============================================

interface FormActionsProps {
  children: React.ReactNode;
  className?: string;
}

export function FormActions({ children, className }: FormActionsProps) {
  return (
    <div className={cn('flex items-center gap-3 pt-2', className)}>
      {children}
    </div>
  );
}
