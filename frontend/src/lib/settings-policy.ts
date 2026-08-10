export type SettingsRole = 'owner' | 'admin' | 'member';

export type SettingsSection =
  | 'profile'
  | 'workspace'
  | 'members'
  | 'api-keys'
  | 'billing';

export interface SettingsNavigationItem {
  section: SettingsSection;
  href: string;
  labelKey: string;
}

export const settingsNavigation: SettingsNavigationItem[] = [
  { section: 'profile', href: '/settings/profile', labelKey: 'settings.sections.profile' },
  { section: 'workspace', href: '/settings/workspace', labelKey: 'settings.sections.workspace' },
  { section: 'members', href: '/settings/members', labelKey: 'settings.sections.members' },
  { section: 'api-keys', href: '/settings/api-keys', labelKey: 'settings.sections.apiKeys' },
  { section: 'billing', href: '/settings/billing', labelKey: 'settings.sections.billing' },
];

const accessBySection: Record<SettingsSection, readonly SettingsRole[]> = {
  profile: ['owner', 'admin', 'member'],
  workspace: ['owner', 'admin'],
  members: ['owner', 'admin', 'member'],
  'api-keys': ['owner', 'admin'],
  billing: ['owner'],
};

export function canAccessSettingsSection(
  role: SettingsRole,
  section: SettingsSection
): boolean {
  return accessBySection[section].includes(role);
}

export function settingsNavigationFor(role: SettingsRole): SettingsNavigationItem[] {
  return settingsNavigation.filter((item) => canAccessSettingsSection(role, item.section));
}
