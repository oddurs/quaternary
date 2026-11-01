---
id: 84
uid: d9ceb64b-54fb-4d33-8202-cd4103a5ea23
title: Atlas pages generated from the ledger
type: feature
status: backlog
milestone: later
depends_on:
- 71
- 83
created: 2026-11-01
updated: 2026-11-01
priority: p2
effort: l
area: atlas
---

## Problem

Deferred: the atlas is a view of the ledger, worth building only once the ledger
and canonical forms exist. A page per code: invariants, each with its status;
links to the dual, Gray image, residue and torsion codes; lattice links via
Construction A mod 4 (octacode → E8, lifted Golay → Leech) out to Nebe–Sloane
and LMFDB.

## Proposal

A static site generated from ledger files, under the label scheme.

## Acceptance criteria

- [ ] Every ledger code has a page, regenerated with one command.
- [ ] Every invariant on a page shows its status and its certificate or citation.
