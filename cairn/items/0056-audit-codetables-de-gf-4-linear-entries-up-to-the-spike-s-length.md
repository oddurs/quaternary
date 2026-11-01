---
id: 56
uid: 8261d567-64d7-42d7-87bf-09846aecb56c
title: 'Audit codetables.de: GF(4)-linear entries up to the spike''s length'
type: feature
status: backlog
milestone: v0.4
depends_on:
- 39
- 41
- 49
- 55
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: ledger
cell:
- gf4
---

## Problem

The GF(4) cell's ledger.

## Proposal

For every (n, k) up to the codetables spike's N, a ledger record whose existence
side is certified by our engine, whose upper bound is `cited` with its source,
and whose table status is set accordingly.

## Acceptance criteria

- [ ] Every (n, k) in range has a record.
- [ ] Every `verified` existence side has a certificate the checker accepts.
- [ ] Discrepancies go through the discrepancy protocol before anything is published.
