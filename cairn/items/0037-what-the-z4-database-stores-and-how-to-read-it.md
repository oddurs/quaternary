---
id: 37
uid: 4ae90fd9-81b4-45e0-945d-defbb9558698
title: What the Z4 database stores, and how to read it
type: spike
status: backlog
milestone: v0.2
depends_on:
- 29
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: s
area: ledger
cell:
- z4
---

## Question

Per entry, what does the database hold (generator matrix, which distances,
construction, citation), in what export format, how many entries, and under what
terms?

## Why it matters

The ingester and the audit's scope depend entirely on the answer.

## Leading hypothesis

Generator matrices with Hamming, Lee and Euclidean minimum distances, n ≤ 128,
exported as Magma, so the Magma reader carries most of the ingest.

## Timebox

One day after access arrives.

## Answer

_Open._

## Acceptance criteria

- [ ] The Answer section states a conclusion and the evidence for it.
- [ ] The items this unblocks are created or updated to match the answer.
