---
id: 47
uid: 25742552-33ad-4bfd-9d84-e91c446467ca
title: 'GF(4)-linear codes: RREF, information sets, Euclidean and Hermitian duals'
type: feature
status: backlog
milestone: v0.3
depends_on:
- 17
- 46
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: core
cell:
- gf4
---

## Problem

Brouwer–Zimmermann needs row reduction and a supply of information sets, and
GF(4) carries two duals that matter (Hermitian duality is the one quantum codes use).

## Proposal

`Gf4Code` with RREF, rank, a sequence of disjoint information sets, both duals,
and the `gf4` header in the `.q4` format (symbols `0 1 w w2`).

## Acceptance criteria

- [ ] Both duals verified by full enumeration on random codes with n ≤ 8.
- [ ] Information sets returned have full rank (checked) and are disjoint where the length allows.
