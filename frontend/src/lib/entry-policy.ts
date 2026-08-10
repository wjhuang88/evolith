export function routeTarget(location: {
  pathname: string;
  search?: string;
  hash?: string;
}): string {
  return `${location.pathname}${location.search ?? ''}${location.hash ?? ''}`;
}

export function safeInternalRedirect(
  candidate: string | null,
  origin = window.location.origin
): string | null {
  if (!candidate || !candidate.startsWith('/') || candidate.startsWith('//')) {
    return null;
  }

  try {
    const target = new URL(candidate, origin);
    const normalizedPath = decodeURIComponent(target.pathname)
      .replace(/\/+$/, '')
      .toLowerCase();
    if (target.origin !== origin || normalizedPath === '/login') {
      return null;
    }
    return routeTarget(target);
  } catch {
    return null;
  }
}

export function loginHrefFor(target: string, origin = window.location.origin): string {
  const redirect = safeInternalRedirect(target, origin);
  return redirect
    ? `/login?${new URLSearchParams({ redirect }).toString()}`
    : '/login';
}

export function hrefWithRedirect(
  pathname: string,
  candidate: string | null,
  origin = window.location.origin
): string {
  const redirect = safeInternalRedirect(candidate, origin);
  return redirect
    ? `${pathname}?${new URLSearchParams({ redirect }).toString()}`
    : pathname;
}

export function entryForRepoCount(repoCount: number): '/onboarding' | '/dashboard' {
  return repoCount === 0 ? '/onboarding' : '/dashboard';
}
