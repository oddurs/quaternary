# quaternary

[![CI](https://github.com/oddurs/quaternary/actions/workflows/ci.yml/badge.svg)](https://github.com/oddurs/quaternary/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

An open referee for quaternary codes. Give it a code over GF(4) or Z4,
classical or quantum, and it computes or checks the exact minimum distance and
hands back a certificate that a separate, deliberately simple checker can
verify.

## Why

The tables researchers cite for these codes (codetables.de for GF(4)-linear
and qubit codes, the Aydin–Asamov database for Z4 codes) are computed with
closed tools, and over Z4 there is no open way to compute an exact minimum Lee
distance at all. quaternary aims to be the place where a result about a
quaternary code gets proved: every distance it prints says how it is known
(`exact`, `lower` or `upper`), and every claim comes with a certificate you can
check without trusting the program that made it.

|        | classical          | quantum                                |
|--------|--------------------|----------------------------------------|
| GF(4)  | linear codes       | qubit stabilizer codes (additive GF(4)) |
| Z4     | Z4-linear codes    | ququart stabilizer codes (Z4 symplectic) |

## Status

Pre-release. Today the repository holds the workspace and the plan: `q4`
prints its version and usage, and nothing computes a distance yet. The first
release, v0.1, gives exact Lee, Hamming and Euclidean distance for small
Z4-linear codes with checkable certificates. [ROADMAP.md](ROADMAP.md) lists
every item from there to v1.0.

## Install

Requires Rust (the toolchain is pinned in `rust-toolchain.toml`).

```sh
cargo install --git https://github.com/oddurs/quaternary q4
q4 --version
```

## Development

Run once after cloning:

```sh
scripts/setup          # git hooks, pinned toolchain, cairn merge driver
scripts/agent doctor   # verifies tools, auth, hooks and a clean tree
```

Work happens one branch per worktree, one pull request per branch:

```sh
scripts/agent start feat/z4-bit-planes   # prints the worktree path to cd into
scripts/agent commit "feat(core): add Z4 bit-plane vectors"
scripts/agent pr                          # runs every check, pushes, opens the PR
scripts/agent done                        # after merge: removes worktree and branch
```

`scripts/task check` runs formatting, lint (warnings denied, plus the crate
boundary rules), the tests and the build, exactly as CI does. The backlog lives
in `cairn/items` and is managed with [cairn](https://github.com/oddurs/cairn);
see [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option. Unless you explicitly state
otherwise, any contribution you submit for inclusion in this work, as defined
in the Apache-2.0 license, is dual licensed as above, without any additional
terms or conditions.
