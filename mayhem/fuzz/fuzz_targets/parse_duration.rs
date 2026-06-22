#![no_main]
use libfuzzer_sys::fuzz_target;

// Fuzz acmed duration parser (acmed/src/duration.rs :: parse_duration), the same code the old
// fork parse-duration Mayhem target intended to drive (its harness crate was never committed,
// so the original target never built). parse_duration multiplies a parsed u64 count by a unit
// multiplier and sums parts; with --debug-assertions that arithmetic can overflow-panic on hostile
// input (e.g. a huge ...w), which is exactly the defect class this target surfaces.
fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = acmed_mayhem_fuzz::parse_duration(s);
    }
});
