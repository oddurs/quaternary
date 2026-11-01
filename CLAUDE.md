# quaternary: instructions for agents

An open referee for quaternary codes: exact minimum distance over GF(4) and Z4,
classical and quantum, with certificates an independent checker verifies. Read
`doc/roadmap-intro.md` for the shape, and `cairn list --view decisions` for why
it is that shape. Do not reopen a decision without new evidence.

## Workflow

- Never commit to `main`. Start every unit of work with
  `scripts/agent start <type>/<slug>` and work in the worktree it prints.
- One item, one worktree, one branch, one pull request. Branch slugs start with
  the cairn item number: `feat/0015-z4-bit-planes`.
- `scripts/agent commit "<message>"` to commit, `scripts/agent pr` to open the
  pull request, `scripts/agent done` after it merges.
- Before reporting work finished: `scripts/task check` passes and `cairn check`
  passes. Show their output; do not claim it.

## The seam

All automation goes through `scripts/task`: `fmt fmt:check lint test build
check`. Do not call cargo from hooks, CI or scripts; add a verb here instead.

## Crate boundaries

- `q4-core`: rings, bit planes, code types, file formats.
- `q4-engine`: distance algorithms. Its answers are trusted only once checked.
- `q4-cert`: certificate data types only; depends on no workspace crate.
- `q4-check`: the independent checker; depends only on `q4-cert` and does its
  own naive arithmetic. Never import engine or core code into it, even to share
  a helper. `scripts/task lint` fails if you do.
- `q4`: the CLI.

## Correctness rules

- Every printed distance carries its status: `exact`, `lower` or `upper`.
- Expected values in tests come from the literature with a citation, never from
  running our own engine and pasting the output.
- A bug fix arrives with the test that would have caught it.

## Commits and pull requests

Conventional Commits, imperative, subject at most 72 characters, no trailing
period; the body says why; `Refs: #<item>` trailer. The pull request states the
problem, the approach, and what a reviewer should look at sceptically.

No tool attribution anywhere: no co-author trailers for tools, no "generated
with" footers, nothing in commits, pull requests, comments or docs that credits
the tool used to write it. The work is published under the maintainer's name.
The `commit-msg` hook rejects such lines.

<!-- cairn:begin -->
## Roadmap and issues

This project tracks its roadmap and issues with `cairn`. Every item is a Markdown file under `cairn/items`, described by the schema in `cairn.toml`.

**Do not create ad-hoc TODO, PLAN or NOTES files.** Create a cairn item instead, so the work appears on the board and in the generated roadmap.

### The loop

1. `cairn next` — what is ready to start. It excludes anything blocked by unfinished dependencies and puts work already in progress first.
2. `cairn claim <ID>` — take it before you start, so no one duplicates the work. `cairn claim --next` picks and claims the top-ranked unclaimed item in one step. Then read `cairn prompt <ID>`: the item with everything it rests on — the outcome it serves, what its dependencies concluded, what done means and what earlier runs learned.
3. Do the work. Record what you learn: `cairn set <ID> <field>=<value>` for fields, `cairn note <ID> "<TEXT>"` for anything that needs a sentence — why you chose something, what you tried, what to watch for.
4. `cairn tick <ID> <N>` as each acceptance criterion becomes true — `cairn show <ID> --criteria` lists them numbered. Tick what is true, not what would let you close.
5. `cairn close <ID>` when it is done, or `cairn release <ID>` to hand it back.
6. `cairn check` before you report finished. It must pass.

### Commands

```sh
cairn next --json                 # ready work, ranked
cairn claim --next                # take the next ready item
cairn search <TEXT> --json        # titles, bodies and labels
cairn list --json                 # all open items
cairn list --filter 'blocked=false,priority=p0'
cairn prompt <ID>                 # the item as a prompt, with what it rests on
cairn show <ID> --json            # one item, including its body
cairn new "<TITLE>" --type <TYPE> --milestone <MILESTONE>
cairn set <ID> status=<STATUS>    # also labels+=x, or any field below
cairn note <ID> "<TEXT>"          # append reasoning; never replaces
cairn show <ID> --criteria        # acceptance criteria, numbered
cairn tick <ID> <N>               # tick one; --all for every one
cairn close <ID>
cairn check                       # validate; run before finishing
cairn render                      # regenerate ROADMAP.md
```

Claims coordinate writers in the same item directory and are seen across the worktrees of this repository: `next` leaves out what another worktree has claimed, `claim` refuses it, and `cairn worktrees` shows what each is doing. Separate clones are not read. Agree on assignments before splitting work across clones.

Items are numbered: `0012`, and commands accept the bare number too. Write the number in `depends_on` and other id references. Each item also carries a `uid` tag; leave it alone. If a merge gives two items one number, `cairn renumber` moves the one that arrived and retargets the references that came with it.

### Schema

- **Types**: `feature`, `bug`, `spike`, `decision`, `chore`, `docs`, `outreach`, `milestone`
- **Statuses**: `backlog` (open), `planned` (open), `doing` (active), `waiting` (open), `done` (done), `dropped` (dropped)
- **`priority`**: one of p0, p1, p2, p3 — Relative to the milestone: p0 means it does not ship without this
- **`effort`**: one of s, m, l, xl — A size, not an estimate: s ≈ hours, m ≈ a day, l ≈ days; xl means split it
- **`area`**: one of core, cert, checker, engine, ledger, frontier, atlas, theory, cli, python, infra, docs — Layer of the referee: core (rings, bit planes, code types, formats), cert (certificate format), checker (the independent verifier), engine (distance algorithms), ledger (audited tables), frontier (gaps and submissions), atlas (pages and labels), theory (mathematics a spike must settle), cli, python, infra, docs
- **`cell`**: list of values — Which cells of the quaternary square this serves; empty means all of them or none in particular
- **`due`**: date, YYYY-MM-DD — When a milestone lands, YYYY-MM-DD
- **`part_of`**: names any items, by id, several allowed — A larger piece of work this belongs to
- **Milestones**: `v0.1` (due 2026-11-29), `v0.2` (due 2027-01-17), `v0.3` (due 2027-03-14), `v0.4` (due 2027-05-09), `v1.0` (due 2027-07-04), `later`
- **Saved views** (`cairn list --view NAME`): `now`, `next`, `triage`, `spikes`, `decisions`, `square`, `waiting`, `outreach`

### Rules

1. Before starting work, find or create the item and set it to an active status.
2. Use the fields above rather than inventing new ones; add new fields to `cairn.toml` first.
3. Never hand-edit the generated roadmap file — change items and run `cairn render`.
4. `cairn check` must pass before the work is considered done.

<!-- cairn:end -->
