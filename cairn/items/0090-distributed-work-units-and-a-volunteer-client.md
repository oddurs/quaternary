---
id: 90
uid: ed0d4d13-768e-4de7-91de-60f37e3728ba
title: Distributed work units and a volunteer client
type: feature
status: backlog
milestone: later
created: 2026-11-01
updated: 2026-11-01
priority: p3
effort: xl
area: frontier
---

## Problem

Deferred: one machine suffices until a single verification takes more than a
day. Parallel Brouwer–Zimmermann has turned five days into five minutes
(arXiv 1911.08963); the plan format already splits into pieces.

## Proposal

Pieces of a plan as hashed work units, each checked twice on different machines,
GIMPS-style.

## Acceptance criteria

- [ ] A certificate assembled from distributed units verifies identically to a local run.
