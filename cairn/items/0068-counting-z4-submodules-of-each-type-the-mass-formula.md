---
id: 68
uid: 4e4a5f8a-3cd9-46cb-bbb3-d5922ed05f7f
title: 'Counting Z4 submodules of each type: the mass formula'
type: spike
status: backlog
milestone: v0.4
created: 2026-11-01
updated: 2026-11-01
priority: p1
effort: s
area: theory
cell:
- z4
---

## Question

What is the closed form for the number of submodules of Z4^n of type (k1, k2)?
Together with the monomial group's order 2^n · n!, this gives the completeness
check Σ over classes of 2^n · n! / |Aut(C)| = N(n, k1, k2).

## Why it matters

Without it, a classification can be missing codes and nobody would know. With it,
completeness is certified.

## Leading hypothesis

It follows from the classical count of subgroups of a finite abelian p-group by
type (Birkhoff; Delsarte–Butler), specialised to (Z/4)^n. Verify against brute
force for n ≤ 5 before trusting it.

## Timebox

Two days.

## Answer

_Open._

## Acceptance criteria

- [ ] The Answer section states a conclusion and the evidence for it.
- [ ] The items this unblocks are created or updated to match the answer.
- [ ] The formula matches brute-force submodule counts for every type with n ≤ 5.
