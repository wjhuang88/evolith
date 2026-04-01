-- Seed data for development and testing
-- Only run in development environment
-- PostgreSQL-native version

-- Test user (password: password123)
INSERT INTO users (id, username, email, password_hash, role)
VALUES (
    '00000000-0000-0000-0000-000000000001',
    'testuser',
    'test@example.com',
    '$argon2id$v=19$m=19456,t=2,p=1$test_salt$password_hash_placeholder',
    'user'
) ON CONFLICT (id) DO NOTHING;

-- Admin user (password: admin123)
INSERT INTO users (id, username, email, password_hash, role)
VALUES (
    '00000000-0000-0000-0000-000000000002',
    'admin',
    'admin@example.com',
    '$argon2id$v=19456,t=2,p=1$admin_salt$admin_hash_placeholder',
    'admin'
) ON CONFLICT (id) DO NOTHING;

-- Sample tool
INSERT INTO tools (id, name, description, input_schema, handler_type, visibility, owner_id)
VALUES (
    '00000000-0000-0000-0001-000000000001',
    'weather-query',
    'Query weather information for a city',
    '{"type":"object","properties":{"city":{"type":"string","description":"City name"}},"required":["city"]}',
    'http',
    'public',
    '00000000-0000-0000-0000-000000000001'
) ON CONFLICT (id) DO NOTHING;

-- Sample skill (minimal)
INSERT INTO skills (id, name, version, description, skill_md, runtime, visibility, owner_id)
VALUES (
    '00000000-0000-0000-0002-000000000001',
    'data-analyzer',
    '1.0.0',
    'Analyze data files and generate reports',
    '# Data Analyzer',
    'python311',
    'public',
    '00000000-0000-0000-0000-000000000001'
) ON CONFLICT (id) DO NOTHING;

-- Sample snippet (minimal)
INSERT INTO snippets (id, name, language, framework, tags, content, code, estimated_tokens, visibility, owner_id)
VALUES (
    '00000000-0000-0000-0003-000000000001',
    'useDebounce Hook',
    'typescript',
    'react',
    '["hooks"]',
    '# useDebounce Hook',
    'export function useDebounce(v,d){return v}',
    150,
    'public',
    '00000000-0000-0000-0000-000000000001'
) ON CONFLICT (id) DO NOTHING;
