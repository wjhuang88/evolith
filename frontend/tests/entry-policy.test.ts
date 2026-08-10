import { describe, expect, test } from 'bun:test';
import {
  entryForRepoCount,
  hrefWithRedirect,
  loginHrefFor,
  routeTarget,
  safeInternalRedirect,
} from '../src/lib/entry-policy';

const origin = 'https://evolith.example';

describe('public entry routing policy', () => {
  test('preserves path, query, and hash as one target', () => {
    expect(routeTarget({ pathname: '/repos/r1/files', search: '?ref=main', hash: '#blob' }))
      .toBe('/repos/r1/files?ref=main#blob');
    expect(loginHrefFor('/repos/r1/files?ref=main#blob', origin))
      .toBe('/login?redirect=%2Frepos%2Fr1%2Ffiles%3Fref%3Dmain%23blob');
  });

  test('accepts only same-origin internal redirects and prevents a login loop', () => {
    expect(safeInternalRedirect('/repos/r1?ref=main#readme', origin))
      .toBe('/repos/r1?ref=main#readme');
    expect(safeInternalRedirect('https://evil.example/steal', origin)).toBeNull();
    expect(safeInternalRedirect('//evil.example/steal', origin)).toBeNull();
    expect(safeInternalRedirect('/login?redirect=/dashboard', origin)).toBeNull();
    expect(safeInternalRedirect('/login/', origin)).toBeNull();
    expect(safeInternalRedirect('/LOGIN', origin)).toBeNull();
    expect(safeInternalRedirect('/%6cogin', origin)).toBeNull();
  });

  test('propagates only safe redirects between public auth pages', () => {
    expect(hrefWithRedirect('/register', '/repos/r1', origin))
      .toBe('/register?redirect=%2Frepos%2Fr1');
    expect(hrefWithRedirect('/register', 'https://evil.example', origin)).toBe('/register');
  });

  test('routes empty and non-empty repository inventories deterministically', () => {
    expect(entryForRepoCount(0)).toBe('/onboarding');
    expect(entryForRepoCount(1)).toBe('/dashboard');
  });
});
