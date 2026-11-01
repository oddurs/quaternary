---
id: 7
uid: 1b38854e-b69e-492e-96aa-60750f1dac00
title: Scope is the quaternary square, not every four-letter alphabet
type: decision
status: done
milestone: v0.1
created: 2026-11-01
updated: 2026-11-01
closed_at: 2026-11-01
---

## Context

The first concept spanned Z4 and GF(4) codes, DNA storage, PAM4 and QPSK
signalling, quantum codes and quaternary sequences under the banner "one
representation, every algebra". Three independent critics (a coding theorist,
a DNA-storage engineer, an open-source maintainer) attacked it in November 2026.

## Options considered

1. The broad toolbelt.
2. Z4 codes only.
3. The four cells: {GF(4), Z4} × {classical, quantum}.

## Decision

Option 3. The shared structure is real only inside coding theory: the Gray map
Z4 → F2² (an isometry, but not additive, so Gray images are often nonlinear),
additive GF(4) codes = qubit stabilizer codes, and Z4 symplectic codes =
ququart stabilizer codes. One bit-plane representation genuinely serves all
four, and every weight is a popcount.

DNA is out: its hard problems are indels, dropout and coverage skew, handled by
byte-level outer codes; the ACGT alphabet is a 2-bit lookup at the end. PAM4 is
out: it is real-valued amplitudes and soft information. Quaternary sequences
and decoders are out: not on the referee path.

A real gap the DNA critic found, recorded so it is not lost: a fast,
permissively licensed read-side engine (cluster → reconstruct → decode)
benchmarked on public wet-lab reads (ENA PRJEB90546). It is a separate project.

## Revisit when

Someone shows, with data, a DNA or signalling problem whose bottleneck is
quaternary algebra.

## Acceptance criteria

- [x] The choice, its evidence and its consequences are recorded.
