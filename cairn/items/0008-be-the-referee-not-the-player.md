---
id: 8
uid: 57cc3736-7c6e-4950-90a4-2fd320dfdcb7
title: Be the referee, not the player
type: decision
status: done
milestone: v0.1
created: 2026-11-01
updated: 2026-11-01
closed_at: 2026-11-01
---

## Context

One proposal was to build the best search for record-breaking codes. Small-length
GF(4) gaps are old: [19,8] has stood at 8–9 since a 2007 Griesmer step. Z4
records still fall (An–Kim–Ling 2026 improved 12 database entries,
arXiv 2606.08964), but each needs an exact Lee distance computed outside Magma.

## Options considered

1. Build the best searcher.
2. Build what every searcher needs: an open, sound verifier and a public list of gaps.

## Decision

Option 2. Anyone (a group, a student, an AI program-search system) submits a
code; the engine certifies it; improvements go to the table maintainers with the
certificate attached. Our own search is a logged baseline that keeps the board
busy, never the product. Model: bbchallenge, many untrusted searchers and one
trusted checker.

## Revisit when

Six months after v0.4 with no outside submission.

## Acceptance criteria

- [x] The choice, its evidence and its consequences are recorded.
