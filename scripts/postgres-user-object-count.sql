-- Return a single integer describing whether a PostgreSQL database contains
-- state that the production database-level pg_dump can emit and a restore
-- must not overwrite.
--
-- Internal schemas and pinned built-in objects are ignored. The default public
-- schema may exist while empty. Every additional user schema counts as
-- non-empty even if it contains no objects. Objects in public are counted
-- across schema-scoped catalogs, while database-level objects emitted by
-- pg_dump are counted independently (extensions, publications, subscriptions,
-- event triggers, large objects, foreign-data objects, non-built-in languages,
-- casts, transforms and access methods).
WITH user_namespaces AS (
    SELECT oid, nspname
    FROM pg_catalog.pg_namespace
    WHERE nspname <> 'information_schema'
      AND nspname !~ '^pg_'
),
nonempty_objects AS (
    SELECT 'schema'::TEXT AS object_kind, n.oid AS object_oid
    FROM user_namespaces AS n
    WHERE n.nspname <> 'public'

    UNION ALL
    SELECT 'relation', c.oid
    FROM pg_catalog.pg_class AS c
    JOIN user_namespaces AS n ON n.oid = c.relnamespace
    WHERE n.nspname = 'public'
      AND c.relkind IN ('r', 'p', 'v', 'm', 'S', 'f', 'c')

    UNION ALL
    SELECT 'routine', p.oid
    FROM pg_catalog.pg_proc AS p
    JOIN user_namespaces AS n ON n.oid = p.pronamespace
    WHERE n.nspname = 'public'

    UNION ALL
    SELECT 'type', t.oid
    FROM pg_catalog.pg_type AS t
    JOIN user_namespaces AS n ON n.oid = t.typnamespace
    WHERE n.nspname = 'public'
      AND t.typisdefined

    UNION ALL
    SELECT 'collation', c.oid
    FROM pg_catalog.pg_collation AS c
    JOIN user_namespaces AS n ON n.oid = c.collnamespace
    WHERE n.nspname = 'public'

    UNION ALL
    SELECT 'conversion', c.oid
    FROM pg_catalog.pg_conversion AS c
    JOIN user_namespaces AS n ON n.oid = c.connamespace
    WHERE n.nspname = 'public'

    UNION ALL
    SELECT 'operator', o.oid
    FROM pg_catalog.pg_operator AS o
    JOIN user_namespaces AS n ON n.oid = o.oprnamespace
    WHERE n.nspname = 'public'

    UNION ALL
    SELECT 'operator_class', o.oid
    FROM pg_catalog.pg_opclass AS o
    JOIN user_namespaces AS n ON n.oid = o.opcnamespace
    WHERE n.nspname = 'public'

    UNION ALL
    SELECT 'operator_family', o.oid
    FROM pg_catalog.pg_opfamily AS o
    JOIN user_namespaces AS n ON n.oid = o.opfnamespace
    WHERE n.nspname = 'public'

    UNION ALL
    SELECT 'text_search_config', t.oid
    FROM pg_catalog.pg_ts_config AS t
    JOIN user_namespaces AS n ON n.oid = t.cfgnamespace
    WHERE n.nspname = 'public'

    UNION ALL
    SELECT 'text_search_dictionary', t.oid
    FROM pg_catalog.pg_ts_dict AS t
    JOIN user_namespaces AS n ON n.oid = t.dictnamespace
    WHERE n.nspname = 'public'

    UNION ALL
    SELECT 'text_search_parser', t.oid
    FROM pg_catalog.pg_ts_parser AS t
    JOIN user_namespaces AS n ON n.oid = t.prsnamespace
    WHERE n.nspname = 'public'

    UNION ALL
    SELECT 'text_search_template', t.oid
    FROM pg_catalog.pg_ts_template AS t
    JOIN user_namespaces AS n ON n.oid = t.tmplnamespace
    WHERE n.nspname = 'public'

    UNION ALL
    SELECT 'extended_statistics', s.oid
    FROM pg_catalog.pg_statistic_ext AS s
    JOIN user_namespaces AS n ON n.oid = s.stxnamespace
    WHERE n.nspname = 'public'

    UNION ALL
    SELECT 'extension', e.oid
    FROM pg_catalog.pg_extension AS e
    WHERE e.oid >= 16384

    UNION ALL
    SELECT 'publication', p.oid
    FROM pg_catalog.pg_publication AS p
    WHERE p.oid >= 16384

    UNION ALL
    SELECT 'subscription', s.oid
    FROM pg_catalog.pg_subscription AS s
    WHERE s.oid >= 16384
      AND s.subdbid = (SELECT oid FROM pg_catalog.pg_database WHERE datname = current_database())

    UNION ALL
    SELECT 'event_trigger', e.oid
    FROM pg_catalog.pg_event_trigger AS e
    WHERE e.oid >= 16384

    UNION ALL
    SELECT 'large_object', l.oid
    FROM pg_catalog.pg_largeobject_metadata AS l
    WHERE l.oid >= 16384

    UNION ALL
    SELECT 'foreign_data_wrapper', f.oid
    FROM pg_catalog.pg_foreign_data_wrapper AS f
    WHERE f.oid >= 16384

    UNION ALL
    SELECT 'foreign_server', s.oid
    FROM pg_catalog.pg_foreign_server AS s
    WHERE s.oid >= 16384

    UNION ALL
    SELECT 'user_mapping', m.oid
    FROM pg_catalog.pg_user_mapping AS m
    WHERE m.oid >= 16384

    UNION ALL
    SELECT 'language', l.oid
    FROM pg_catalog.pg_language AS l
    WHERE l.oid >= 16384

    UNION ALL
    SELECT 'cast', c.oid
    FROM pg_catalog.pg_cast AS c
    WHERE c.oid >= 16384

    UNION ALL
    SELECT 'transform', t.oid
    FROM pg_catalog.pg_transform AS t
    WHERE t.oid >= 16384

    UNION ALL
    SELECT 'access_method', a.oid
    FROM pg_catalog.pg_am AS a
    WHERE a.oid >= 16384
)
SELECT COUNT(*) FROM nonempty_objects;
