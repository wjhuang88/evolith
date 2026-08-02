# EVO-118-D Navigator Review Packet

> Review target: PR #7, EVO-118-D, Iteration 053, production gate DATA-01.
>
> This packet is prepared by the Driver. It is evidence routing, not an approval. The Navigator must independently inspect the exact Head, changed files, failure behavior and CI evidence before recording `Complete`, `Partial`, or `Blocked`.

## 1. Review identity and invariants

- Repository: `wjhuang88/evolith`
- Pull request: [PR #7](https://github.com/wjhuang88/evolith/pull/7)
- Base branch: `main`
- Base SHA at packet preparation: `38c19b19cff5aab7a08ac40a1cf417e1712e1b07`
- Implementation branch: `agent/evo-118-d-git-durability-recovery`
- Pre-packet validated Head: `83351a2375b6537ddb83d71d84a2d22bfaaacc15`
- Current gate owner: [EVO-118-D](../backlog/active/EVO-118-D-git-storage-durability-and-recovery.md)
- Iteration owner: [Iteration 053](../iterations/ITERATION-053.md)
- Gate: `DATA-01`
- Required independence: the Driver and CI cannot substitute for Navigator judgment.
- WIP boundary: do not start or approve EVO-118-E/F/G/H, EVO-112-B, Agent writes, Webhooks, Indexer or Outbox as part of this review.

The Navigator must re-read the PR Head before reviewing. Any commit after the validated Head invalidates old exact-head CI as final evidence and requires the current Head to be revalidated.

## 2. Intended user-visible guarantee

After Backend process or container recreation, and after a joint PostgreSQL plus Git Storage backup is restored into an empty environment, an existing user must still be able to:

1. log in;
2. list the repository metadata;
3. clone the repository through real Git Smart HTTP;
4. observe the expected main Commit SHA;
5. observe the extra Branch and Tag;
6. receive readiness `503` while Git Storage is unusable and `200` after recovery.

The implementation must fail closed when the backup is incomplete, corrupt, incompatible, inconsistent, or targeted at a non-empty environment. A DB-only or Git-only result must never be reported as successful recovery.

## 3. Architecture and ownership boundaries to inspect

### 3.1 Persistent storage

Inspect:

- `docker-compose.prod.yml`
- `deploy/k8s/backend.yaml`
- `backend/Dockerfile`
- `docs/reference/CONFIG.md`

Required findings:

- `GIT_STORAGE__BASE_PATH` is explicit and consistent.
- Production Compose mounts a named Git volume at the configured path.
- K8s mounts persistent storage at the same path.
- The implementation does not claim unsupported multi-replica shared-storage semantics.
- Runtime user permissions allow the Backend to access the mounted directory without making the application run as root.

### 3.2 Joint readiness

Inspect:

- `backend/crates/api/src/handlers/health.rs`

Required findings:

- Readiness represents both PostgreSQL and Git Storage, not process liveness alone.
- Git Storage checks path existence/type and real create/write/sync/read/cleanup behavior.
- A Git failure returns `503` even when PostgreSQL is healthy.
- Recovery returns readiness to `200`.
- Temporary probe files are removed and failures do not expose credentials or internal data.

### 3.3 Backup

Inspect:

- `scripts/backup.sh`
- `scripts/git-storage-inventory.sh`
- `docs/sop/RELEASE.md`
- `docs/reference/SCRIPTS-RELEASE-NOTES.md`

Required findings:

- Backup requires an explicit quiesced/maintenance boundary.
- PostgreSQL dump and Git Storage are included in one versioned archive.
- Manifest and checksums cover all required artifacts.
- Git repository refs/inventory are captured and validated.
- Any required subcommand failure produces a non-zero exit and no success claim.
- Secrets such as database passwords, JWT secrets, API keys or user tokens are not written into the manifest or ordinary logs.

### 3.4 Restore

Inspect:

- `scripts/restore.sh`
- `scripts/git-storage-inventory.sh`

Required findings:

- Restore accepts only an empty database target and empty Git target by default.
- Archive paths and entry types are validated before extraction.
- Manifest version, layout and checksums are validated before target mutation.
- Git repositories are validated as bare repositories and expected refs are checked.
- Staging is used before final placement.
- A failure cleans staging and does not report partial success.
- Post-restore DB/Git inventory must pass.

### 3.5 Database compatibility

Inspect:

- `backend/migrations/postgres/005_payment_integration.sql`
- related SQLite migration/API assumptions

Required findings:

- Billing plan identifiers are textual consistently with existing values such as `plan-free`.
- The fresh PostgreSQL migration path no longer attempts to insert textual IDs into UUID columns.
- The fix does not silently change unrelated billing behavior.

## 4. Failure-first evidence to inspect

### 4.1 Script matrix

Inspect:

- `scripts/tests/durability.sh`

The matrix should prove rejection of at least:

- missing quiescence contract;
- DB-only or Git-only input;
- corrupt or incomplete archive;
- checksum mismatch;
- incompatible manifest version/layout;
- unsafe archive path or entry type;
- non-empty restore targets;
- DB/Git inventory disagreement;
- malformed repository layout or invalid bare repository;
- missing expected default branch/commit evidence.

### 4.2 Generic volume recreation

Inspect:

- `scripts/tests/container-volume-persistence.sh`

Required evidence:

- a named Docker volume survives deletion of one container and attachment to another;
- Commit, Branch, Tag and clone/refs behavior are verified, not only file existence.

### 4.3 Real Backend container recreation

Inspect:

- `scripts/tests/backend-container-recreation.sh`
- `.github/workflows/data-durability-container.yml`

Required evidence:

- the actual Evolith Backend binary runs inside two sequential containers;
- both containers use the same PostgreSQL database and the same named Git volume;
- the first container registers a user, creates a repository and pushes refs;
- after deleting the first container, the second supports login, repo list, clone and refs verification;
- CSRF is satisfied using the service's double-submit contract for state-changing API calls;
- the CI-only runtime image uses dedicated UID/GID `10001`, avoiding the Ubuntu base-image UID `1000` collision;
- the test is not a substitute for the production Dockerfile convergence owned by EVO-118-E.

### 4.4 Application-level empty-environment recovery

Inspect:

- `scripts/tests/application-recovery-drill.sh`
- `.github/workflows/ci.yml`

Required evidence:

- fresh source and target PostgreSQL databases are used;
- a real Backend process performs register, repo create and Smart HTTP pushes;
- multiple commits, an extra branch and a tag are created;
- the source application is stopped before backup;
- restore occurs into an empty target database and Git directory;
- the restored application supports login, list, clone, exact SHA and refs checks;
- a real Git Storage readiness failure is injected and returns `503`, then recovers to `200`;
- restarting Backend against restored data still supports login and clone.

## 5. Previously discovered defects and required disposition

The Driver found and fixed three defects while establishing evidence. The Navigator should verify each fix rather than treating it as trusted narrative.

1. **Fresh PostgreSQL migration defect**: migration 005 declared textual plan identifiers as UUID columns, causing `plan-free` migration failure. Fixed by aligning PostgreSQL column types with SQLite/API textual identifiers.
2. **CSRF harness defect**: recovery scripts attempted repository creation with Bearer authentication but omitted the CSRF cookie/header required for state-changing browser/API routes. Fixed using a cookie jar plus matching `X-CSRF-Token`.
3. **CI runtime-image UID collision**: Ubuntu 24.04 already occupied UID 1000, causing `useradd --uid 1000` to fail. Fixed with dedicated UID/GID `10001`.

These were test or migration defects, but they are also useful negative evidence: a passing review must verify that the final tests exercise the intended application paths rather than bypassing controls.

## 6. Exact-head CI evidence before this packet

The implementation Head immediately before this review packet was `83351a2375b6537ddb83d71d84a2d22bfaaacc15`.

- `ci` run `30736864612`, run number `157`: **success**
- `data-durability-container` run `30736864631`, run number `3`: **success**

The successful main CI included:

- exact-head checkout verification;
- `git diff --check` and Markdown local link validation;
- frontend install, type-check and build;
- durability script syntax;
- production Compose Git volume mapping;
- real PostgreSQL plus Git backup/restore integration matrix;
- generic Docker volume recreation;
- Rust format/check/clippy and SQLite workspace tests;
- application-level recovery drill;
- production Compose clean-build diagnostic.

The successful dedicated workflow included the real Backend container deletion/recreation drill.

Because this packet itself changes the PR Head, the Navigator must require successful workflows for the new exact Head before acceptance. Old-head success remains supporting evidence only.

## 7. Security and data-integrity questions

The Navigator should answer each question explicitly:

1. Can a backup command fail while the script still returns zero or prints success?
2. Can restore mutate either target before all archive/version/checksum/path validations pass?
3. Can a non-empty target be overwritten accidentally?
4. Can a DB-only or Git-only state pass inventory?
5. Can path traversal, symlink, device or unexpected archive entry types escape staging?
6. Can manifest or logs leak credentials?
7. Can readiness pass when Git Storage is missing, wrong type, read-only or unwritable?
8. Can the readiness probe leave files behind or damage repository contents?
9. Does container recreation genuinely reuse the same named volume rather than copying data outside the volume?
10. Does the recovery drill verify real Git behavior and exact refs instead of checking only directory presence?
11. Does the implementation preserve SQLite behavior while fixing PostgreSQL migration compatibility?
12. Are all multi-instance/shared-storage claims appropriately limited?
13. Are EVO-118-E/F/G/H and EVO-112-B still excluded from this PR?

Any unresolved data-loss, partial-success, credential-leak or false-readiness path is a blocking finding.

## 8. Navigator execution checklist

1. Re-read PR #7 metadata, current base SHA, current Head SHA, comments, reviews and changed files.
2. Confirm `main` has not advanced; if it has, assess mergeability and require rebase plus exact-head reruns.
3. Read the owner Story and Iteration before implementation files.
4. Inspect all files listed in Sections 3 and 4, including shell error handling and cleanup paths.
5. Compare the implementation to the BDD scenarios and non-goals.
6. Inspect the current Head workflow runs and individual failed/skipped steps.
7. Treat tag/manual-only tests and production-startup diagnostics according to documented ownership; do not allow diagnostics to replace DATA-01 evidence.
8. Record findings with severity and exact file/line or behavior evidence.
9. Require fixes and a new exact-head CI run for every blocking change.
10. Only after all blocking findings are resolved, record one final result: `Complete`, `Partial`, or `Blocked`.

## 9. Result template

The independent Navigator should append or publish a result using this structure:

```text
Review target: <exact head SHA>
Base: <main SHA>
Navigator identity/session: <independent reviewer>

Blocking findings:
- <none, or numbered findings with file/behavior evidence>

Non-blocking findings / residual ownership:
- <finding -> owner Story>

Evidence independently checked:
- <files, commands, workflow run IDs, failure cases>

Scope check:
- EVO-118-D only: yes/no
- EVO-118-E not started: yes/no
- DATA-01 may close now: yes/no

Final result: Complete | Partial | Blocked
```

Until that independent result exists for the latest exact Head, PR #7 must remain Draft, Iteration 053 remains Active, EVO-118-D remains In Progress, and DATA-01 remains Open.
