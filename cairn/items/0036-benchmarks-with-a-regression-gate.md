---
id: 36
uid: 8b5603e9-96b4-4ea8-a5a1-e49cd570ce6e
title: Benchmarks with a regression gate
type: chore
status: backlog
milestone: v0.2
depends_on:
- 33
created: 2026-11-01
updated: 2026-11-01
priority: p1
effort: s
area: infra
---

## Purpose

"Fast" has to be a measurement, and a regression has to fail CI rather than be
noticed months later.

## Approach

criterion benchmarks on the corpus; a stored baseline; CI compares against it.

## Acceptance criteria

- [ ] CI fails on a slowdown of more than 10% on any benchmark.
- [ ] The README states the lifted Golay timing with the hardware it was measured on.
