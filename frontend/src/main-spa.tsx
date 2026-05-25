import React from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
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
import SnippetsPage from './app/snippets/page';
import SnippetNewPage from './app/snippets/new/page';
import SnippetDetailPage from './app/snippets/[id]/page';
import TenantSettingsPage from './app/tenant/settings/page';
import TenantMembersPage from './app/tenant/members/page';
import TenantBillingPage from './app/tenant/billing/page';
import TenantApiKeysPage from './app/tenant/api-keys/page';

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
            <Route path="/onboarding" element={<OnboardingPage />} />
            <Route path="/verify-email" element={<VerifyEmailPage />} />
            <Route path="/dashboard" element={<DashboardPage />} />
            <Route path="/profile" element={<ProfilePage />} />
            <Route path="/tools" element={<ToolsPage />} />
            <Route path="/tools/new" element={<ToolNewPage />} />
            <Route path="/tools/:id" element={<ToolDetailPage />} />
            <Route path="/skills" element={<SkillsPage />} />
            <Route path="/skills/new" element={<SkillNewPage />} />
            <Route path="/skills/:id" element={<SkillDetailPage />} />
            <Route path="/snippets" element={<SnippetsPage />} />
            <Route path="/snippets/new" element={<SnippetNewPage />} />
            <Route path="/snippets/:id" element={<SnippetDetailPage />} />
            <Route path="/tenant/settings" element={<TenantSettingsPage />} />
            <Route path="/tenant/members" element={<TenantMembersPage />} />
            <Route path="/tenant/billing" element={<TenantBillingPage />} />
            <Route path="/tenant/api-keys" element={<TenantApiKeysPage />} />
            <Route path="*" element={<NotFound />} />
          </Routes>
        </LayoutWrapper>
      </Providers>
    </BrowserRouter>
  </React.StrictMode>
);
