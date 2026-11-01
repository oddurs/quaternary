---
id: 34
uid: a4693a02-e679-438e-bf45-f01cd546be83
title: 'Threshold queries: is d at least D?'
type: feature
status: backlog
milestone: v0.2
depends_on:
- 33
created: 2026-11-01
updated: 2026-11-01
priority: p1
effort: s
area: engine
cell:
- z4
---

## Problem

Searchers and auditors rarely need d itself; they need to know whether a code
reaches a target, and the answer is far cheaper.

## Proposal

`q4 distance FILE --at-least D`: stop at the first codeword lighter than D
(status `upper`, with that witness), or finish enumerating up to weight D−1
(status `lower`: d ≥ D). Both answers come with certificates.

## Acceptance criteria

- [ ] On every corpus code, `--at-least d` reports `lower` and `--at-least d+1` reports `upper`.
- [ ] Both certificates are accepted by the checker.
