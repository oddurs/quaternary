---
id: 19
uid: 312360fa-8d47-45fe-9dfe-eab42c174330
title: Known-code corpus with literature values
type: feature
status: backlog
milestone: v0.1
depends_on:
- 17
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: s
area: engine
cell:
- z4
---

## Problem

An engine tested against itself proves nothing. Expected values must come from
the literature, never from our own output.

## Proposal

`corpus/z4/*.q4` plus `corpus/z4/index.toml` holding n, type, d_L, d_H, d_E and
a citation per entry. Candidates; confirm each value against its source before
committing it:

- octacode: n = 8, type 4^4, d_L = 6, d_H = 4, d_E = 8 (Forney–Sloane–Trott 1993; Hammons et al. 1994)
- Z4 Kerdock, n = 32, type 4^6, d_L = 28 (its Gray image is binary Kerdock (64, 2^12, 28))
- lifted Golay code, n = 24, type 4^12, d_L = 12, d_E = 16 (Bonnecaze–Solé–Calderbank 1995)
- several self-dual codes from the Harada–Munemasa tables with their stated minimum weights

## Acceptance criteria

- [ ] At least 10 entries, each with a citation and a page or table reference.
- [ ] A test runs every entry up to 2^26 codewords through brute force and compares all three distances.
