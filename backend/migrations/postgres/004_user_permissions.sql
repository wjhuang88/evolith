-- User permissions and email verification migration
-- Adds email verification, password reset, and API key support
-- PostgreSQL-native version

-- ============================================
-- Add verification and reset fields to users
-- ============================================

ALTER TABLE users ADD COLUMN email_verified BOOLEAN DEFAULT FALSE;
ALTER TABLE users ADD COLUMN verify_token TEXT;
ALTER TABLE users ADD COLUMN verified_at TIMESTAMPTZ;
ALTER TABLE users ADD COLUMN reset_token TEXT;
ALTER TABLE users ADD COLUMN reset_expires_at TIMESTAMPTZ;

-- ============================================
-- API Keys table
-- ============================================

CREATE TABLE IF NOT EXISTS api_keys (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    user_id UUID NOT NULL REFERENCES users(id),
    name TEXT NOT NULL,
    key_hash TEXT NOT NULL,
    key_prefix TEXT NOT NULL,
    permissions JSONB DEFAULT '["read"]'::jsonb,
    expires_at TIMESTAMPTZ,
    last_used_at TIMESTAMPTZ,
    rate_limit INTEGER DEFAULT 1000,
    status TEXT DEFAULT 'active',
    request_count INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================
-- Audit logs table
-- ============================================

CREATE TABLE IF NOT EXISTS audit_logs (
    id UUID PRIMARY KEY,
    tenant_id UUID REFERENCES tenants(id),
    user_id UUID REFERENCES users(id),
    action TEXT NOT NULL,
    resource_type TEXT,
    resource_id UUID,
    details JSONB DEFAULT '{}'::jsonb,
    ip_address TEXT,
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================
-- Email verification tokens table
-- ============================================

CREATE TABLE IF NOT EXISTS email_verification_tokens (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    token TEXT UNIQUE NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================
-- Password reset tokens table
-- ============================================

CREATE TABLE IF NOT EXISTS password_reset_tokens (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    token TEXT UNIQUE NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================
-- Indexes
-- ============================================

CREATE INDEX IF NOT EXISTS idx_api_keys_tenant ON api_keys(tenant_id);
CREATE INDEX IF NOT EXISTS idx_api_keys_prefix ON api_keys(key_prefix);
CREATE INDEX IF NOT EXISTS idx_api_keys_user ON api_keys(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_tenant ON audit_logs(tenant_id, created_at);
CREATE INDEX IF NOT EXISTS idx_audit_user ON audit_logs(user_id, created_at);
CREATE INDEX IF NOT EXISTS idx_audit_action ON audit_logs(action);
CREATE INDEX IF NOT EXISTS idx_email_tokens_user ON email_verification_tokens(user_id);
CREATE INDEX IF NOT EXISTS idx_email_tokens_token ON email_verification_tokens(token);
CREATE INDEX IF NOT EXISTS idx_password_tokens_user ON password_reset_tokens(user_id);
CREATE INDEX IF NOT EXISTS idx_password_tokens_token ON password_reset_tokens(token);
