---
id: 33
uid: ec955876-af5e-4043-9fdb-9b5060d2598a
title: Fast exact Lee distance engine
type: feature
status: backlog
milestone: v0.2
depends_on:
- 20
- 30
- 31
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: l
area: engine
cell:
- z4
---

## Problem

Brute force stops at around 2^32 codewords; the database runs to n = 128.

## Proposal

Implement the method from the coset spike with the bit-plane inner loop, run in
parallel over pieces, emitting `q4-cert/1` certificates. Hamming and Euclidean
distance use the same machinery where the spike says it applies, and fall back
to brute force otherwise.

## Acceptance criteria

- [ ] Agrees with brute force on every corpus code and on 10,000 seeded random codes with n ≤ 14.
- [ ] Certifies the lifted Golay code within the time the spike measured, or explains the gap in a note.
- [ ] Every result's certificate is accepted by the independent checker.
