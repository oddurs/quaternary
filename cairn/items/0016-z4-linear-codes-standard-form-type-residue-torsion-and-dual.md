---
id: 16
uid: 587747f0-bd82-4669-a83a-ef8aa6c931c0
title: 'Z4-linear codes: standard form, type, residue, torsion and dual'
type: feature
status: backlog
milestone: v0.1
depends_on:
- 15
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: core
cell:
- z4
---

## Problem

Distance, certificates and the dual all need a canonical way to hold a code and
to enumerate its codewords exactly once.

## Proposal

`Z4Code::from_generators(rows)` reduces to standard form

    [ I_k1  A     B  ]
    [ 0     2I_k2 2C ]

up to a recorded column permutation. Expose the type (k1, k2), the size
4^k1 · 2^k2, the residue code (binary [n, k1]), the torsion code (binary
[n, k1+k2]) and the dual's generator (type (n−k1−k2, k2)). A codeword iterator
over coefficient vectors in Z4^k1 × Z2^k2.

## Acceptance criteria

- [ ] A generator with redundant and dependent rows yields the correct type.
- [ ] The iterator yields exactly 4^k1 · 2^k2 distinct codewords on every corpus code.
- [ ] The dual has type (n−k1−k2, k2), and the dual of the dual equals the code by full enumeration on random codes with n ≤ 8.
- [ ] Row operations never change the codeword set (property test).
