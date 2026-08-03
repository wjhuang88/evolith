-- Return a single integer describing whether a PostgreSQL database contains
-- user-owned schema state that a restore must not overwrite.
--
-- Internal schemas are ignored. The default public schema may exist while
-- empty, but every additional user schema counts as non-empty even if it has
-- no objects. Objects in public are counted across the schema-scoped catalogs
-- used by pg_dump for application data and programmable database objects.
WITH user_namespaces AS (
    SELECT oid, nspname
    FROM pg_catalog.pg_namespace
    WHERE nspname <> 'information_schema'
      AND nspname !~ '^pg_'
)
SELECT
    (SELECT COUNT(*)
       FROM user_namespaces
      WHERE nspname <> 'public')
    +
    (SELECT COUNT(*)
       FROM pg_catalog.pg_class AS c
       JOIN user_namespaces AS n ON n.oid = c.relnamespace
      WHERE n.nspname = 'public'
        AND c.relkind IN ('r', 'p', 'v', 'm', 'S', 'f', 'c'))
    +
    (SELECT COUNT(*)
       FROM pg_catalog.pg_proc AS p
       JOIN user_namespaces AS n ON n.oid = p.pronamespace
      WHERE n.nspname = 'public')
    +
    (SELECT COUNT(*)
       FROM pg_catalog.pg_type AS t
       JOIN user_namespaces AS n ON n.oid = t.typnamespace
      WHERE n.nspname = 'public'
        AND t.typisdefined)
    +
    (SELECT COUNT(*)
       FROM pg_catalog.pg_collation AS c
       JOIN user_namespaces AS n ON n.oid = c.collnamespace
      WHERE n.nspname = 'public')
    +
    (SELECT COUNT(*)
       FROM pg_catalog.pg_conversion AS c
       JOIN user_namespaces AS n ON n.oid = c.connamespace
      WHERE n.nspname = 'public')
    +
    (SELECT COUNT(*)
       FROM pg_catalog.pg_operator AS o
       JOIN user_namespaces AS n ON n.oid = o.oprnamespace
      WHERE n.nspname = 'public')
    +
    (SELECT COUNT(*)
       FROM pg_catalog.pg_opclass AS o
       JOIN user_namespaces AS n ON n.oid = o.opcnamespace
      WHERE n.nspname = 'public')
    +
    (SELECT COUNT(*)
       FROM pg_catalog.pg_opfamily AS o
       JOIN user_namespaces AS n ON n.oid = o.opfnamespace
      WHERE n.nspname = 'public')
    +
    (SELECT COUNT(*)
       FROM pg_catalog.pg_ts_config AS t
       JOIN user_namespaces AS n ON n.oid = t.cfgnamespace
      WHERE n.nspname = 'public')
    +
    (SELECT COUNT(*)
       FROM pg_catalog.pg_ts_dict AS t
       JOIN user_namespaces AS n ON n.oid = t.dictnamespace
      WHERE n.nspname = 'public')
    +
    (SELECT COUNT(*)
       FROM pg_catalog.pg_ts_parser AS t
       JOIN user_namespaces AS n ON n.oid = t.prsnamespace
      WHERE n.nspname = 'public')
    +
    (SELECT COUNT(*)
       FROM pg_catalog.pg_ts_template AS t
       JOIN user_namespaces AS n ON n.oid = t.tmplnamespace
      WHERE n.nspname = 'public')
    +
    (SELECT COUNT(*)
       FROM pg_catalog.pg_statistic_ext AS s
       JOIN user_namespaces AS n ON n.oid = s.stxnamespace
      WHERE n.nspname = 'public');
