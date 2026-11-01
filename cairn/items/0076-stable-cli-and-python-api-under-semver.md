---
id: 76
uid: 499bac3b-9af6-44a1-92df-25d9c2b0755b
title: Stable CLI and Python API under semver
type: chore
status: backlog
milestone: v1.0
depends_on:
- 60
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: cli
---

## Purpose

Scripts and notebooks built on v1.0 must keep working.

## Approach

Review every command, flag, exit code and Python name against the docs; remove
or rename anything we will not support; write the deprecation policy.

## Acceptance criteria

- [ ] Every public command and Python name appears in the docs, and nothing undocumented is public.
- [ ] The deprecation policy is published in the docs.
