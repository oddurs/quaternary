---
id: 77
uid: 0bc78ded-1c24-442d-a043-69c5e1b1dfa6
title: 'Release packaging: binaries, PyPI wheels and Homebrew'
type: chore
status: backlog
milestone: v1.0
depends_on:
- 76
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: infra
---

## Purpose

"Install is one command" on every platform named.

## Approach

A tag-triggered workflow building binaries for macOS (arm64, x86_64), Linux
(x86_64, aarch64) and Windows (x86_64); maturin wheels to PyPI; a Homebrew tap.

## Acceptance criteria

- [ ] One tag produces every artefact, and a smoke test runs on each platform.
- [ ] `pip install quaternary` and `brew install` both give a working tool on a clean machine.
