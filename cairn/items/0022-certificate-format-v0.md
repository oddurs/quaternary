---
id: 22
uid: 4bf2c176-b96a-4e26-8ba9-3ab6cb23c008
title: Certificate format v0
type: feature
status: backlog
milestone: v0.1
depends_on:
- 12
- 13
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: cert
cell:
- z4
---

## Problem

A distance with nothing to check it against is a claim, not a result.

## Proposal

Implement the format the certificate spike settles, in `q4-cert`: serde types,
JSON with canonical key order, `"format": "q4-cert/0"`, holding the code in
standard form, metric, claim, witness and plan `all`. The crate depends on
nothing else in the workspace.

## Acceptance criteria

- [ ] A JSON Schema for the format is committed and CI validates every emitted certificate against it.
- [ ] Serialising the same certificate twice is byte-identical.
- [ ] An unknown format version is rejected with the version in the message.
- [ ] The spike's worked octacode certificate round-trips unchanged.
