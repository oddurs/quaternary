---
id: 89
uid: 6185ce3a-4249-4962-b583-60e7e96e6730
title: Galois rings GR(4,m) and cyclic Z4 constructions
type: feature
status: backlog
milestone: later
created: 2026-11-01
updated: 2026-11-01
priority: p3
effort: l
area: core
cell:
- z4
---

## Problem

Deferred: cyclic Z4 codes come from Hensel lifts of binary cyclic codes, which
needs GR(4, m) at 2m bits per symbol, outside the two-plane layout. Valuable for
constructing codes, not for checking them.

## Proposal

GR(4, m) arithmetic and Hensel lifting of binary polynomials; constructors for
Z4 cyclic, QR and Kerdock codes from first principles.

## Acceptance criteria

- [ ] Constructs the octacode and Z4 Kerdock codes from first principles, and they match the corpus.
