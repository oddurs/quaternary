---
id: 35
uid: 27132f6b-0d97-4d07-93b5-b3bfcd875b3c
title: Partial answers under a time budget
type: feature
status: backlog
milestone: v0.2
depends_on:
- 33
created: 2026-11-01
updated: 2026-11-01
priority: p1
effort: s
area: engine
cell:
- z4
---

## Problem

Some entries will not finish. An honest partial answer is worth more than a timeout.

## Proposal

`--budget 10m`: when time runs out, return `lower: d ≥ x` with a certificate
covering the pieces completed.

## Acceptance criteria

- [ ] With a budget too small to finish the lifted Golay code, it returns `lower` with a bound the checker accepts.
- [ ] With an ample budget, it returns the same `exact` result as no budget.
