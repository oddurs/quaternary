---
id: 52
uid: eb419b0c-13a9-438f-b615-92f7ab1e832f
title: Ququart stabilizer codes (Z4 symplectic)
type: feature
status: backlog
milestone: v0.3
depends_on:
- 14
- 15
- 50
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: l
area: engine
cell:
- ququart
---

## Problem

The empty cell. No open tool computes these distances.

## Proposal

Following the ququart spike's answer: four planes (x hi/lo, z hi/lo); the
symplectic form x·z′ − z·x′ mod 4; isotropy; |S| and logical dimension; weight
as the popcount of the OR of all four planes; exact distance with certificates,
using the stabilizer spike's method.

## Acceptance criteria

- [ ] Matches brute force on every ququart code with n ≤ 5 in a seeded random sample of 1,000.
- [ ] Reproduces any example code with published parameters that the ququart spike found.
- [ ] Rejects non-isotropic generators with the offending pair named.
