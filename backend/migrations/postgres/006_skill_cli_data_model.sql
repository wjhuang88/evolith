-- EVO-049-A: Agent Skills compatible fields for skills table
ALTER TABLE skills ADD COLUMN author TEXT;
ALTER TABLE skills ADD COLUMN tags JSONB DEFAULT '[]'::jsonb;
ALTER TABLE skills ADD COLUMN skill_type TEXT DEFAULT 'instruction';
ALTER TABLE skills ADD COLUMN execution TEXT DEFAULT 'client';
ALTER TABLE skills ADD COLUMN entrypoint TEXT;
ALTER TABLE skills ADD COLUMN timeout INTEGER DEFAULT 30;
ALTER TABLE skills ADD COLUMN memory_mb INTEGER DEFAULT 256;
ALTER TABLE skills ADD COLUMN permissions JSONB;
ALTER TABLE skills ADD COLUMN license TEXT;
ALTER TABLE skills ADD COLUMN compatibility TEXT;
ALTER TABLE skills ADD COLUMN disable_model_invocation BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE skills ADD COLUMN user_invocable BOOLEAN NOT NULL DEFAULT TRUE;
ALTER TABLE skills ADD COLUMN argument_hint TEXT;

-- EVO-049-A: Structured CLI fields for snippets table
ALTER TABLE snippets ADD COLUMN version TEXT DEFAULT '1.0.0';
ALTER TABLE snippets ADD COLUMN summary TEXT;
ALTER TABLE snippets ADD COLUMN command TEXT;
ALTER TABLE snippets ADD COLUMN subcommands JSONB DEFAULT '[]'::jsonb;
ALTER TABLE snippets ADD COLUMN inputs JSONB DEFAULT '[]'::jsonb;
ALTER TABLE snippets ADD COLUMN output JSONB;
ALTER TABLE snippets ADD COLUMN examples JSONB DEFAULT '[]'::jsonb;
ALTER TABLE snippets ADD COLUMN error_model JSONB;
