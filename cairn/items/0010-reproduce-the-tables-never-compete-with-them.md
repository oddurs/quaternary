---
id: 10
uid: a981c356-8175-404a-b56e-bcc28cfdc512
title: Reproduce the tables, never compete with them
type: decision
status: done
milestone: v0.1
created: 2026-11-01
updated: 2026-11-01
closed_at: 2026-11-01
---

## Context

The tables exist and have owners: codetables.de (Markus Grassl; GF(q)-linear and
qubit codes), the Aydin–Asamov Z4 database (quantumcodes.info/Z4, n ≤ 128), and
the Harada–Munemasa self-dual tables. A rival table without their communities
would die and would read as arrogance.

## Options considered

1. Our own bounds table.
2. Independent reproduction of theirs, with corrections sent back.

## Decision

Option 2. We audit, publish our recomputation keyed by their identifiers, mirror
only what a maintainer permits, report discrepancies to the maintainer privately
before publishing, and send improvements to them rather than keeping them. The
only table we create is the one nobody keeps: ququart stabilizer codes.

## Revisit when

A maintainer asks us to host their table.

## Acceptance criteria

- [x] The choice, its evidence and its consequences are recorded.
