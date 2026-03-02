'use client';

import { useState } from 'react';
import Link from 'next/link';
import { useRouter } from 'next/navigation';
import { useAuthStore } from '@/stores';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';

export default function RegisterPage() {
  const router = useRouter();
  const { register, isLoading, error, clearError } = useAuthStore();
  const [formData, setFormData] = useState({
    email: '',
    username: '',
    password: '',
    confirmPassword: '',
    tenantName: '',
    tenantSlug: '',
  });
  const [validationError, setValidationError] = useState('');

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({ ...prev, [name]: value }));
    clearError();
    setValidationError('');
  };

  const validateForm = () => {
    if (!formData.email || !formData.username || !formData.password) {
      return 'Please fill in all required fields';
    }
    if (formData.password !== formData.confirmPassword) {
      return 'Passwords do not match';
    }
    if (formData.password.length < 8) {
      return 'Password must be at least 8 characters';
    }
    if (formData.tenantSlug && !/^[a-z0-9-]+$/.test(formData.tenantSlug)) {
      return 'Tenant slug can only contain lowercase letters, numbers, and hyphens';
    }
    return '';
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    const validation = validateForm();
    if (validation) {
      setValidationError(validation);
      return;
    }
    
    try {
      await register(
        formData.email, 
        formData.username, 
        formData.password,
        formData.tenantName || undefined,
        formData.tenantSlug || undefined
      );
      router.push('/dashboard');
    } catch {
      // Error is handled in store
    }
  };

  return (
    <div className="flex min-h-screen items-center justify-center bg-background px-4 py-12">
      <div className="w-full max-w-md">
        <div className="mb-8 text-center">
          <Link href="/" className="inline-flex items-center gap-2">
            <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-gradient-to-br from-primary-500 to-purple-600">
              <span className="text-white font-bold">E</span>
            </div>
          </Link>
          <h1 className="mt-6 text-2xl font-bold text-foreground">Create your account</h1>
          <p className="mt-2 text-sm text-muted-foreground">
            Start building with Evolith today
          </p>
        </div>

        <div className="rounded-lg border border-border bg-card p-6 shadow-sm">
          <form onSubmit={handleSubmit} className="space-y-4">
            {(error || validationError) && (
              <div className="rounded-md bg-destructive/10 p-3 text-sm text-destructive">
                {error || validationError}
              </div>
            )}

            <div>
              <label htmlFor="email" className="block text-sm font-medium text-foreground">
                Email *
              </label>
              <Input
                id="email"
                name="email"
                type="email"
                value={formData.email}
                onChange={handleChange}
                placeholder="you@company.com"
                required
                className="mt-1"
              />
            </div>

            <div>
              <label htmlFor="username" className="block text-sm font-medium text-foreground">
                Username *
              </label>
              <Input
                id="username"
                name="username"
                type="text"
                value={formData.username}
                onChange={handleChange}
                placeholder="john"
                required
                className="mt-1"
              />
            </div>

            <div>
              <label htmlFor="password" className="block text-sm font-medium text-foreground">
                Password *
              </label>
              <Input
                id="password"
                name="password"
                type="password"
                value={formData.password}
                onChange={handleChange}
                placeholder="••••••••"
                required
                className="mt-1"
              />
              <p className="mt-1 text-xs text-muted-foreground">
                Minimum 8 characters
              </p>
            </div>

            <div>
              <label htmlFor="confirmPassword" className="block text-sm font-medium text-foreground">
                Confirm Password *
              </label>
              <Input
                id="confirmPassword"
                name="confirmPassword"
                type="password"
                value={formData.confirmPassword}
                onChange={handleChange}
                placeholder="••••••••"
                required
                className="mt-1"
              />
            </div>

            <hr className="my-4 border-border" />

            <div>
              <label htmlFor="tenantName" className="block text-sm font-medium text-foreground">
                Organization Name
              </label>
              <Input
                id="tenantName"
                name="tenantName"
                type="text"
                value={formData.tenantName}
                onChange={handleChange}
                placeholder="My Company"
                className="mt-1"
              />
              <p className="mt-1 text-xs text-muted-foreground">
                Optional - will be created with your account
              </p>
            </div>

            <div>
              <label htmlFor="tenantSlug" className="block text-sm font-medium text-foreground">
                Organization URL
              </label>
              <div className="mt-1 flex rounded-md">
                <span className="inline-flex items-center rounded-l-md border border-r-0 border-input bg-muted px-3 text-sm text-muted-foreground">
                  evolith.io/
                </span>
                <Input
                  id="tenantSlug"
                  name="tenantSlug"
                  type="text"
                  value={formData.tenantSlug}
                  onChange={handleChange}
                  placeholder="my-company"
                  className="rounded-l-none"
                />
              </div>
            </div>

            <Button type="submit" className="w-full" disabled={isLoading}>
              {isLoading ? 'Creating account...' : 'Create account'}
            </Button>
          </form>

          <p className="mt-4 text-center text-sm text-muted-foreground">
            Already have an account?{' '}
            <Link href="/login" className="font-medium text-primary hover:underline">
              Sign in
            </Link>
          </p>
        </div>
      </div>
    </div>
  );
}