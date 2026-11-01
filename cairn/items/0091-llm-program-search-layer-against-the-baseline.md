---
id: 91
uid: 053de341-797c-41da-9d3c-6fd32d1a6fb0
title: LLM program-search layer against the baseline
type: feature
status: backlog
milestone: later
created: 2026-11-01
updated: 2026-11-01
priority: p3
effort: l
area: frontier
---

## Problem

Deferred, and admitted only on evidence: FunSearch-style program search improved
cap sets and deletion codes, but no coding-table record from it was found. It
earns a place only by beating the classical baseline at an equal budget, and the
comparison is published either way.

## Proposal

A search layer that proposes generator programs, scored by threshold queries
against the gap list.

## Acceptance criteria

- [ ] A head-to-head run against the baseline at equal compute is published, whatever the result.
