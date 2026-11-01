---
id: 38
uid: b94c883a-7083-432d-9e89-b9e07e53c0d8
title: Ingest the Z4 database
type: feature
status: backlog
milestone: v0.2
depends_on:
- 18
- 37
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: ledger
cell:
- z4
---

## Problem

The audit needs every entry as a code plus its claimed distances.

## Proposal

An ingester for the format the database spike found, reusing the Magma reader
where it applies, producing one `Z4Code` and its claimed values per entry.

## Acceptance criteria

- [ ] Every entry either ingests or appears in an ingest report with the reason it did not.
- [ ] Re-running the ingest on the same export gives byte-identical output.
