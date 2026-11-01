---
id: 49
uid: e6411879-521e-48b4-a5a8-4ae3680b834c
title: Checker replays information-set plans
type: feature
status: backlog
milestone: v0.3
depends_on:
- 32
- 48
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: checker
cell:
- gf4
---

## Problem

The GF(4) engine's certificates need the independent checker too.

## Proposal

In `q4-check`, with GF(4) as a 4×4 lookup table: verify each information set's
rank, verify the enumeration bound arithmetic, replay.

## Acceptance criteria

- [ ] Accepts every certificate the GF(4) engine emits for its tests.
- [ ] Rejects a certificate with a rank-deficient set or an overstated bound, each with a specific reason.
