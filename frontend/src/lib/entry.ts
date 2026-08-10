import { reposApi } from '@/lib/api/repos';
import { entryForRepoCount, safeInternalRedirect } from '@/lib/entry-policy';

export { safeInternalRedirect } from '@/lib/entry-policy';

export async function resolveDefaultAuthenticatedEntry(tenantId: string): Promise<string> {
  const repos = await reposApi.list(tenantId);
  return entryForRepoCount(repos.length);
}

export async function resolveAuthenticatedEntry(
  tenantId: string,
  candidateRedirect: string | null = null
): Promise<string> {
  const redirect = safeInternalRedirect(candidateRedirect);
  return redirect ?? resolveDefaultAuthenticatedEntry(tenantId);
}
