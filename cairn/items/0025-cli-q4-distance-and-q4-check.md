---
id: 25
uid: 2dd60c24-056d-43fd-9b37-f6adb8309a8b
title: 'CLI: q4 distance and q4 check'
type: feature
status: backlog
milestone: v0.1
depends_on:
- 17
- 23
- 24
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: cli
cell:
- z4
---

## Problem

The user-facing promise of v0.1 is two commands.

## Proposal

    q4 distance FILE [--metric lee|hamming|euclidean|all] [--cert OUT] [--json]
    q4 check CERT [--json]

`q4 distance` prints e.g. `n=8 type=4^4 d_L=6 (exact)`. `q4 check` prints
`ok: d_L = 6 (exact)` or `rejected: <reason>`. Exit codes: 0 ok, 1 rejected,
2 bad input, 3 too large for this engine.

## Acceptance criteria

- [ ] Every exit code is covered by an integration test.
- [ ] Every printed distance carries its status word, in text and JSON output.
- [ ] `q4 --help` lists both commands, with an example each.
