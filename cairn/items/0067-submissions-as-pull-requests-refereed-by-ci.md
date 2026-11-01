---
id: 67
uid: 8721f21c-ae34-4fd7-936d-5f2f676601b8
title: Submissions as pull requests, refereed by CI
type: chore
status: backlog
milestone: v0.4
depends_on:
- 66
created: 2026-11-01
updated: 2026-11-01
priority: p0
effort: m
area: frontier
---

## Purpose

Anyone can submit with no server to run: a pull request is the submission and CI
is the referee (bbchallenge's model, without the infrastructure).

## Approach

A `frontier/submissions/` directory; a GitHub Action that runs `q4 submit` on
added files and posts the verdict as a comment.

## Acceptance criteria

- [ ] A test pull request with a valid code receives a verdict comment with no maintainer action.
- [ ] A malformed or wrongly claimed submission fails the check with the reason in the comment.
