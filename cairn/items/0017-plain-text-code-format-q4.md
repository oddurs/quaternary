---
id: 17
uid: 0363dd0f-8da6-41b9-9b58-03c89bede82c
title: Plain-text code format (.q4)
type: feature
status: backlog
milestone: v0.1
depends_on:
- 16
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: s
area: core
cell:
- z4
---

## Problem

The CLI needs an input format a person can write by hand and a diff can read.

## Proposal

A header line `z4 n=8`, then one generator row per line with symbols `0 1 2 3`
separated by spaces, `#` starting a comment. The reader reports errors with
`file:line:col`. The writer emits standard form. The header words `gf4`,
`qubit` and `ququart` are reserved now so v0.3 does not break the format.

## Acceptance criteria

- [ ] read → write → read is the identity on every corpus file.
- [ ] A row of the wrong length or a symbol 5 reports `file:line:col` and the CLI exits 2.
- [ ] A reserved header gives "not supported yet: gf4", not a parse error.
