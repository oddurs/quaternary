---
id: 43
uid: f417a3f9-be16-46fd-9785-c11b1602b978
title: Publish the Z4 audit
type: feature
status: backlog
milestone: v0.2
depends_on:
- 40
- 42
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: ledger
cell:
- z4
---

## Problem

An audit nobody can read did not happen.

## Proposal

A static report generated from the ledger: summary, per-entry statuses keyed by
the database's own identifiers, certificates, and how to replay them. Mirror
entries only to the extent the maintainers permitted.

## Acceptance criteria

- [ ] The report regenerates from the ledger with one command.
- [ ] What is mirrored matches the recorded permission terms exactly.
- [ ] Published only after the maintainer outreach item is closed.
