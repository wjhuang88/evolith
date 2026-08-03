# EVO-118-D Navigator Review Packet

> PR #7 / Iteration 053 / DATA-01. This is Driver-authored evidence routing, not approval. The independent Navigator must review the exact latest Head and return only `Complete`, `Partial`, or `Blocked`.

## Current governance

- Repository: `wjhuang88/evolith`
- Branch: `agent/evo-118-d-git-durability-recovery`
- Base: `main` @ `38c19b19cff5aab7a08ac40a1cf417e1712e1b07`
- Story: `In Progress / Navigator Blocked / Remediation`
- Iteration 053: `Active / Navigator Blocked / Remediation`
- PR #7: Open / Draft
- DATA-01: Open
- EVO-118-E: Ready / Not Started

Any commit invalidates older exact-head workflow evidence. Driver implementation and CI cannot substitute for independent Navigator judgment.

## Independent review history

### Review 1 — Head `c8b83dec54c1ed7df75c29b34b3d1b3a0f40a885`

Result: [`Blocked`](https://github.com/wjhuang88/evolith/pull/7#pullrequestreview-4839239246).

1. PostgreSQL emptiness and rollback covered only `public`, so non-`public` user state could bypass preflight or survive rollback.
2. Git Storage readiness wrote and synced but did not close, reopen, read and compare exact bytes.

### Review 2 — Head `7608be9f5e7234c0797e4aeba23a133ca91552dc`

Result: `Blocked`.

- Git Storage independent reopen/read/compare and cleanup: **Resolved**.
- Non-`public` schema preflight and rollback for schema-scoped objects: **Resolved**.
- Remaining blocker: production backup uses database-level `pg_dump`; database-level objects such as an empty Publication were not represented in the shared emptiness helper or rollback cleanup. A Publication-only target could be treated as empty and a restored Publication could survive a later failure without triggering incomplete-rollback `CRITICAL`.

All runs associated with those reviewed Heads are historical evidence only.

### Review 3 — Head `551e71331a579020a0037c3db2a7ccd98c781c05`

Result: `Blocked`.

The Navigator independently marked every earlier finding resolved, including non-`public` schemas, Publication-only refusal, Publication backup/restore/rollback, incomplete rollback `CRITICAL`, and Git Storage reopen/read/compare. Two new blockers remain:

1. Production `pg_dump` runs as the default PostgreSQL superuser but does not pass PostgreSQL 16 `--no-subscriptions`; a dumped `CREATE SUBSCRIPTION ... CONNECTION '<conninfo>'` can place plaintext host/user/password material into `database.sql.gz` and the final ordinary archive.
2. Production non-interactive `psql` calls do not pass `-X`; system or user startup files can execute SQL before the preflight gate, alter `ON_ERROR_STOP`, and create changes outside the armed rollback boundary.

Head `551e713...` workflow runs `ci` #197 / `30832407367` and `data-durability-container` #43 / `30832407357` are historical evidence because they do not exercise either new negative path.


## Database-level PostgreSQL remediation

Inspect:

- `scripts/postgres-user-object-count.sql`
- `scripts/restore.sh`
- `scripts/tests/durability.sh`
- `scripts/tests/postgres-database-object-durability.sh`
- `.github/workflows/ci.yml`

Required behavior now implemented:

- Keep the existing schema-scoped contract: internal schemas are ignored, default empty `public` is permitted, every additional user schema counts as non-empty, and user objects inside `public` count as non-empty.
- Also count database-level state that production `pg_dump` can emit: user extensions, publications, current-database subscriptions, event triggers, large objects, foreign-data wrappers/servers/user mappings, and non-built-in languages, casts, transforms and access methods.
- Use the same SQL helper for restore preflight, rollback verification and tests.
- Reject a Publication-only target before PostgreSQL or Git writes and preserve the existing Publication.
- Restore a production backup containing an empty Publication and prove that the Publication, database marker and Git commit all return.
- On a later post-write inventory failure, remove subscriptions safely, then extensions, publications, foreign-data objects, large objects, all user schemas and remaining database-level objects; recreate empty `public`, clean Git, and require the shared helper to return exactly zero.
- Preserve the existing deterministic event-trigger cleanup failure: restore remains non-zero, success is suppressed, and `CRITICAL: automatic rollback was incomplete` is emitted when complete cleanup is impossible.

The focused Publication matrix must prove three independent gates:

1. production backup and successful restore carry the Publication;
2. a Publication-only target is rejected before writes and remains unchanged;
3. post-write failure removes the restored Publication, all schema state and Git data, after which the shared helper returns `0`.

## Git Storage readiness disposition
## Subscription credential boundary

Inspect:

- `scripts/backup.sh`
- `scripts/postgres-user-object-count.sql`
- `scripts/tests/postgres-database-object-durability.sh`
- `docs/reference/SCRIPTS-RELEASE-NOTES.md`

Required behavior:

- Production backup uses PostgreSQL 16 `pg_dump --no-subscriptions` while retaining Publication behavior.
- Subscription conninfo is environment-level sensitive operational configuration outside the DATA-01 ordinary joint backup format; it must be reconstructed through a separate secure controlled process rather than extending this PR into encrypted backup/key management.
- The shared empty-database helper continues counting current-database Subscriptions, so a Subscription-only target is rejected before writes and preserved.
- The focused test creates a real `connect=false`, no-slot Subscription whose catalog conninfo contains a unique credential sentinel.
- The test unpacks the final outer archive, decompresses `database.sql.gz`, and proves the sentinel, Subscription name and `CREATE SUBSCRIPTION` are absent from SQL; sentinel is also absent from manifest, refs, checksum, archive listing and ordinary backup/restore stdout/stderr.
- Publication, database marker, Git refs and inventory still restore successfully, while the target contains no Subscription.

## psql startup-file isolation

Inspect:

- `scripts/restore.sh`
- `scripts/git-storage-inventory.sh`
- `scripts/tests/postgres-database-object-durability.sh`

Required behavior:

- Every production non-interactive `psql` path uses a centralized `-X` / `ON_ERROR_STOP=1` wrapper, including preflight helper, plain SQL restore, rollback cleanup, rollback verification and inventory invoked directly or through backup.
- Plain SQL restore retains `--single-transaction`; rollback, exact `CRITICAL` and no-success-on-failure semantics remain unchanged.
- The focused test supplies a hostile `PSQLRC` that would create `public.psqlrc_sentinel` and alter `ON_ERROR_STOP`.
- Restore must execute far enough to validate the archive and reach the legitimate non-empty database preflight gate, but must not enter PostgreSQL restore, Git installation or post-write inventory.
- The preexisting marker and normalized schema snapshot remain exact, `psqlrc_sentinel` is absent, Git remains empty, and no success message is printed.
- Test setup and verification SQL also use `-X`.


Inspect `backend/crates/api/src/handlers/health.rs` and its tests.

The second Navigator already marked this blocker resolved. The implementation writes fixed bytes, `sync_all`s, closes the writer, reopens with `File::open`, reads to EOF, compares exact bytes, and treats reopen/read/mismatch/cleanup failure as not-ready. Deterministic tests cover successful cleanup, short/corrupt mismatch and read-open/read failure without touching real repository data.

## Existing DATA-01 evidence to retain

The Navigator must also re-check:

- quiesced joint PostgreSQL + Git backup;
- strict manifest/checksum/archive/ref validation;
- staging-first restore and single-transaction SQL restore;
- DB/Git inventory and no false success;
- non-`public` schema refusal and rollback;
- deterministic incomplete rollback `CRITICAL`;
- named-volume recreation and real Backend container recreation;
- real application register/create/Smart HTTP push/backup/empty restore/login/list/clone/refs/restart drill;
- frontend gates, Rust fmt/check/clippy/SQLite workspace tests and readiness tests;
- PostgreSQL/SQLite textual plan-ID compatibility.

Production Compose no-cache build/startup remains release/tag/manual evidence owned by EVO-118-E / DEPLOY-01. Multi-replica shared storage, Repo lifecycle Reconciler, EVO-118-F/G/H, EVO-112-B, Agent writes, Webhooks, Indexer and Outbox remain outside PR #7.

## Exact-head acceptance

The final review request in the PR conversation must identify the current exact Head and two workflow runs:

1. `ci`: success, including exact-head checkout, diff/link checks, frontend gates, shell syntax, Compose mapping, original durability matrix, database-level Publication matrix, volume recreation, Rust fmt/check/clippy, SQLite tests and application recovery.
2. `data-durability-container`: success, proving real Backend recreation against the same PostgreSQL database and named Git volume.

## Navigator checklist

1. Re-read PR metadata, current Base/Head, all comments, Reviews, Review Threads and requested reviewers.
2. Verify both workflows target the exact current Head and completed successfully.
3. Inspect the production dump object contract: Publication remains in scope, Subscription credentials are explicitly excluded, and Subscription-only targets are still non-empty.
4. Confirm the credential sentinel exists in `pg_subscription.subconninfo`, then verify the test inspects the unpacked archive and decompressed SQL rather than grepping compressed bytes.
5. Confirm production `psql` uses `-X` in every preflight/restore/rollback/inventory path and the hostile startup-file test reaches the named pre-write gate without side effects.
6. Confirm all earlier negative matrices still run, each failure reaches its named gate, and no failed restore prints success.
7. Confirm no mixed target, hidden database-level residue, incomplete rollback without `CRITICAL`, credential leakage, startup-file dependence, or false readiness remains.
8. Keep PR Draft and DATA-01 Open during review.
9. Publish one result:

```text
Review target: <exact Head>
Base: <main SHA>
Blocking findings: <none or numbered findings>
Evidence independently checked: <files, tests, run IDs>
Scope check: EVO-118-D only; EVO-118-E not started
Final result: Complete | Partial | Blocked
```

Only `Complete` on the exact latest Head with no blocking finding permits Ready/merge. Merge alone does not close DATA-01; post-merge governance closeout remains mandatory.
