---
id: 78
uid: 0b32206d-14fc-49ba-bae2-bbe4b14f0b41
title: Nightly replay of every ledger certificate
type: chore
status: backlog
milestone: v1.0
depends_on:
- 39
- 62
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: ledger
---

## Purpose

The ledgers' claim is that anyone can replay them. That has to be checked
continuously, not once.

## Approach

A scheduled CI job: clean clone, build, `q4 ledger verify` over every cell.

## Acceptance criteria

- [ ] The nightly job is green, and a failure opens an issue automatically.
- [ ] The one-command replay is documented in the README.
