---
id: 11
uid: 393f2a87-4250-443a-865b-0b021ef633f8
title: Every claim carries its proof status
type: decision
status: done
milestone: v0.1
created: 2026-11-01
updated: 2026-11-01
closed_at: 2026-11-01
---

## Context

A table entry [n,k,d] has two sides. Existence (some code reaches d) needs a code
and a proof that its distance is at least d, which is an enumeration.
Nonexistence (no code beats d) needs a theorem, an LP/SDP dual certificate or an
exhaustive classification. A low-weight codeword only bounds one code's distance
from above; it settles nothing about the table.

## Options considered

1. Print distances, as existing tools do.
2. Never print a distance without saying how it is known.

## Decision

Option 2. For a code's distance: `exact` (witness plus complete enumeration),
`lower` (enumeration incomplete, d ≥ x), `upper` (witness only, d ≤ x). For a
table entry: `verified`, `open` or `discrepancy`, with the existence and
nonexistence sides tracked separately and upper bounds marked `cited` until we
can prove them.

## Revisit when

Nonexistence certificates (Delsarte LP duals) land; they add a status, they do
not change these.

## Acceptance criteria

- [x] The choice, its evidence and its consequences are recorded.
