import { describe, expect, test } from 'bun:test';
import {
  canAccessSettingsSection,
  settingsNavigationFor,
  type SettingsRole,
  type SettingsSection,
} from '../src/lib/settings-policy';

const expectedByRole: Record<SettingsRole, SettingsSection[]> = {
  member: ['profile', 'members'],
  admin: ['profile', 'workspace', 'members', 'api-keys'],
  owner: ['profile', 'workspace', 'members', 'api-keys', 'billing'],
};

describe('settings authorization policy', () => {
  for (const role of ['member', 'admin', 'owner'] as const) {
    test(`${role} sees only authorized settings entries`, () => {
      expect(settingsNavigationFor(role).map((item) => item.section)).toEqual(expectedByRole[role]);
    });
  }

  test('direct route checks use the same matrix as navigation visibility', () => {
    for (const role of ['member', 'admin', 'owner'] as const) {
      for (const section of ['profile', 'workspace', 'members', 'api-keys', 'billing'] as const) {
        expect(canAccessSettingsSection(role, section)).toBe(
          expectedByRole[role].includes(section)
        );
      }
    }
  });
});
