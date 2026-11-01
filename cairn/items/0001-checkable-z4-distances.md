---
id: 1
uid: f64df48e-7518-47cf-8550-343482ea9057
key: v0.1
title: Checkable Z4 distances
type: milestone
status: backlog
created: 2026-11-01
updated: 2026-11-01
priority: p2
due: 2026-11-29
---

## Ships

A researcher installs `q4` with one command, gives it a Z4-linear code, and gets
its exact minimum Lee, Hamming and Euclidean distance with a certificate that a
separate checker verifies. Brute force only, so small codes only, but every
answer is proved and says how it was proved.

## Done when

- [ ] `cargo install` from a clean checkout gives a working `q4`; CI is green on macOS and Linux.
- [ ] `q4 distance` agrees with every entry in the literature corpus.
- [ ] `q4 check` accepts every certificate the engine emits and rejects every certificate in the mutation suite.

## Explicitly not in this milestone

- Fast enumeration (v0.2). GF(4) and quantum codes (v0.3). Python (v0.2). Any table or audit (v0.2).
