---
id: 21
uid: 3de9f212-f9c2-46cf-96ca-0e318a9706c9
title: Weight enumerators with the MacWilliams identity as a test
type: feature
status: backlog
milestone: v0.1
depends_on:
- 20
created: 2026-11-01
updated: 2026-11-01
priority: p1
effort: m
area: engine
cell:
- z4
---

## Problem

Enumeration, duals and weights can each be subtly wrong. The MacWilliams identity
ties all three together and catches all of them.

## Proposal

Compute the symmetrized weight enumerator swe(W, X, Y), counting zeros, ±1s and
2s, by brute force. Test
swe_{C⊥}(W, X, Y) = (1/|C|) · swe_C(W + 2X + Y, W − Y, W − 2X + Y)
with exact big-integer polynomial arithmetic. `q4 enumerator FILE` prints the Lee
weight distribution.

## Acceptance criteria

- [ ] The identity holds exactly for every corpus code and 1,000 seeded random codes with n ≤ 10.
- [ ] `q4 enumerator` output for the octacode matches its published Lee weight distribution.
