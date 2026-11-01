---
id: 45
uid: ba56b93e-bd9f-442b-8f36-38e5488c79e1
title: Certificate format specification
type: docs
status: backlog
milestone: v0.2
depends_on:
- 31
- 32
created: 2026-11-01
updated: 2026-11-01
priority: p1
effort: s
area: docs
---

## Reader and question

Someone who wants to check our certificates without running our code, in another
language.

## Change

`doc/certificates.md`: every field, both plans, the canonical serialisation, and
what the checker must verify, in order.

## Acceptance criteria

- [ ] The reader can complete the described task with the current program.
- [ ] A reference checker for plan `all`, written in Python from the spec alone, lives in `tools/` and passes the corpus in CI.
