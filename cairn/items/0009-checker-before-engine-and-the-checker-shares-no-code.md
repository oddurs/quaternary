---
id: 9
uid: e75ad0fe-0116-400e-b36e-1b2ea1053cea
title: Checker before engine, and the checker shares no code
type: decision
status: done
milestone: v0.1
created: 2026-11-01
updated: 2026-11-01
closed_at: 2026-11-01
---

## Context

A fast engine that is wrong, with a certificate attached, is worse than no
answer. A certificate only means something if what checks it is independent of
what produced it.

## Options considered

1. One crate, engine and verification sharing arithmetic.
2. A separate checker crate with its own naive arithmetic, written first.

## Decision

Option 2. `q4-check` depends only on the certificate data types. It does Z4 and
GF(4) arithmetic naively (one byte per symbol, `% 4`, lookup tables), with no
bit planes and no code from `q4-core` or `q4-engine`, and it stays small enough
to read in one sitting. Brute force is the oracle for the engine; the fast
engine is an optimisation that must agree with both.

## Revisit when

Never for the independence rule. The checker's core lemma may later be proved in
Lean (see `later`).

## Acceptance criteria

- [x] The choice, its evidence and its consequences are recorded.
