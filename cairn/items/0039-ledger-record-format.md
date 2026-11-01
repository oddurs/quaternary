---
id: 39
uid: 917a39cc-2def-4f37-a8e7-6d8f5136d768
title: Ledger record format
type: feature
status: backlog
milestone: v0.2
depends_on:
- 31
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: ledger
---

## Problem

An audit needs one place per entry where the claim, our result, its status and
its proof meet.

## Proposal

One file per entry under `ledger/<cell>/`, git-diffable: source table, entry
identifier, citation; a reference to the code (a hash of its standard form);
claimed values; recomputed values with their status words; table status
`verified`, `open` or `discrepancy`; certificate path; engine version; wall time.
`q4 ledger verify` re-checks every referenced certificate with the independent
checker.

## Acceptance criteria

- [ ] A schema for the record is committed and CI validates every ledger file.
- [ ] `q4 ledger verify` fails if any certificate is missing or rejected.
