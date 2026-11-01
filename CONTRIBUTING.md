# Contributing

## Setup

```sh
scripts/setup
scripts/agent doctor
```

## Workflow

- `main` advances only through merged pull requests. Branch protection enforces
  this on GitHub, and the `pre-push` hook refuses a push to `main` locally.
- One unit of work, one worktree, one branch, one pull request. Start with
  `scripts/agent start <type>/<slug>`; worktrees live in
  `../.worktrees/quaternary/<branch>/`. Never share a checkout between two
  pieces of work.
- Branch names: `<type>/<slug>`, type one of `feat fix chore docs perf refactor
  test`. When the work has a cairn item, start the slug with its number:
  `feat/0015-z4-bit-planes`.
- Find work with `cairn next`, take it with `cairn claim <id>`, and tick its
  acceptance criteria as they become true. Do not create TODO or PLAN files;
  create a cairn item.

## Commits

[Conventional Commits](https://www.conventionalcommits.org/): `type(scope)!:
subject`, imperative, subject at most 72 characters, no trailing period. The
body explains why; the diff shows what. Reference the cairn item with a
`Refs: #0015` trailer. The `commit-msg` hook checks all of this, and CI checks
the pull request title, which becomes the squashed commit on `main`.

## Checks

`scripts/task check` must pass before a branch becomes a pull request:
formatting, clippy with warnings denied, the crate boundary rules, the full test
suite, and the build. `scripts/agent pr` runs it for you. Never bypass a hook
with `--no-verify`.

Two rules are specific to this project:

- `q4-check` never depends on `q4-core` or `q4-engine`, and `q4-cert` depends on
  no other workspace crate. `scripts/task lint` enforces both.
- A bug fix arrives with the test that would have caught it. A wrong distance is
  fixed together with a corpus entry whose expected value comes from the
  literature, not from our own output.

## Review

Pull requests need the `required` status check to pass, the branch to be up to
date with `main`, and every conversation resolved. The project has a single
maintainer, so no approving review is required; this is deliberate and will
change when there are more maintainers. Merges are squash-only.
