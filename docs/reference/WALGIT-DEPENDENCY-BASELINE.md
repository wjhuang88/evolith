# WalGit Dependency Baseline

> Owner: EVO-126-A / Iteration 069
> Status: In Progress dependency baseline
> This document records the allowed upstream dependency boundary. It does not claim that
> WalGit is the current Evolith runtime Git engine.

## Pinned upstream

- Repository: `https://github.com/tobi/walgit`
- Exact revision: `80e9a20b29e29aefd16a4dae6f8e274cce85cca5`
- Snapshot selected: 2026-09-22
- Upstream workspace version: `0.1.0`
- Upstream Rust MSRV: `1.90`
- Upstream edition: `2024`
- License: MIT

Cargo manifests are authoritative for the pin. Branch, tag or `main` dependencies are not allowed.

## Allowed engine crates

Evolith may depend directly on these crates at the same exact revision:

- `walgit-config`
- `walgit-store`
- `walgit-git`
- `walgit-wal`

`walgit-bundle` is intentionally deferred to EVO-126-G.

## Excluded product/server boundary

`walgit-server` is **not** an Evolith runtime dependency. Evolith continues to own:

- Actix HTTP routing and request lifecycle;
- authentication, tenant resolution and RBAC;
- Agent identity/scoped-token semantics;
- Evolith PolicyEvaluator, Commit/Promote and audit/outbox semantics;
- product UI and repository lifecycle state.

Later Smart HTTP work may derive/adapt protocol orchestration from WalGit server code under its
MIT license, but the Axum Router/AppState/Auth/UI/Product Policy boundary is not imported.

## Dependency graph notes

- Evolith currently uses `gix 0.78` for the existing filesystem Repo Context path.
- The pinned WalGit revision uses `gix 0.86` internally. Duplicate gix generations are accepted
  temporarily in EVO-126-A so the current runtime is not rewritten in the dependency Story.
- WalGit `walgit-store` defaults enable S3 and GCS providers, and `walgit-wal` depends on that
  store. Provider configuration, credentials, readiness and production selection belong to
  EVO-126-F; A only proves the pinned graph compiles and is locked.
- The committed Evolith `backend/Cargo.lock` is part of this baseline because crates.io
  transitive versions are not fixed by the WalGit Git SHA alone.

## Temporary bisync compatibility patch

The pinned WalGit revision still uses `gix 0.86`, whose `gix-protocol 0.64` dependency
requires `bisync ^0.3.0`. The original `bisync 0.3.0/0.3.1` releases were withdrawn
(yanked), so a clean Cargo resolution cannot select them. Gitoxide fixed the upstream problem
in [PR #2940](https://github.com/GitoxideLabs/gitoxide/pull/2940) and released
`gix-protocol 0.65.1` / `gix 0.87.1`, but those versions are outside WalGit's current
`gix = "0.86"` requirement.

Evolith therefore applies one narrow `[patch.crates-io]` exception:

- package identity exposed to `gix-protocol`: `bisync 0.3.2`;
- local shim: `backend/third_party/bisync-compat`;
- implementation dependency: exact crates.io `bisync2 = 0.3.2`;
- `bisync2` is a maintained API-compatible fork of the withdrawn `bisync` crate;
- the shim contains no copied implementation; it only re-exports `bisync2` under the
  package name expected by `gix-protocol`.

This is a compatibility exception, not a permanent platform dependency. Remove the patch when
the pinned WalGit revision moves to a gix generation that no longer depends on withdrawn
`bisync`.

## Upgrade procedure

A WalGit revision change is a deliberate dependency review, not a floating update:

1. Read upstream changes since the currently pinned revision, including workspace `Cargo.toml`,
   contracts/goal docs and license changes.
2. Select one exact commit SHA; never use `branch = "main"` or an unpinned Git URL.
3. Update all allowed WalGit workspace dependencies to the same revision.
4. Regenerate and review `backend/Cargo.lock`; inspect new/removed transitive dependencies and
   Rust-version requirements.
5. Run Rust 1.90 `fmt/check/clippy/test`, `cargo tree -p service-git`, exact-head CI,
   durability/recovery workflows and any Story-specific protocol tests.
6. Re-check whether the temporary `bisync` compatibility patch is still required; remove it
   when WalGit's gix requirement includes the upstream replacement.
7. Update this baseline and `THIRD_PARTY_NOTICES.md` if revision/license attribution changes.
8. Do not advance EVO-126-B+ if the pinned graph cannot be reproduced from a clean checkout.

## Runtime statement

Until EVO-126-H, the current Git runtime remains filesystem-backed Smart HTTP/subprocess + gix
Repo Context. Presence of WalGit crates in the dependency graph is not a storage cutover.
