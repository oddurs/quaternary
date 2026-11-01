---
id: 26
uid: 8fd44f58-e2de-4f3e-82f0-71b94e4875dc
title: Property tests for the Z4 algebra
type: feature
status: backlog
milestone: v0.1
depends_on:
- 16
- 20
created: 2026-11-01
updated: 2026-11-01
priority: p1
effort: s
area: core
cell:
- z4
---

## Problem

Unit tests check the cases somebody thought of. The algebra has laws that can be
checked on cases nobody thought of.

## Proposal

proptest properties: the Gray map is an isometry; minimum distance is invariant
under monomial transforms (coordinate permutations and per-coordinate negation);
the dual of the dual is the code; standard form is idempotent.

## Acceptance criteria

- [ ] Each property runs 1,000 cases in CI.
- [ ] A failing case is persisted in `proptest-regressions/` and replayed on every run.
