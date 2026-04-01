-- Initialize database schema for Evolith
-- PostgreSQL-native version

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'user',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Tools table
CREATE TABLE IF NOT EXISTS tools (
    id UUID PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    description TEXT NOT NULL,
    input_schema JSONB NOT NULL,
    output_schema JSONB,
    handler_type TEXT NOT NULL,
    handler_url TEXT,
    handler_method TEXT,
    handler_timeout INTEGER,
    visibility TEXT NOT NULL DEFAULT 'private',
    owner_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Skills table
CREATE TABLE IF NOT EXISTS skills (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    description TEXT NOT NULL,
    skill_md TEXT NOT NULL,
    code_package_path TEXT,
    runtime TEXT NOT NULL,
    dependencies JSONB DEFAULT '[]'::jsonb,
    visibility TEXT NOT NULL DEFAULT 'private',
    owner_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(name, version)
);

-- Snippets table
CREATE TABLE IF NOT EXISTS snippets (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    language TEXT NOT NULL,
    framework TEXT,
    tags JSONB DEFAULT '[]'::jsonb,
    content TEXT NOT NULL,
    code TEXT NOT NULL,
    dependencies JSONB DEFAULT '[]'::jsonb,
    estimated_tokens INTEGER NOT NULL DEFAULT 0,
    visibility TEXT NOT NULL DEFAULT 'private',
    owner_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_tools_owner ON tools(owner_id);
CREATE INDEX IF NOT EXISTS idx_tools_visibility ON tools(visibility);
CREATE INDEX IF NOT EXISTS idx_skills_owner ON skills(owner_id);
CREATE INDEX IF NOT EXISTS idx_skills_visibility ON skills(visibility);
CREATE INDEX IF NOT EXISTS idx_skills_name_version ON skills(name, version);
CREATE INDEX IF NOT EXISTS idx_snippets_owner ON snippets(owner_id);
CREATE INDEX IF NOT EXISTS idx_snippets_visibility ON snippets(visibility);
CREATE INDEX IF NOT EXISTS idx_snippets_language ON snippets(language);
