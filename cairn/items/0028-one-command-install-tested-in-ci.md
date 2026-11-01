---
id: 28
uid: 22fb6bcc-85ef-4b46-b9ee-a1a1470fc3e4
title: One-command install, tested in CI
type: chore
status: backlog
milestone: v0.1
depends_on:
- 25
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: s
area: infra
---

## Purpose

"A stranger can install it" is the v0.1 test, so it has to be checked by CI,
not assumed.

## Approach

A CI job on a clean runner: `cargo install --path crates/q4`, then
`q4 distance corpus/z4/octacode.q4`.

## Acceptance criteria

- [ ] The job passes on macOS and Linux.
- [ ] The README install line is the exact command the job runs.
