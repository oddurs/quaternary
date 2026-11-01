---
id: 14
uid: cc54fba5-e43a-476e-957e-12c19ea3fe84
title: 'Ququart stabilizer codes: confirm the Z4 symplectic formalism and look for an existing table'
type: spike
status: backlog
milestone: v0.1
created: 2026-11-01
updated: 2026-11-01
priority: p2
effort: s
area: theory
cell:
- ququart
---

## Question

For qudit dimension 4, are stabilizer codes on n ququarts exactly the
submodules S of Z4^(2n) that are isotropic under the symplectic form
⟨(x,z),(x′,z′)⟩ = x·z′ − z·x′ (mod 4), with logical dimension 4^n/|S| and
distance the minimum symplectic weight in S^⊥ \ S? What changes because Z4 is
not a field? And does anyone already publish a table of such codes?

## Why it matters

The v0.3 milestone and the claim that the fourth cell is empty both rest on this.
It is filed early on purpose: cheap to ask now, expensive to discover in March.

## Leading hypothesis

Yes to the formalism (the Z_d Pauli group generalises to composite d; see
Gottesman's qudit work and Gheorghiu 2014, "Standard form of qudit stabilizer
groups"). No public table: QDistRnd covers only prime-power fields
(https://github.com/QEC-pages/QDistRnd) and codetables.de covers qubits. Also
settle which ququarts hardware groups mean (Z4 Paulis or GF(4) qudits); they are
different cells.

## Timebox

One day.

## Answer

_Open._

## Acceptance criteria

- [ ] The Answer section states a conclusion and the evidence for it.
- [ ] The items this unblocks are created or updated to match the answer.
- [ ] The Answer cites the formalism's source and links any table found, or the searches that found none.
- [ ] If a table exists, the "first ququart table" item is rewritten as an audit.
