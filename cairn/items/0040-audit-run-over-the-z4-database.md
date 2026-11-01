---
id: 40
uid: 3fe86eaf-e670-4fd9-a59b-4c7894bd0e15
title: Audit run over the Z4 database
type: feature
status: backlog
milestone: v0.2
depends_on:
- 33
- 35
- 38
- 39
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: ledger
cell:
- z4
---

## Problem

The milestone's headline result.

## Proposal

Run every ingested entry through the fast engine under a per-entry budget, write
ledger records, and resume after interruption without redoing finished work.

## Acceptance criteria

- [ ] Every ingested entry has a ledger record with a status.
- [ ] A summary reports counts of verified, open and discrepancy.
- [ ] Two runs on the same input produce identical ledgers, wall times aside.
