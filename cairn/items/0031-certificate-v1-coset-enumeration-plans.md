---
id: 31
uid: ce8a1252-0561-4a10-82a1-be2eb66ca51c
title: 'Certificate v1: coset-enumeration plans'
type: feature
status: backlog
milestone: v0.2
depends_on:
- 22
- 30
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: cert
cell:
- z4
---

## Problem

Plan `all` cannot certify codes too large to enumerate.

## Proposal

Add plan `cosets` as the coset spike decided: a list of pieces, each naming a
coset of the kernel and the weight threshold enumerated in it, plus the counting
data that shows the pieces cover the code. The format version becomes
`q4-cert/1`; v0 certificates stay valid.

## Acceptance criteria

- [ ] The JSON Schema covers both plans, and CI validates every emitted certificate against it.
- [ ] Every v0 certificate in the test corpus still parses and verifies.
