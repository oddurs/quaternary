---
id: 53
uid: 927eee8e-0273-46b9-b564-272ef7582e1c
title: Checker replays stabilizer-code plans
type: feature
status: backlog
milestone: v0.3
depends_on:
- 51
- 52
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: checker
cell:
- qubit
- ququart
---

## Problem

Both quantum engines emit certificates that nothing independent has checked.

## Proposal

In `q4-check`: verify isotropy of S, membership tests, and replay the plan over
N(S), for both the qubit and the ququart case.

## Acceptance criteria

- [ ] Accepts every certificate both quantum engines emit for their tests.
- [ ] Rejects a witness that lies in S (weight-0 logical) and a non-isotropic S, each with a specific reason.
