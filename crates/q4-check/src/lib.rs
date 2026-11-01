//! Independent checker for quaternary distance certificates.
//!
//! It depends only on `q4-cert` and does its own deliberately naive
//! arithmetic. `scripts/task lint` fails if it ever links `q4-core` or
//! `q4-engine`: a certificate checked by the code that produced it proves
//! nothing.
