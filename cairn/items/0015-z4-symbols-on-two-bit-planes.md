---
id: 15
uid: 5a1bb1cd-73ac-4e4e-94d8-6d7b7244344d
title: Z4 symbols on two bit planes
type: feature
status: backlog
milestone: v0.1
depends_on:
- 12
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: core
cell:
- z4
---

## Problem

Every algorithm here runs on vectors of Z4 symbols. The memory layout decides
both speed and how much the Gray map costs.

## Proposal

`Z4Vec { hi: Vec<u64>, lo: Vec<u64>, len: usize }`, symbol a = 2·hi + lo,
64 symbols per word. Operations, each a few instructions per word:

- add: `lo = a0^b0`, `hi = a1^b1^(a0&b0)`
- negate: `lo = a0`, `hi = a1^a0`
- elementwise multiply: `hi = a1&b0 ^ a0&b1`, `lo = a0&b0`
- Lee weight `popcount(hi) + popcount(hi^lo)`; Hamming `popcount(hi|lo)`;
  Euclidean `popcount(lo) + 4·popcount(hi & !lo)`
- Gray map to binary length 2n: `(hi, hi^lo)`

Bits past `len` in the last word are always zero.

## Acceptance criteria

- [ ] add, negate and multiply match integer arithmetic mod 4 on all 16 symbol pairs, exhaustively.
- [ ] Each weight matches the per-symbol tables Lee (0,1,2,1), Hamming (0,1,1,1), Euclidean (0,1,4,1) on random vectors of lengths 1, 63, 64, 65 and 200.
- [ ] Lee distance equals the Hamming distance of Gray images on 10,000 random pairs.
- [ ] The zero-tail invariant is asserted in debug builds after every operation.
