---
id: 71
uid: 91e54093-539f-4b21-b166-ffe03783890d
title: Reproduce the Z4 code classification for n ≤ 7
type: feature
status: backlog
milestone: v0.4
depends_on:
- 68
- 69
created: 2026-11-01
updated: 2026-11-01
priority: p1
effort: l
area: atlas
cell:
- z4
---

## Problem

Araya–Harada–Ito–Saito classified all Z4 codes up to equivalence for n ≤ 7
(N = 2, 9, 35, 145, 682, 3947, 29001; arXiv 1707.01356), using about 1,954
CPU-hours in Magma. Nobody has reproduced it in the open, and the sequence is
not in the OEIS.

## Proposal

Classify by canonical augmentation with our canonical form, with completeness
certified by the mass formula for every type.

## Acceptance criteria

- [ ] The counts per (n, k1, k2) match the paper's tables, or each difference is documented and reported to the authors.
- [ ] The mass check closes for every type.
- [ ] The sequence is submitted to the OEIS with the paper and our reproduction cited.
