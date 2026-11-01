---
id: 18
uid: f1e8c4ba-8bd7-4d82-adcd-6c30649c291e
title: Read Magma Z4 code literals
type: feature
status: backlog
milestone: v0.1
depends_on:
- 17
created: 2026-11-01
updated: 2026-11-01
priority: p1
effort: s
area: core
cell:
- z4
---

## Problem

Researchers keep codes as Magma; Harada–Munemasa and Aydin distribute Magma
files. Retyping them is where errors come from.

## Proposal

Parse the forms used in real files, at least `LinearCode<Integers(4), n | [...]>`
and `LinearCode(Matrix(Integers(4), r, n, [...]))`, into `Z4Code`. Take test
files from https://www.math.is.tohoku.ac.jp/~mharada/research/codes/sd4.htm.

## Acceptance criteria

- [ ] At least three real Harada–Munemasa files parse into codes of the stated size and length.
- [ ] Unsupported syntax names the construct it could not read, with its position.
