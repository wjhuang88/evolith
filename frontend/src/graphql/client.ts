import {
  GraphQLClient,
  type RequestOptions,
  type Variables,
} from 'graphql-request';

import { getToken, isTokenExpired } from '@/lib/api/client';

const GRAPHQL_ENDPOINT = '/graphql';

function getCsrfToken(): string | null {
  if (typeof document === 'undefined') return null;
  const match = document.cookie.match(new RegExp('(^| )csrf_token=([^;]+)'));
  return match ? decodeURIComponent(match[2]) : null;
}

function requestHeaders(): Record<string, string> {
  const headers: Record<string, string> = {
    'X-Request-ID': crypto.randomUUID(),
  };
  const token = getToken();
  if (token && !isTokenExpired()) {
    headers.Authorization = `Bearer ${token}`;
  }
  const csrfToken = getCsrfToken();
  if (csrfToken) {
    headers['X-CSRF-Token'] = csrfToken;
  }
  return headers;
}

const client = new GraphQLClient(GRAPHQL_ENDPOINT, {
  credentials: 'include',
  headers: requestHeaders,
});

export function executeGraphQL<TResult, TVariables extends Variables>(
  options: RequestOptions<TVariables, TResult>,
): Promise<TResult> {
  return client.request<TResult, TVariables>(options);
}
