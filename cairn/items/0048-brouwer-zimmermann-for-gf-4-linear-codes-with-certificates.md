---
id: 48
uid: d1509393-e6b8-4360-8bba-9be3438a4fdf
title: Brouwer–Zimmermann for GF(4)-linear codes, with certificates
type: feature
status: backlog
milestone: v0.3
depends_on:
- 31
- 47
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: l
area: engine
cell:
- gf4
---

## Problem

The GF(4) cell needs exact distance at the lengths codetables.de lists.

## Proposal

Standard Brouwer–Zimmermann over multiple information sets with the bit-plane
inner loop. A new certificate plan `info-sets` lists the information sets and
the weight enumerated in each, so the checker can verify ranks and replay.

## Acceptance criteria

- [ ] Agrees with brute force on 10,000 seeded random codes with k ≤ 10.
- [ ] Reproduces the distance of at least 20 codetables.de constructions it was given.
- [ ] Its certificates validate against the JSON Schema.
