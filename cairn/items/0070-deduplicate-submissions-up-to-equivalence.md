---
id: 70
uid: 1758e3c8-271b-4b8a-86a5-7bb17e270557
title: Deduplicate submissions up to equivalence
type: feature
status: backlog
milestone: v0.4
depends_on:
- 67
- 69
created: 2026-11-01
updated: 2026-11-01
priority: p1
effort: s
area: frontier
cell:
- z4
---

## Problem

The same code arrives permuted and sign-flipped, and looks new.

## Proposal

`q4 submit` canonicalises Z4 codes and reports "equivalent to entry …" when a
submission matches a ledger code.

## Acceptance criteria

- [ ] A permuted, negated copy of a ledger code is reported as equivalent, naming the entry.
