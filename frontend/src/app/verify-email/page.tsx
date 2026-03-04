'use client';

import { useState, useEffect } from 'react';
import Link from 'next/link';
import { useRouter, useSearchParams } from 'next/navigation';
import { Button } from '@/components/ui/Button';

export default function VerifyEmailPage() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const token = searchParams.get('token');

  const [status, setStatus] = useState<'loading' | 'success' | 'error'>('loading');
  const [message, setMessage] = useState('Verifying your email...');

  useEffect(() => {
    if (!token) {
      setStatus('error');
      setMessage('Invalid verification link. Please check your email or request a new verification link.');
      return;
    }

    verifyEmail(token);
  }, [token]);

  const verifyEmail = async (token: string) => {
    try {
      const response = await fetch('/api/v1/auth/verify-email', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ token }),
      });

      const data = await response.json();

      if (data.success) {
        setStatus('success');
        setMessage('Your email has been verified successfully! You can now log in to your account.');
      } else {
        setStatus('error');
        setMessage(data.error?.message || 'Verification failed. The link may have expired or is invalid.');
      }
    } catch {
      setStatus('error');
      setMessage('An error occurred during verification. Please try again later.');
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
          <h1 className="mt-6 text-2xl font-bold text-foreground">
            {status === 'loading' && 'Verifying Email'}
            {status === 'success' && 'Email Verified!'}
            {status === 'error' && 'Verification Failed'}
          </h1>
        </div>

        <div className="rounded-lg border border-border bg-card p-6 shadow-sm">
          <div className="text-center">
            {status === 'loading' && (
              <div className="mb-4">
                <div className="inline-block h-8 w-8 animate-spin rounded-full border-4 border-solid border-primary border-r-transparent" />
              </div>
            )}

            {status === 'success' && (
              <div className="mb-4 rounded-full bg-green-100 p-3 dark:bg-green-900/20">
                <svg
                  className="mx-auto h-8 w-8 text-green-600 dark:text-green-400"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 13l4 4L19 7"
                  />
                </svg>
              </div>
            )}

            {status === 'error' && (
              <div className="mb-4 rounded-full bg-red-100 p-3 dark:bg-red-900/20">
                <svg
                  className="mx-auto h-8 w-8 text-red-600 dark:text-red-400"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M6 18L18 6M6 6l12 12"
                  />
                </svg>
              </div>
            )}

            <p className="mb-6 text-muted-foreground">{message}</p>

            {status === 'success' && (
              <Button onClick={() => router.push('/login')} className="w-full">
                Go to Login
              </Button>
            )}

            {status === 'error' && (
              <div className="space-y-3">
                <Button onClick={() => router.push('/login')} variant="outline" className="w-full">
                  Go to Login
                </Button>
                <p className="text-sm text-muted-foreground">
                  Need a new verification link?{' '}
                  <Link href="/login" className="text-primary hover:underline">
                    Log in to resend
                  </Link>
                </p>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
