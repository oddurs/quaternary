---
id: 59
uid: f0981d6f-dec8-47de-b359-811968c24f83
title: Export qubit stabilizer codes for Stim
type: feature
status: backlog
milestone: v0.3
depends_on:
- 51
created: 2026-11-01
updated: 2026-11-01
priority: p2
effort: s
area: cli
cell:
- qubit
---

## Problem

Quantum researchers simulate in Stim; a code we certify should drop straight in.

## Proposal

`q4 export --stim FILE` writes the stabilizers as Pauli strings, the input
`stim.Tableau.from_stabilizers` takes.

## Acceptance criteria

- [ ] A Python test loads the exported Steane code into `stim.Tableau.from_stabilizers(..., allow_underconstrained=True)` without error.
