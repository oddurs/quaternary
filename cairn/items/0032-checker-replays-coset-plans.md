---
id: 32
uid: 2a4a22e3-944e-4756-9871-24e0c93f769d
title: Checker replays coset plans
type: feature
status: backlog
milestone: v0.2
depends_on:
- 23
- 31
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: checker
cell:
- z4
---

## Problem

A v1 certificate is only as good as the checker's replay of its plan.

## Proposal

In `q4-check`, still with naive arithmetic: verify that the pieces' sizes sum to
|C| and that the pieces are distinct cosets; replay each piece up to its
threshold; confirm the witness.

## Acceptance criteria

- [ ] Accepts every v1 certificate the engine emits for the corpus.
- [ ] Rejects a certificate with one piece removed, one duplicated, or one threshold raised, each with a specific reason.
