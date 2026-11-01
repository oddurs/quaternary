---
id: 66
uid: 97df0d1b-76e2-49bb-9572-246f3b7cb8eb
title: Submission format and q4 submit
type: feature
status: backlog
milestone: v0.4
depends_on:
- 24
- 62
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: frontier
---

## Problem

Turning "I found a code" into something a maintainer will accept is tedious, so
people skip it.

## Proposal

`q4 submit CODE.q4` certifies the code, compares it with the ledger and writes a
record report: certificate, table comparison, replay command, and a draft
message to the maintainer. When it is not an improvement, it says which known
entry it matches or falls short of.

## Acceptance criteria

- [ ] Submitting a known table code reports "matches entry …", not a record.
- [ ] Submitting a code that beats a seeded fake ledger entry produces a complete record report.
