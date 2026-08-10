import {
  Link as RRLink,
  useParams as useRRParams,
  useLocation,
  useNavigate,
  useSearchParams as useRRSearchParams,
} from 'react-router-dom';
import { useMemo } from 'react';

export const Link = RRLink;

export function useRouter() {
  const navigate = useNavigate();
  return useMemo(() => ({
    push: (href: string) => navigate(href),
    replace: (href: string) => navigate(href, { replace: true }),
    back: () => navigate(-1),
    forward: () => navigate(1),
    refresh: () => navigate(0),
    prefetch: () => {},
  }), [navigate]);
}

export function usePathname() {
  const { pathname } = useLocation();
  return pathname;
}

export function useRouteLocation() {
  const { pathname, search, hash } = useLocation();
  return { pathname, search, hash };
}

export function useParams<T extends Record<string, string | string[]> = Record<string, string | string[]>>() {
  return useRRParams() as T;
}

export function useSearchParams() {
  const [searchParams, setSearchParams] = useRRSearchParams();
  return [searchParams, setSearchParams] as const;
}
