---
id: 62
uid: 6532ecb4-f3e7-4694-aa3a-1796d6e50153
title: Gap list for every cell
type: feature
status: backlog
milestone: v0.4
depends_on:
- 40
- 56
- 57
- 58
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: frontier
---

## Problem

Searchers need to know where to look, and the gaps come straight from the ledgers.

## Proposal

From each ledger: per (n, k), the best certified lower bound, the cited upper
bound and the width. Output JSON and a static HTML table sorted by width, per
cell. `q4 gaps --cell z4`.

## Acceptance criteria

- [ ] Regenerates from the ledgers with one command, deterministically.
- [ ] Every lower bound links to its certificate and every upper bound to its citation.
