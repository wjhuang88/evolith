'use client';

import NextLink from 'next/link';
import {
  useParams as useNextParams,
  usePathname as useNextPathname,
  useRouter as useNextRouter,
  useSearchParams as useNextSearchParams,
} from 'next/navigation';

export const Link = NextLink;

export function useRouter() {
  return useNextRouter();
}

export function usePathname() {
  return useNextPathname();
}

export function useParams<T extends Record<string, string | string[]> = Record<string, string | string[]>>() {
  return useNextParams<T>();
}

export function useSearchParams() {
  return useNextSearchParams();
}
