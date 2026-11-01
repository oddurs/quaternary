---
id: 55
uid: 27269250-7d78-4dc4-a492-7d483123e83f
title: What codetables.de exposes per entry, and how far the audit can reach
type: spike
status: backlog
milestone: v0.4
depends_on:
- 48
- 54
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: s
area: ledger
cell:
- gf4
- qubit
---

## Question

For GF(4)-linear and qubit entries: is a generator or construction available per
entry, and in what form? Up to which length n can our engine certify the
existence side within a day of compute?

## Why it matters

It sets the audit's scope (the N in the two audit items) and its ingest work.

## Leading hypothesis

Many entries give a construction chain rather than a matrix; a matrix can be
rebuilt for small n. The existence side is feasible to roughly n ≤ 30 for GF(4).

## Timebox

Two days.

## Answer

_Open._

## Acceptance criteria

- [ ] The Answer section states a conclusion and the evidence for it.
- [ ] The items this unblocks are created or updated to match the answer.
