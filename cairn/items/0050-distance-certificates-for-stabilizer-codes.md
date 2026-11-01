---
id: 50
uid: eac2fd87-ff27-484d-b524-33622fc73a48
title: Distance certificates for stabilizer codes
type: spike
status: backlog
milestone: v0.3
depends_on:
- 13
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: s
area: theory
cell:
- qubit
- ququart
---

## Question

A stabilizer code's distance is the minimum weight in N(S) \ S, a set
difference rather than a linear code. What enumeration certifies "no logical
operator of weight below d", and what does the checker replay?

## Why it matters

Both quantum cells (qubit and ququart) depend on the answer.

## Leading hypothesis

Run Brouwer–Zimmermann over N(S) with symplectic weight and reject candidates in
S by a membership test; the certificate carries S's generators, the plan over
N(S) and a witness logical. Prior art: QDistRnd (randomised, upper bounds only),
Magma (exact, closed), Lean-QEC (SAT with LRAT proofs for CSS codes,
arXiv 2605.16523).

## Timebox

Two days.

## Answer

_Open._

## Acceptance criteria

- [ ] The Answer section states a conclusion and the evidence for it.
- [ ] The items this unblocks are created or updated to match the answer.
