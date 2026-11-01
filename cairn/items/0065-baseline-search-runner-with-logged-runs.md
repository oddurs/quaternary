---
id: 65
uid: 76604b4d-3fb6-43f4-bf48-1c46f4a6b7a7
title: Baseline search runner with logged runs
type: feature
status: backlog
milestone: v0.4
depends_on:
- 34
- 62
- 64
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: frontier
cell:
- z4
---

## Problem

The board needs a player even if nobody else plays yet, and every run must be
reproducible.

## Proposal

Draws candidates from the generator, kills them cheaply with threshold queries
against the gap list, and fully certifies the survivors. Logs seed, family,
budget, candidates tried and best result per (n, type). An improvement becomes a
submission file.

## Acceptance criteria

- [ ] A run replays from its log to the same result.
- [ ] Every claimed improvement carries a certificate the checker accepts.
