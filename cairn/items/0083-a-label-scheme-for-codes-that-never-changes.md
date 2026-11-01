---
id: 83
uid: 69c5ef1b-9439-4116-a0a1-8219cfec0ca9
title: A label scheme for codes that never changes
type: spike
status: backlog
milestone: later
created: 2026-11-01
updated: 2026-11-01
priority: p2
effort: s
area: atlas
---

## Question

How should a code be named so a paper can cite "Z4.8.4.0.a1" forever? LMFDB
labels are built from invariants; a hash of a canonical form is fragile until
its tie-breaking is specified independently of any implementation.

## Why it matters

The atlas is worthless if its labels drift.

## Leading hypothesis

Invariant-based prefix (cell, n, type) plus an ordinal assigned inside a completed classification.

## Timebox

Two days.

## Answer

_Open._

## Acceptance criteria

- [ ] The Answer section states a conclusion and the evidence for it.
- [ ] The items this unblocks are created or updated to match the answer.
