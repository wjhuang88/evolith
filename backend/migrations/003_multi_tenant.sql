-- Multi-tenant schema migration
-- Adds tenant support to Evolith

-- ============================================
-- Tenants table
-- ============================================
CREATE TABLE IF NOT EXISTS tenants (
    id TEXT PRIMARY KEY,
    
    -- Basic info
    name TEXT NOT NULL,
    slug TEXT UNIQUE NOT NULL,
    domain TEXT UNIQUE,
    
    -- Contact info
    owner_id TEXT NOT NULL,
    billing_email TEXT,
    
    -- Subscription plan
    plan TEXT NOT NULL DEFAULT 'free',      -- free | starter | pro | enterprise
    plan_status TEXT DEFAULT 'active',      -- active | past_due | cancelled
    
    -- Quota limits
    max_users INTEGER DEFAULT 5,
    max_tools INTEGER DEFAULT 10,
    max_skills INTEGER DEFAULT 20,
    max_snippets INTEGER DEFAULT 100,
    max_api_calls_per_month INTEGER DEFAULT 10000,
    max_storage_mb INTEGER DEFAULT 500,
    
    -- Usage stats
    current_users INTEGER DEFAULT 0,
    current_tools INTEGER DEFAULT 0,
    current_skills INTEGER DEFAULT 0,
    current_snippets INTEGER DEFAULT 0,
    current_api_calls INTEGER DEFAULT 0,
    current_storage_mb INTEGER DEFAULT 0,
    
    -- Configuration (JSON stored as TEXT for SQLite compatibility)
    settings TEXT DEFAULT '{}',
    features TEXT DEFAULT '{}',
    
    -- Status
    status TEXT DEFAULT 'active',           -- active | suspended | deleted
    
    -- Timestamps
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    deleted_at TEXT
);

-- Indexes for tenants
CREATE INDEX IF NOT EXISTS idx_tenants_slug ON tenants(slug);
CREATE INDEX IF NOT EXISTS idx_tenants_domain ON tenants(domain);
CREATE INDEX IF NOT EXISTS idx_tenants_status ON tenants(status);
CREATE INDEX IF NOT EXISTS idx_tenants_owner ON tenants(owner_id);

-- ============================================
-- Add tenant_id to existing tables
-- ============================================

-- Add tenant_id to users
ALTER TABLE users ADD COLUMN tenant_id TEXT REFERENCES tenants(id);
ALTER TABLE users ADD COLUMN tenant_role TEXT DEFAULT 'member';  -- owner | admin | member

-- Add tenant_id to tools
ALTER TABLE tools ADD COLUMN tenant_id TEXT REFERENCES tenants(id);

-- Add tenant_id to skills  
ALTER TABLE skills ADD COLUMN tenant_id TEXT REFERENCES tenants(id);

-- Add tenant_id to snippets
ALTER TABLE snippets ADD COLUMN tenant_id TEXT REFERENCES tenants(id);

-- Indexes for tenant filtering
CREATE INDEX IF NOT EXISTS idx_users_tenant ON users(tenant_id);
CREATE INDEX IF NOT EXISTS idx_tools_tenant ON tools(tenant_id);
CREATE INDEX IF NOT EXISTS idx_skills_tenant ON skills(tenant_id);
CREATE INDEX IF NOT EXISTS idx_snippets_tenant ON snippets(tenant_id);

-- ============================================
-- Tenant invitations table
-- ============================================
CREATE TABLE IF NOT EXISTS tenant_invitations (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    email TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'member',
    token TEXT UNIQUE NOT NULL,
    expires_at TEXT NOT NULL,
    accepted_at TEXT,
    created_by TEXT NOT NULL REFERENCES users(id),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    
    UNIQUE(tenant_id, email)
);

CREATE INDEX IF NOT EXISTS idx_invitations_tenant ON tenant_invitations(tenant_id);
CREATE INDEX IF NOT EXISTS idx_invitations_token ON tenant_invitations(token);
CREATE INDEX IF NOT EXISTS idx_invitations_email ON tenant_invitations(email);

-- ============================================
-- Tenant usage logs table
-- ============================================
CREATE TABLE IF NOT EXISTS tenant_usage_logs (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    resource_type TEXT NOT NULL,   -- tool_call | skill_exec | api_call
    resource_id TEXT,
    action TEXT NOT NULL,
    details TEXT DEFAULT '{}',
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_usage_logs_tenant_date ON tenant_usage_logs(tenant_id, created_at);
CREATE INDEX IF NOT EXISTS idx_usage_logs_type ON tenant_usage_logs(resource_type);

-- ============================================
-- Create default tenant for existing data
-- ============================================
-- This ensures existing data has a valid tenant
INSERT INTO tenants (id, name, slug, owner_id, plan, status)
SELECT 
    '00000000-0000-0000-0000-000000000001',
    'Default Tenant',
    'default',
    COALESCE((SELECT id FROM users LIMIT 1), '00000000-0000-0000-0000-000000000000'),
    'pro',
    'active'
WHERE NOT EXISTS (SELECT 1 FROM tenants WHERE id = '00000000-0000-0000-0000-000000000001');

-- Assign existing users to default tenant
UPDATE users SET tenant_id = '00000000-0000-0000-0000-000000000001' WHERE tenant_id IS NULL;
UPDATE users SET tenant_role = 'owner' WHERE tenant_role IS NULL AND id IN (SELECT owner_id FROM tenants);

-- Assign existing tools to default tenant
UPDATE tools SET tenant_id = '00000000-0000-0000-0000-000000000001' WHERE tenant_id IS NULL;

-- Assign existing skills to default tenant
UPDATE skills SET tenant_id = '00000000-0000-0000-0000-000000000001' WHERE tenant_id IS NULL;

-- Assign existing snippets to default tenant
UPDATE snippets SET tenant_id = '00000000-0000-0000-0000-000000000001' WHERE tenant_id IS NULL;
