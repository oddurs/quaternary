---
id: 13
uid: b227692d-ed67-4ea6-a262-75820a924a1f
title: Certificate shape for Z4 distance claims
type: spike
status: backlog
milestone: v0.1
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: cert
cell:
- z4
---

## Question

What must a certificate for "the minimum Lee (or Hamming, or Euclidean)
distance of this Z4-linear code is exactly d" contain so that a checker far
simpler than the engine can verify it? Z4 codes are generally not free modules
(type 4^k1 2^k2), so binary information-set arguments do not transfer directly,
and the Gray image of a Z4-linear code is usually nonlinear.

## Why it matters

The certificate format is the product. v0.1 certifies brute force; v0.2 must
certify coset enumeration without breaking the format.

## Leading hypothesis

A certificate is {format version, code as a generator in standard form with
its type, metric, claimed d, a witness codeword with its coefficient vector, an
enumeration plan}. Plan `all`: the checker enumerates all 4^k1 · 2^k2
codewords. Plan `cosets` (v0.2): a list of pieces, each a coset of the binary
linear kernel of the Gray image (Villanueva–Zeng–Pujol, DCC 2015,
https://ddd.uab.cat/record/142404) with a weight threshold to replay; the
checker verifies by counting that the pieces cover the code, then replays each.

## Timebox

Two days.

## Answer

_Open._

## Acceptance criteria

- [ ] The Answer section states a conclusion and the evidence for it.
- [ ] The items this unblocks are created or updated to match the answer.
- [ ] The Answer includes a fully worked certificate for the octacode.
- [ ] The Answer states the checker's cost relative to the engine's for both plans.
