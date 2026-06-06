# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Detected stack
- Languages: Rust.
- Frameworks: none detected from the supported starter markers.

## Verification
- Run Rust verification from repo root: `scripts/fmt.sh --check`; for formatting use `scripts/fmt.sh`. Run Rust clippy/tests from `rust/`: `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`
- `src/` and `tests/` are both present; update both surfaces together when behavior changes.

## Repository shape
- `rust/` contains the Rust workspace and active CLI/runtime implementation.
- `src/` contains source files that should stay consistent with generated guidance and tests.
- `tests/` contains validation surfaces that should be reviewed alongside code changes.

## Working agreement
- Prefer small, reviewable changes and keep generated bootstrap files aligned with actual repo workflows.
- Keep shared defaults in `.claude.json`; reserve `.claude/settings.local.json` for machine-local overrides.
- Do not overwrite existing `CLAUDE.md` content automatically; update it intentionally when repo workflows change.
## Fork sync safety

This fork uses two important branches:

- `main` - sync branch for `upstream/main`.
- `claw-local` - local working branch with custom Claw Code changes.

Before syncing, read:
`docs/fork-sync-workflow.md`
and:
`docs/claw-local-workflow.md`

Do not run `git rebase upstream/main` automatically.

Use:
`scripts/check-upstream-updates.ps1`

Safe update flow:
1. update `main` from `upstream/main`;
2. merge `main` into `claw-local`;
3. push `claw-local`.

Never push, reset hard, clean, skip rebase commits, discard commits, or force-push without explicit user confirmation.
