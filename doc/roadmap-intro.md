quaternary is an open referee for quaternary codes. Give it a code over GF(4)
or Z4, classical or quantum, and it computes or checks the minimum distance and
hands back a certificate that a separate, deliberately simple checker can
verify. It does not try to own the best search or the best table. It is the
place where anyone's result gets proved.

## The square

|        | classical                         | quantum                                    |
|--------|-----------------------------------|--------------------------------------------|
| GF(4)  | `gf4`: linear codes (codetables.de) | `qubit`: additive codes = qubit stabilizer codes (codetables.de) |
| Z4     | `z4`: Z4-linear codes (Aydin–Asamov database) | `ququart`: Z4 symplectic codes (no open tool, no table found) |

One representation serves all four: two bit planes per symbol, every weight a
popcount. Three cells have tables kept by trusted maintainers and computed with
closed tools; we reproduce them and send corrections back. The fourth has no
table; we build the first one.

## Layers, in build order

1. **checker**: a small, independent verifier. Correctness is defined here first.
2. **engine**: exact distance, fast. An optimisation the checker must agree with.
3. **ledger**: every table entry with a status (verified, open, discrepancy).
4. **frontier**: the gaps, and a path for anyone to submit a code against them.
5. **atlas**: a page per code. Last, because it is a view of the ledger.

## Out of scope

DNA storage, PAM4 and signalling (the alphabet is a coincidence there). Our own
competing bounds table. Decoders. General finite fields or a computer algebra
system. The `decisions` view (`cairn list --view decisions`) records why.

