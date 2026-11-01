---
id: 75
uid: 84fdb98a-1e2e-4f0e-9323-2ea236973d75
title: Freeze certificate format 1.0 with a compatibility promise
type: chore
status: backlog
milestone: v1.0
depends_on:
- 45
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: s
area: cert
---

## Purpose

People will cite certificates in papers. A certificate that stops verifying
breaks the citation.

## Approach

`doc/compatibility.md` states the promise: every certificate any release ever
emitted stays checkable. A golden corpus of certificates from every earlier
format version runs in CI.

## Acceptance criteria

- [ ] CI checks a v0.1-era certificate with the current checker, and passes.
- [ ] The promise document says exactly what may change and what may not.
