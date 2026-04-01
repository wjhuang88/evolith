import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server';

// Public paths that do NOT require authentication
const publicPaths = [
  '/',
  '/login',
  '/register',
  '/forgot-password',
  '/reset-password',
  '/verify-email',
];

// Check if a path is public (no auth required)
function isPublicPath(pathname: string): boolean {
  return publicPaths.some((path) => pathname === path || pathname.startsWith(path + '/'));
}

// Check if user is authenticated via cookie
function isAuthenticated(request: NextRequest): boolean {
  // Check for auth cookie (set by server or client)
  const token = request.cookies.get('evolith_token')?.value;
  return !!token;
}

export function middleware(request: NextRequest) {
  const { pathname } = request.nextUrl;

  // Skip middleware for static files, API routes, and public paths
  if (
    pathname.startsWith('/_next') ||
    pathname.startsWith('/api') ||
    pathname.startsWith('/static') ||
    pathname.includes('.') || // Static files like .css, .js, .png, etc.
    isPublicPath(pathname)
  ) {
    return NextResponse.next();
  }

  // For protected routes, check authentication
  // Note: Middleware cannot access localStorage, so we rely on cookies
  // The AuthGuard client component will handle localStorage-based auth
  const hasAuthCookie = isAuthenticated(request);

  if (!hasAuthCookie) {
    // Redirect to login with the original URL as redirect param
    const loginUrl = new URL('/login', request.url);
    loginUrl.searchParams.set('redirect', pathname);
    return NextResponse.redirect(loginUrl);
  }

  return NextResponse.next();
}

// Configure which routes the middleware runs on
export const config = {
  matcher: [
    /*
     * Match all request paths except:
     * - _next/static (static files)
     * - _next/image (image optimization files)
     * - favicon.ico (favicon file)
     * - public files (e.g., .png, .jpg, .svg, etc.)
     * - API routes
     */
    '/((?!_next/static|_next/image|favicon.ico|api|.*\\..*).*)',
  ],
};
