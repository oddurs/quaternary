---
id: 23
uid: 6b6b28d0-ca57-4ab1-808e-a7eae71a9b79
title: Independent checker for v0 certificates
type: feature
status: backlog
milestone: v0.1
depends_on:
- 22
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: checker
cell:
- z4
---

## Problem

The checker is what makes a certificate worth having. It must be independent of
the engine and simple enough to audit by reading.

## Proposal

`q4-check` verifies a v0 certificate using only `q4-cert` types and its own
naive arithmetic (a byte per symbol, `% 4`):

1. Rebuild the codeword from the witness's coefficient vector and the generator, and check it matches.
2. Check the witness weight equals d under the stated metric.
3. Plan `all`: enumerate every codeword and check no nonzero one has weight below d.

Target under 500 lines.

## Acceptance criteria

- [ ] Accepts the engine's certificate for every corpus code.
- [ ] A mutation suite (change d, a witness symbol, a generator entry, the metric) is rejected in every case, each with a specific reason.
- [ ] CI reports the crate's line count, and `cargo tree -p q4-check` shows no `q4-core` or `q4-engine`.
