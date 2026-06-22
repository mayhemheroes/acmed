//! Additive harness library for the Mayhem two-branch model.
//!
//! Compiles acmed REAL duration parser (`acmed/src/duration.rs`) verbatim via `include!`, so
//! the fuzz target and the KAT oracle both exercise the upstream code: a patch to
//! `acmed/src/duration.rs` is recompiled straight into this crate. acmed is a binary crate with no
//! `lib` target, so `parse_duration` is otherwise unreachable; `include!` pulls it in without
//! touching any upstream file (the integration stays purely additive).
//!
//! The included file only non-`nom` dependency is `acme_common::error::Error` — an opaque
//! String-backed error type — satisfied by the local `acme_common` shim crate.
include!("../../../acmed/src/duration.rs");
