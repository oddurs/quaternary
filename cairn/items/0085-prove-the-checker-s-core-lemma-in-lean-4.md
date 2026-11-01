---
id: 85
uid: 7037f9cf-d78a-451b-b000-f8a20cfd5e34
title: Prove the checker's core lemma in Lean 4
type: feature
status: backlog
milestone: later
created: 2026-11-01
updated: 2026-11-01
priority: p2
effort: l
area: checker
---

## Problem

Deferred: it matters once outside groups rely on our certificates. Until then,
an independent checker plus a second implementation is the trust argument.
Precedent: Lean-QEC proves CSS distances in the Lean kernel (arXiv 2605.16523);
Keller's conjecture and the empty hexagon used verified checkers.

## Proposal

State and prove in Lean that if the pieces cover the code and each replay finds
no codeword lighter than d, the minimum distance is at least d. Link the
checker's logic to the statement.

## Acceptance criteria

- [ ] The lemma is proved in Lean 4 with no `sorry`, and checked in CI.
