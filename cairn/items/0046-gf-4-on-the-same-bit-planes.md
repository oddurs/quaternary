---
id: 46
uid: 08dce4b0-a267-474a-bacc-1b8b14e317a9
title: GF(4) on the same bit planes
type: feature
status: backlog
milestone: v0.3
depends_on:
- 15
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: s
area: core
cell:
- gf4
- qubit
---

## Problem

Three of the four cells live over GF(4) or its additive group.

## Proposal

Element = a1·ω + a0 with ω² = ω + 1, held in the same `hi`/`lo` planes:

- add: XOR both planes
- multiply: `hi = a1b1 ^ a1b0 ^ a0b1`, `lo = a1b1 ^ a0b0`
- Frobenius (conjugation) x ↦ x²: `hi = a1`, `lo = a1 ^ a0`, the same bit operation as the Z4 Gray map
- trace x + x²: the `hi` plane

## Acceptance criteria

- [ ] Add, multiply, conjugate and trace match a lookup table on all symbol pairs, exhaustively.
- [ ] Field laws (distributivity, x^4 = x, nonzero inverses) hold under property tests.
