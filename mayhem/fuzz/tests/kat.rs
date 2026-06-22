//! Known-answer oracle for acmed duration parser (the fuzzed code, acmed/src/duration.rs).
//! acmed ships NO tests for this parser, so we assert its documented behavior directly: unit
//! multipliers (s/m/h/d/w), multi-part sums, and rejection of malformed input. These are
//! value-exact assertions on parse_duration OUTPUT — a no-op / Ok(()) / output-altering
//! patch to the parser FAILS here (anti-reward-hacking). Run via `cargo test --test kat`.
use acmed_mayhem_fuzz::parse_duration;
use std::time::Duration;

#[test]
fn unit_multipliers() {
    assert_eq!(parse_duration("1s").unwrap(), Duration::from_secs(1));
    assert_eq!(parse_duration("1m").unwrap(), Duration::from_secs(60));
    assert_eq!(parse_duration("1h").unwrap(), Duration::from_secs(3_600));
    assert_eq!(parse_duration("1d").unwrap(), Duration::from_secs(86_400));
    assert_eq!(parse_duration("1w").unwrap(), Duration::from_secs(604_800));
}

#[test]
fn multi_part_sums() {
    assert_eq!(parse_duration("1h30m").unwrap(), Duration::from_secs(5_400));
    assert_eq!(parse_duration("2d12h").unwrap(), Duration::from_secs(216_000));
    assert_eq!(parse_duration("1w2d").unwrap(), Duration::from_secs(777_600));
    assert_eq!(parse_duration("0s").unwrap(), Duration::from_secs(0));
}

#[test]
fn rejects_malformed() {
    assert!(parse_duration("").is_err());
    assert!(parse_duration("10").is_err());
    assert!(parse_duration("5x").is_err());
    assert!(parse_duration("h").is_err());
    assert!(parse_duration("1h ").is_err());
    assert!(parse_duration("abc").is_err());
}
