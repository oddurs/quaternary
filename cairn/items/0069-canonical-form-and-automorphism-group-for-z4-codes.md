---
id: 69
uid: 8a8c2f34-e432-4e33-bf73-d21a95cc76b9
title: Canonical form and automorphism group for Z4 codes
type: feature
status: backlog
milestone: v0.4
depends_on:
- 16
created: 2026-11-01
updated: 2026-11-01
priority: p1
effort: l
area: atlas
cell:
- z4
---

## Problem

Without canonical forms we cannot tell a new code from a known one, dedupe
submissions, or classify.

## Proposal

Feulner's partition refinement for Z4 codes ("Canonization of linear codes over
Z4", Advances in Mathematics of Communications 5, 2011). CodeCan, Feulner's GPL
C++ implementation, is used only as an external test oracle run as a separate
process, never linked, because of its licence.

## Acceptance criteria

- [ ] Canonical forms agree for 1,000 random pairs of monomially equivalent codes.
- [ ] Canonical forms differ for every pair of inequivalent codes from a Harada–Munemasa list.
- [ ] |Aut(C)| agrees with CodeCan on every corpus code.
