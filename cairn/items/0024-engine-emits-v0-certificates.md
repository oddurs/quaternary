---
id: 24
uid: 62ef5e17-3835-435a-9427-9b6c6099a854
title: Engine emits v0 certificates
type: feature
status: backlog
milestone: v0.1
depends_on:
- 20
- 22
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: s
area: engine
cell:
- z4
---

## Problem

The brute-force oracle returns a `Distance`; nothing turns it into something the
checker can read.

## Proposal

`Distance::certificate(&code) -> q4_cert::Certificate`, converting standard form,
witness and plan into the certificate types.

## Acceptance criteria

- [ ] Every corpus result converts to a certificate that the independent checker accepts.
