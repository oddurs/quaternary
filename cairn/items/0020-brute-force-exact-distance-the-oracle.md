---
id: 20
uid: 51dff9e6-e999-4c52-bf1b-753848dacf26
title: 'Brute-force exact distance: the oracle'
type: feature
status: backlog
milestone: v0.1
depends_on:
- 16
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: engine
cell:
- z4
---

## Problem

Before anything fast, we need an answer that is obviously right.

## Proposal

Enumerate codewords in Gray-code order over the coefficient vector, so each step
is a single vector add. Track the minimum weight per metric and a witness.
Parallelise over the top coefficient. Return `Distance { metric, d, witness,
status: Exact }`. Refuse codes above a size limit (default 2^32 codewords) with
an error that names the codeword count and points at the fast engine.

## Acceptance criteria

- [ ] Matches every corpus value in all three metrics.
- [ ] The witness is a codeword and its weight equals d (checked by the independent checker in its tests).
- [ ] Single-threaded and parallel runs return the same d on every corpus code.
- [ ] An oversized code exits 3 with the codeword count in the message.
