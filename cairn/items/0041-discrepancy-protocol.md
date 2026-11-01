---
id: 41
uid: 0a9c56c2-5988-4b43-8da4-6e32f21ba591
title: Discrepancy protocol
type: feature
status: backlog
milestone: v0.2
depends_on:
- 40
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: s
area: ledger
cell:
- z4
---

## Problem

Claiming a published table is wrong is serious. It must be right, and it must
reach the maintainer before anyone else.

## Proposal

Each discrepancy is re-run on a second machine, its certificate re-checked by
the independent checker, and a minimal reproduction written: the code file, the
claim, our result, the certificate and the replay command.

## Acceptance criteria

- [ ] Each discrepancy has a reproduction bundle that replays from a clean checkout with one command.
- [ ] No discrepancy appears in anything public before the maintainer has been sent it.
