---
id: 51
uid: 8e676dea-29de-465e-8857-fb1e18e3ce3b
title: Qubit stabilizer codes as additive GF(4) codes
type: feature
status: backlog
milestone: v0.3
depends_on:
- 46
- 50
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: l
area: engine
cell:
- qubit
---

## Problem

The qubit cell: the most used of the four, and the one with the most open tools
to check against.

## Proposal

Symplectic representation on two planes (`x`, `z`); an isotropy check; logical
dimension; symplectic weight `popcount(x|z)`; exact distance by the stabilizer
spike's method, with certificates. A `qubit` header in `.q4`, with Pauli-string input.

## Acceptance criteria

- [ ] [[5,1,3]], Steane [[7,1,3]], Shor [[9,1,3]] and [[13,1,5]] give the right parameters.
- [ ] Agrees with QDistRnd's upper bounds on 100 random small codes and never reports a d above them.
- [ ] Rejects non-commuting generators with the offending pair named.
