---
id: 12
uid: 21d50ae6-0660-42c7-b115-8c74fb743df9
title: Cargo workspace, scripts/task seam and CI
type: chore
status: backlog
milestone: v0.1
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: s
area: infra
---

## Purpose

Every later item needs somewhere to land and a gate that keeps it green.

## Approach

Workspace crates, whose boundaries carry the checker-independence decision:

- `q4-core`: rings, bit planes, code types, file formats.
- `q4-engine`: distance algorithms.
- `q4-cert`: certificate data types and serialisation only; depends on nothing in the workspace.
- `q4-check`: the independent checker; depends only on `q4-cert`.
- `q4`: the CLI.

`scripts/task` with the verbs `fmt fmt:check lint test build check`; GitHub
Actions on macOS and Linux runs `scripts/task check`. Licence MIT OR Apache-2.0.
The `/bootstrap-repo` skill does most of this.

## Acceptance criteria

- [ ] `scripts/task check` passes locally and in CI on macOS and Linux.
- [ ] Lint runs clippy with warnings denied.
- [ ] CI fails if `cargo tree -p q4-check` lists `q4-core` or `q4-engine`.
