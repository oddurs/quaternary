---
id: 60
uid: edf91d29-1a54-444e-b1c9-54d940979646
title: CLI and Python across all four cells
type: feature
status: backlog
milestone: v0.3
depends_on:
- 44
- 48
- 49
- 51
- 52
- 53
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: s
area: cli
---

## Problem

The square only exists for users if the same two commands work on every cell.

## Proposal

`q4 distance` and `q4 check` dispatch on the `.q4` header; the Python bindings
gain `Gf4Code`, `QubitCode` and `QuquartCode` with the same `distance()` shape.

## Acceptance criteria

- [ ] One corpus code per cell passes through `q4 distance` and `q4 check` in an integration test.
- [ ] The same four pass through Python in pytest.
