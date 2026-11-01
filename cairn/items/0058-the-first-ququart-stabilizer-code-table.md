---
id: 58
uid: fa1fa73e-7367-4615-94f6-a77018ef3784
title: The first ququart stabilizer code table
type: feature
status: backlog
milestone: v0.3
depends_on:
- 39
- 53
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: l
area: ledger
cell:
- ququart
---

## Problem

The fourth cell has no table. Building it is the result that makes this project
more than a reimplementation.

## Proposal

For n up to a length the ququart spike fixes (likely 6–8) and every k: the best d
found by exhaustive search where feasible and seeded random search beyond, each
with a certificate. Upper bounds from the quantum Singleton bound
(k ≤ n − 2(d − 1)) and other bounds the spike shows apply over Z4, all `cited`.
Gaps are marked `open`. If the spike found an existing table, this becomes an
audit of it.

## Acceptance criteria

- [ ] Every (n, k) in range has a ledger record with lower and upper sides and a status.
- [ ] Every lower bound has a certificate the checker accepts.
- [ ] Each search run's seed, method and budget are logged next to the table.
