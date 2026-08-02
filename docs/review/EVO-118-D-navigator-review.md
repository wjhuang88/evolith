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
- First fully validated implementation Head: `83351a2375b6537ddb83d71d84a2d22bfaaacc15`
- Current gate owner: [EVO-118-D](../backlog/active/EVO-118-D-git-storage-durability-and-recovery.md)
- Iteration owner: [Iteration 053](../iterations/ITERATION-053.md)
- Gate: `DATA-01`
- Required independence: the Driver and CI cannot substitute for Navigator judgment.
- WIP boundary: do not start or approve EVO-118-E/F/G/H, EVO-112-B, Agent writes, Webhooks, Indexer or Outbox as part of this review.

The Navigator must re-read the PR Head before reviewing. Any commit after a validated Head invalidates old exact-head CI as final evidence and requires the current Head to be revalidated.

## 2. Intended user-visible guarantee

After Backend process or container recreation, and after a joint PostgreSQL plus Git Storage backup is restored into an empty environment, an existing user must still be able to:

1. log in;
2. list the repository metadata;
3. clone the repository through real Git Smart HTTP;
4. observe the expected main Commit SHA;
5. observe the extra Branch and Tag;
6. receive readiness `503` while Git Storage is unusable and `200` after recovery.

The implementation must fail closed when the backup is incomplete, corrupt, incompatible, inconsistent, unrestorable, untraceable, or targeted at a non-empty environment. A DB-only or Git-only result must never be reported as successful recovery.

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
- `EVOLITH_APP_VERSION` is mandatory and remains non-empty after CR/LF sanitization.
- Manifest and checksums cover all required artifacts.
- Git repository refs/inventory are captured and validated.
- Source FIFO/socket/device or other special files are rejected before publication.
- The generated Git archive and final package are checked for unsafe paths, symlink/hardlink/special entry types and structural readability before atomic publication.
- A backup cannot print success for an archive that the corresponding restore entry-type policy will reject.
- Any required subcommand or self-validation failure produces a non-zero exit and no success claim.
- Secrets such as database passwords, JWT secrets, API keys or user tokens are not written into the manifest or ordinary logs.

### 3.4 Restore

Inspect:

- `scripts/restore.sh`
- `scripts/git-storage-inventory.sh`

Required findings:

- Restore accepts only an empty database target and empty Git target by default.
- Archive paths and entry types are validated before extraction.
- Manifest v1 contains exactly seven required keys; version, application version, UTC `created_at`, consistency, database format, Git format and Git Storage layout are validated before target mutation.
- Four unique checksums cover every required component exactly once.
- Git repositories are validated as bare repositories and expected refs are checked.
- The complete staged Git ref set must equal `git-refs.tsv`; an omitted extra Branch or Tag is rejected, not silently restored outside the inventory.
- Staging is used before final placement.
- PostgreSQL plain SQL is applied using one transaction with `ON_ERROR_STOP`; malformed SQL cannot leave committed partial schema/data.
- A later Git installation or inventory failure clears the previously empty targets; an incomplete automatic rollback is emitted as a CRITICAL condition rather than hidden.
- A failure cleans staging and never reports partial success.
- Post-restore DB/Git inventory must pass.

### 3.5 Database compatibility

Inspect:

- `backend/migrations/postgres/005_payment_integration.sql`
- related SQLite migration/API assumptions

Required findings:

- Billing plan identifiers are textual consistently with existing values such as `plan-free`.
- The fresh PostgreSQL migration path no longer attempts to insert textual IDs into UUID columns.
- The fix does not silently change unrelated billing behavior.

### 3.6 CI gate ownership

Inspect:

- `.github/workflows/ci.yml`
- `.github/workflows/data-durability-container.yml`

Required findings:

- PR required CI includes the DATA-01 backup/restore, generic volume, real application and real Backend-container evidence.
- The expensive production Compose clean-build/startup diagnostic remains available for release/tag/manual validation but does not consume the DATA-01 PR job timeout.
- A DEPLOY-01 diagnostic failure or runner-level timeout cannot overwrite the result of already completed DATA-01 required gates.
- Production build convergence remains owned by EVO-118-E and is not falsely declared complete by this PR.

## 4. Failure-first evidence to inspect

### 4.1 Script matrix

Inspect:

- `scripts/tests/durability.sh`

The matrix should prove rejection or rollback for at least:

- missing quiescence contract;
- missing application version and a version containing only CR/LF after sanitization;
- a FIFO/special file in source Git Storage, with no publishable backup left behind;
- DB-only or Git-only input;
- corrupt or incomplete archive;
- checksum mismatch and duplicate checksum entries;
- incompatible manifest version whose checksum is recomputed so the version gate is actually exercised;
- missing manifest key, incompatible Git layout and invalid creation timestamp;
- a `git-refs.tsv` snapshot missing an otherwise valid Branch or Tag from the Git archive;
- unsafe archive path, symlink, hardlink or special entry type;
- non-empty restore targets;
- malformed SQL after earlier valid statements, with the database remaining empty;
- post-write DB/Git inventory disagreement, with both targets rolled back empty;
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

## 5. Driver-discovered defects and required disposition

The Driver found and fixed the following defects while establishing and then auditing the evidence. The Navigator should verify every fix rather than treating this narrative as trusted.

1. **Fresh PostgreSQL migration defect**: migration 005 declared textual plan identifiers as UUID columns, causing `plan-free` migration failure. Fixed by aligning PostgreSQL column types with SQLite/API textual identifiers.
2. **CSRF harness defect**: recovery scripts attempted repository creation with Bearer authentication but omitted the CSRF cookie/header required for state-changing browser/API routes. Fixed using a cookie jar plus matching `X-CSRF-Token`.
3. **CI runtime-image UID collision**: Ubuntu 24.04 already occupied UID 1000, causing `useradd --uid 1000` to fail. Fixed with dedicated UID/GID `10001`.
4. **Unrestorable backup false-success gap**: backup previously checked only that tar could list an archive, while restore rejected link/special entry types. Fixed by rejecting special source entries and validating both generated archives against the restore-compatible path/type policy before publication.
5. **False version-test coverage**: the original version-mismatch mutation changed `manifest.env` without recomputing checksums, so restore failed at checksum validation instead of the version gate. Fixed by recomputing checksums and adding missing-key/layout/timestamp negative cases.
6. **Database partial-restore risk**: plain SQL previously relied on later best-effort schema cleanup after a SQL error. Fixed by applying the dump with `psql --single-transaction` and adding a malformed-SQL test that proves the previously empty target stays empty.
7. **Manifest contract under-validation**: restore previously ignored `git_storage_layout` and did not require every emitted field when no expected app version was supplied. Fixed with an exact seven-key v1 schema and explicit validation of every value.
8. **Incomplete ref-inventory acceptance**: restore previously verified only the refs listed in `git-refs.tsv`, allowing an archive to contain an unrecorded extra Branch or Tag. Fixed by requiring exact equality between the staged Git ref set and the inventory, with a missing-entry negative case.
9. **Untraceable backup version**: backup previously allowed a default `application_version=unknown`, and an intermediate check could accept a CR/LF-only value that became empty after sanitization. Fixed by requiring a non-empty sanitized application version and testing both missing and line-break-only inputs.
10. **DEPLOY-01 diagnostic overriding DATA-01 CI**: run `30755307482` completed every required DATA-01 step successfully, then the no-cache production Compose build consumed the 55-minute job timeout. Runner/job termination bypassed step-level `continue-on-error` and marked the whole workflow failed. Fixed by retaining production build/startup diagnostics only for release/tag/manual runs; PR required CI now ends after DATA-01 evidence.

These defects are useful negative evidence: a passing review must verify that the final tests exercise the intended gate and application path rather than succeeding for an earlier, unrelated failure reason.

## 6. Exact-head CI evidence history

The first fully successful implementation Head was `83351a2375b6537ddb83d71d84a2d22bfaaacc15`.

- `ci` run `30736864612`, run number `157`: **success**
- `data-durability-container` run `30736864631`, run number `3`: **success**

That successful main CI included:

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

Head `6bbb2dabc39232f7c057c08215a9faed0e0b3b9b` then passed all DATA-01 required steps in main run `30755307482` and passed dedicated container run `30755307491`. The main job was nevertheless reported failed only because the later DEPLOY-01 diagnostic hit the whole-job timeout; this is supporting evidence for the gate-ownership fix, not final green exact-head evidence.

The Navigator must identify the latest PR Head after the CI decoupling commits and require both current exact-head workflows to succeed. Old-head success and the timed-out diagnostic run remain supporting evidence only.

## 7. Security and data-integrity questions

The Navigator should answer each question explicitly:

1. Can a backup command, inventory check, special-file check or generated-archive self-check fail while the script still returns zero or prints success?
2. Can backup publish an archive that restore rejects because of symlink, hardlink, FIFO/socket/device or another special entry type?
3. Can an empty or sanitized-empty application version enter a published manifest?
4. Can restore mutate either target before all archive/schema/version/checksum/path validations pass?
5. Can malformed SQL commit earlier statements, or does the single transaction preserve an empty target?
6. Can a later Git installation/inventory failure leave committed DB objects or Git files while returning only the original error?
7. Can a non-empty target be overwritten accidentally?
8. Can a DB-only or Git-only state pass inventory?
9. Can path traversal, symlink, device or unexpected archive entry types escape staging?
10. Can missing, duplicate, extra or incompatible manifest keys pass validation?
11. Can an archive contain refs not represented exactly in `git-refs.tsv`?
12. Can manifest or logs leak credentials?
13. Can readiness pass when Git Storage is missing, wrong type, read-only or unwritable?
14. Can the readiness probe leave files behind or damage repository contents?
15. Does container recreation genuinely reuse the same named volume rather than copying data outside the volume?
16. Does the recovery drill verify real Git behavior and exact refs instead of checking only directory presence?
17. Does the implementation preserve SQLite behavior while fixing PostgreSQL migration compatibility?
18. Are all multi-instance/shared-storage claims appropriately limited?
19. Can an EVO-118-E/DEPLOY-01 diagnostic timeout make PR required DATA-01 evidence appear failed?
20. Are EVO-118-E/F/G/H and EVO-112-B still excluded from this PR?

Any unresolved data-loss, unrestorable-backup, partial-success, credential-leak, false-readiness or gate-ownership path is a blocking finding.

## 8. Navigator execution checklist

1. Re-read PR #7 metadata, current base SHA, current Head SHA, comments, reviews and changed files.
2. Confirm `main` has not advanced; if it has, assess mergeability and require rebase plus exact-head reruns.
3. Read the owner Story and Iteration before implementation files.
4. Inspect all files listed in Sections 3 and 4, including shell error handling and cleanup paths.
5. Compare the implementation to the BDD scenarios and non-goals.
6. Inspect the current Head workflow runs and individual failed/skipped steps.
7. Confirm that each negative test reaches its intended gate rather than being rejected earlier for a different reason.
8. Confirm production Compose build/startup diagnostics remain release/tag/manual evidence owned by EVO-118-E and cannot replace or override DATA-01 PR gates.
9. Record findings with severity and exact file/line or behavior evidence.
10. Require fixes and a new exact-head CI run for every blocking change.
11. Only after all blocking findings are resolved, record one final result: `Complete`, `Partial`, or `Blocked`.

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
