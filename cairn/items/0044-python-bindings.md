---
id: 44
uid: 9d07bda4-27e5-4cfe-8711-f42afb833d94
title: Python bindings
type: feature
status: backlog
milestone: v0.2
depends_on:
- 24
- 33
created: 2026-11-01
updated: 2026-11-01
priority: p1
effort: m
area: python
cell:
- z4
---

## Problem

Coding theorists and quantum researchers work in Python and Sage notebooks.

## Proposal

PyO3 and maturin:

    import quaternary as q4
    c = q4.Z4Code.from_file("octacode.q4")
    r = c.distance("lee")        # r.d, r.status, r.certificate
    q4.check(r.certificate)

## Acceptance criteria

- [ ] Wheels build in CI for macOS and Linux.
- [ ] A pytest suite reproduces every corpus value through the bindings.
