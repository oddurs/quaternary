---
id: 64
uid: 69a23caf-3c49-4b92-a0ea-2e2fc91c259e
title: Quasi-cyclic and quasi-twisted Z4 code generator
type: feature
status: backlog
milestone: v0.4
depends_on:
- 16
- 63
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: frontier
cell:
- z4
---

## Problem

The baseline search needs candidate codes from the families where records come from.

## Proposal

Generate the families the records spike names, parameterised by index, length
and generator polynomials, as `Z4Code` values.

## Acceptance criteria

- [ ] Reproduces at least three record codes from the spike's papers by construction, matching their stated parameters.
