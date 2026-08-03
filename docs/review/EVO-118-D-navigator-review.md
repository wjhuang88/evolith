# EVO-118-D Navigator Review Packet

> PR #7 / Iteration 053 / DATA-01. This packet is Driver-authored evidence routing, not approval. The independent Navigator must review the exact latest Head and return only `Complete`, `Partial`, or `Blocked`.

## Current state

- Repository: `wjhuang88/evolith`
- Branch: `agent/evo-118-d-git-durability-recovery`
- Base at remediation start: `38c19b19cff5aab7a08ac40a1cf417e1712e1b07`
- Story: `In Progress / Navigator Blocked / Remediation`
- Iteration 053: `Active / Navigator Blocked / Remediation`
- PR #7: Open / Draft
- DATA-01: Open
- EVO-118-E: Ready / Not Started

Any commit invalidates older exact-head workflow evidence. Driver implementation and CI cannot substitute for independent Navigator judgment.

## Previous independent result

The Navigator reviewed Head `c8b83dec54c1ed7df75c29b34b3d1b3a0f40a885` and concluded [`Blocked`](https://github.com/wjhuang88/evolith/pull/7#pullrequestreview-4839239246).

1. `scripts/restore.sh` and its tests defined PostgreSQL emptiness only inside `public`. Restore could accept a target containing `legacy.marker`, or leave restored non-`public` objects after a later Git/inventory failure while rollback appeared successful.
2. Git Storage readiness wrote and synced a probe but did not close the writer, reopen for reading, read all bytes and compare them exactly. A broken read path could return readiness 200.

Old successful runs `30756053633` (`ci` #177) and `30756053641` (`data-durability-container` #23) target the blocked Head and are not final evidence.

## Remediation files

### PostgreSQL empty target and rollback

Inspect:

- `scripts/postgres-user-object-count.sql`
- `scripts/restore.sh`
- `scripts/tests/durability.sh`

Required behavior:

- Ignore PostgreSQL internal schemas (`information_schema` and `pg_*`).
- Permit the default empty `public` schema.
- Treat every additional user schema as non-empty even when empty.
- Detect user relations, partitions, views, materialized views, sequences, foreign tables, functions/procedures, defined types and other schema-scoped user catalog objects.
- Run this definition before any target write.
- Reject a target containing `legacy.marker` before the PostgreSQL restore phase; preserve the marker and leave Git empty.
- Arm rollback only after both target-empty gates pass.
- On later failure, drop every non-internal user schema with `CASCADE`, recreate empty `public`, clean Git, and verify both targets are empty using the same SQL definition.
- Any cleanup or verification failure keeps restore non-zero, emits `CRITICAL`, and never prints restore success.
- Keep `ON_ERROR_STOP` plus `--single-transaction` for plain SQL restore.

The durability source fixture includes non-`public` schema `audit`, a table, function and enum type. Success must restore them. The normal post-write inventory failure must reach the intended gate and prove that `public`, `audit`, all schema objects and Git files are gone.

A separate deterministic incomplete-rollback case restores an event trigger that rejects `DROP SCHEMA`, then forces post-write inventory failure. It must reach restore and inventory, invoke rollback, return non-zero, print `CRITICAL: automatic rollback was incomplete`, suppress restore success, and prove the injected blocker/user state remains. The test recreates the isolated reject database afterwards; it must not weaken production rollback behavior with a test-only bypass.

### Git Storage readiness readback

Inspect `backend/crates/api/src/handlers/health.rs`.

Required behavior:

- Verify the base path exists, is a directory and is listable.
- Create an isolated UUID probe directory/file and write fixed bytes.
- Call `sync_all`, close the write handle, reopen with `File::open`, read to EOF and compare exact bytes.
- Return failure for reopen/read error, mismatch or cleanup error.
- Attempt cleanup on success and all readback failure paths.
- Never touch real repository content.
- Production always uses real reopen/read/compare; the injected readback callback exists only to construct deterministic tests.

Required tests cover normal readback/cleanup, exact byte comparison, truncated/corrupt mismatch, deterministic reopen/read failure, cleanup after both failures, missing path, regular file, unwritable directory and readiness status.

## Existing DATA-01 evidence to re-check

The Navigator must also inspect:

- `scripts/backup.sh` and `scripts/git-storage-inventory.sh`;
- `scripts/tests/application-recovery-drill.sh`;
- `scripts/tests/container-volume-persistence.sh`;
- `scripts/tests/backend-container-recreation.sh`;
- `docker-compose.prod.yml`, `deploy/k8s/backend.yaml`, `backend/Dockerfile`;
- PostgreSQL/SQLite migration 005 behavior;
- `.github/workflows/ci.yml` and `.github/workflows/data-durability-container.yml`;
- Release, configuration and script release-note documentation.

Confirm quiesced joint PostgreSQL+Git backup, required sanitized application version, exact manifest/checksum/ref inventory, safe archive types/paths, staging-first restore, DB/Git inventory, real Smart HTTP recovery, named-volume and real Backend container recreation, and textual `plan-free` compatibility.

Production no-cache Compose build/startup remains release/tag/manual evidence owned by EVO-118-E / DEPLOY-01. Multi-replica shared Git storage, Repo lifecycle Reconciler, EVO-118-F/G/H, EVO-112-B, Agent writes, Webhooks, Indexer and Outbox remain outside this PR.

## Failure-path evidence

`scripts/tests/durability.sh` must explicitly prove the intended gate, including:

- quiescence/application-version/special-file backup rejection;
- successful restore of `public` and non-`public` database objects;
- non-empty `public` and `legacy.marker` target refusal;
- missing/corrupt/incompatible/malicious archive refusal;
- malformed SQL reaches the transactional restore and leaves the target empty;
- post-write inventory failure reaches inventory, invokes rollback, removes all user schemas/objects and Git files;
- deliberately blocked rollback reaches the same post-write gate, remains non-zero, emits the exact `CRITICAL` incomplete-rollback warning and leaves demonstrably non-empty state;
- no failed restore prints `restore completed successfully`;
- DB-only, Git-only and invalid layout remain non-zero.

## Exact-head acceptance

The exact final remediation Head must have:

1. `ci`: success, including exact-head checkout, diff/link checks, frontend gates, shell syntax, Compose mapping, full PostgreSQL+Git matrix with non-`public` and `CRITICAL` rollback cases, volume recreation, Rust fmt/check/clippy, SQLite tests, application recovery and readiness readback tests.
2. `data-durability-container`: success, proving real Backend recreation with the same PostgreSQL database and named Git volume.

The re-review request must record the latest Base SHA, exact Head SHA, both run IDs, both blocker dispositions, added negative tests, and unchanged governance state.

## Navigator checklist

1. Re-read PR metadata, current base/head, all comments, Reviews, Review Threads and requested reviewers.
2. Check base drift, mergeability and exact-head workflow association.
3. Verify each new negative test reaches its named gate instead of failing earlier.
4. Confirm no false success, partial/mixed restore, hidden incomplete rollback or false readiness remains.
5. Keep PR Draft and DATA-01 Open during review.
6. Publish exact evidence and one result:

```text
Review target: <exact Head>
Base: <main SHA>
Blocking findings: <none or numbered findings>
Evidence independently checked: <files, tests, run IDs>
Scope check: EVO-118-D only; EVO-118-E not started
Final result: Complete | Partial | Blocked
```

Only a `Complete` result on the exact latest Head with no blocking finding permits Ready/merge. Merge alone does not close DATA-01; post-merge governance closeout remains required.