---
id: 79
uid: c9ea38c0-2de0-49ca-84c4-5352a38630e2
title: Fuzz every parser
type: chore
status: backlog
milestone: v1.0
depends_on:
- 18
- 31
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: s
area: infra
---

## Purpose

Hostile or broken input must give exit 2 with a location, never a panic. The
submission path invites strangers' files.

## Approach

cargo-fuzz targets for `.q4`, Magma and certificate parsing; seeded from the corpus.

## Acceptance criteria

- [ ] Each target runs for an hour with no crash, and the corpus is committed.
- [ ] Every crash found has a regression test.
