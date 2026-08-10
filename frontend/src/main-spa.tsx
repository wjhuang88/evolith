import React from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { Providers } from './app/providers';
import { LayoutWrapper } from './components/layout/LayoutWrapper';
import '@/styles/globals.css';

import LandingPage from './app/page';
import DashboardPage from './app/dashboard/page';
import LoginPage from './app/login/page';
import RegisterPage from './app/register/page';
import ForgotPasswordPage from './app/forgot-password/page';
import ResetPasswordPage from './app/reset-password/page';
import JoinPage from './app/join/page';
import OnboardingPage from './app/onboarding/page';
import ProfilePage from './app/profile/page';
import VerifyEmailPage from './app/verify-email/page';
import ToolsPage from './app/tools/page';
import ToolNewPage from './app/tools/new/page';
import ToolDetailPage from './app/tools/[id]/page';
import SkillsPage from './app/skills/page';
import SkillNewPage from './app/skills/new/page';
import SkillDetailPage from './app/skills/[id]/page';
import InterfacesPage from './app/interfaces/page';
import InterfaceNewPage from './app/interfaces/new/page';
import InterfaceDetailPage from './app/interfaces/[id]/page';
import TenantSettingsPage from './app/tenant/settings/page';
import TenantMembersPage from './app/tenant/members/page';
import TenantBillingPage from './app/tenant/billing/page';
import TenantApiKeysPage from './app/tenant/api-keys/page';
import { SettingsAccessGuard, SettingsLayout } from './app/settings/layout';
import ReposPage from './app/repos/page';
import NewRepoPage from './app/repos/new/page';
import RepoDetailPage from './app/repos/[id]/page';
import CommitEvidencePage from './app/repos/[id]/commits/[sha]/page';
import { AuthGuard } from './components/AuthGuard';

function NotFound() {
  return (
    <div className="flex min-h-screen items-center justify-center">
      <div className="text-center">
        <h1 className="text-4xl font-bold">404</h1>
        <p className="mt-2 text-muted-foreground">Page not found</p>
      </div>
    </div>
  );
}

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <BrowserRouter>
      <Providers>
        <LayoutWrapper>
          <Routes>
            <Route path="/" element={<LandingPage />} />
            <Route path="/login" element={<LoginPage />} />
            <Route path="/register" element={<RegisterPage />} />
            <Route path="/forgot-password" element={<ForgotPasswordPage />} />
            <Route path="/reset-password" element={<ResetPasswordPage />} />
            <Route path="/join" element={<JoinPage />} />
            <Route path="/accept-invitation" element={<JoinPage />} />
            <Route path="/onboarding" element={<AuthGuard><OnboardingPage /></AuthGuard>} />
            <Route path="/verify-email" element={<VerifyEmailPage />} />
            <Route path="/dashboard" element={<DashboardPage />} />
            <Route path="/repos" element={<ReposPage />} />
            <Route path="/repos/new" element={<NewRepoPage />} />
            <Route path="/repos/:id" element={<RepoDetailPage />} />
            <Route path="/repos/:id/commits/:sha" element={<CommitEvidencePage />} />
            <Route path="/repos/:id/:tab" element={<RepoDetailPage />} />
            <Route path="/tools" element={<ToolsPage />} />
            <Route path="/tools/new" element={<ToolNewPage />} />
            <Route path="/tools/:id" element={<ToolDetailPage />} />
            <Route path="/skills" element={<SkillsPage />} />
            <Route path="/skills/new" element={<SkillNewPage />} />
            <Route path="/skills/:id" element={<SkillDetailPage />} />
            <Route path="/interfaces" element={<InterfacesPage />} />
            <Route path="/interfaces/new" element={<InterfaceNewPage />} />
            <Route path="/interfaces/:id" element={<InterfaceDetailPage />} />
            <Route path="/snippets" element={<Navigate to="/interfaces" replace />} />
            <Route path="/snippets/new" element={<Navigate to="/interfaces/new" replace />} />
            <Route path="/snippets/:id" element={<Navigate to="/interfaces/:id" replace />} />
            <Route path="/settings" element={<SettingsLayout />}>
              <Route index element={<Navigate to="/settings/profile" replace />} />
              <Route path="profile" element={<SettingsAccessGuard section="profile"><ProfilePage /></SettingsAccessGuard>} />
              <Route path="workspace" element={<SettingsAccessGuard section="workspace"><TenantSettingsPage /></SettingsAccessGuard>} />
              <Route path="members" element={<SettingsAccessGuard section="members"><TenantMembersPage /></SettingsAccessGuard>} />
              <Route path="api-keys" element={<SettingsAccessGuard section="api-keys"><TenantApiKeysPage /></SettingsAccessGuard>} />
              <Route path="billing" element={<SettingsAccessGuard section="billing"><TenantBillingPage /></SettingsAccessGuard>} />
            </Route>
            <Route path="*" element={<NotFound />} />
          </Routes>
        </LayoutWrapper>
      </Providers>
    </BrowserRouter>
  </React.StrictMode>
);
