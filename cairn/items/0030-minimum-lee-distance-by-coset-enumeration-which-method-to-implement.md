---
id: 30
uid: 4192a7a2-a00d-466f-8629-2b4c26159d29
title: 'Minimum Lee distance by coset enumeration: which method to implement'
type: spike
status: backlog
milestone: v0.2
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: engine
cell:
- z4
---

## Question

Which published route to exact minimum Lee distance for Z4 codes should the
fast engine implement? (a) the binary linear kernel of the Gray image plus its
cosets (Villanueva–Zeng–Pujol, DCC 2015); (b) projection to binary
(Kiermaier–Wassermann 2008, used for Z4 QR codes); (c) Brouwer–Zimmermann
adapted to the residue and torsion pair. Torres-Martín's 2024 UAB thesis covers
the homogeneous-weight case.

## Why it matters

The fast engine is the critical path for the audit, and the chosen method fixes
the certificate plan the checker has to replay.

## Leading hypothesis

(a). The Gray image is a union of cosets of its linear kernel K, a binary
linear code, so binary Brouwer–Zimmermann runs on each coset with the bit-plane
inner loop, and the cover is countable, which suits the checker.

## Timebox

Three days, including a throwaway prototype.

## Answer

_Open._

## Acceptance criteria

- [ ] The Answer section states a conclusion and the evidence for it.
- [ ] The items this unblocks are created or updated to match the answer.
- [ ] The Answer reports the prototype's time on the lifted Golay code next to brute force.
