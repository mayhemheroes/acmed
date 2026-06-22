#!/usr/bin/env bash
#
# acmed/mayhem/build.sh — build acmed's duration-parser fuzz target as a sanitized libFuzzer
# binary, replicating OSS-Fuzz's Rust path (cargo-fuzz + ASan via RUSTFLAGS).
#
# acmed is an ACME (RFC 8555) client DAEMON — a binary crate with no lib target. The fuzzed code is
# its duration parser (acmed/src/duration.rs :: parse_duration). Because the parser is not an
# exported API, our additive cargo-fuzz crate (mayhem/fuzz/) compiles that file VERBATIM via
# include!() against a tiny acme_common shim (just the opaque Error type) + the same nom 8 — so the
# REAL parser is what gets fuzzed, with no edit to any upstream file. cargo-fuzz targets it via
# `--fuzz-dir mayhem/fuzz`.
set -euo pipefail

[ -n "${SOURCE_DATE_EPOCH:-}" ] || unset SOURCE_DATE_EPOCH

: "${SRC:=/mayhem}"
: "${MAYHEM_JOBS:=$(nproc)}"
export MAYHEM_JOBS
export CARGO_BUILD_JOBS="$MAYHEM_JOBS"

cd "$SRC"

FUZZ_DIR="mayhem/fuzz"
FUZZ_TARGETS=()
for f in "$FUZZ_DIR"/fuzz_targets/*.rs; do
  FUZZ_TARGETS+=("$(basename "${f%.*}")")
done
[ "${#FUZZ_TARGETS[@]}" -gt 0 ] || { echo "ERROR: no fuzz targets under $FUZZ_DIR/fuzz_targets/" >&2; exit 1; }
TRIPLE="x86_64-unknown-linux-gnu"

# OSS-Fuzz Rust RUSTFLAGS: libFuzzer + ASan + debug info. cargo-fuzz would set ASan itself, but pin it explicitly.
# Force DWARF 3 for compliance with spec-v2 §6.2 item 10 (DWARF < 4).
export RUST_DEBUG_FLAGS="${RUST_DEBUG_FLAGS:--Cdebuginfo=2 -Cllvm-args=-dwarf-version=3}"
export RUSTFLAGS="${RUSTFLAGS:-} --cfg fuzzing -Zsanitizer=address $RUST_DEBUG_FLAGS -Cforce-frame-pointers"

echo "=== cargo fuzz build (image-default nightly, ASan via RUSTFLAGS) ==="
echo "targets: ${FUZZ_TARGETS[*]}"

for t in "${FUZZ_TARGETS[@]}"; do
  echo "--- building fuzz target: $t ---"
  cargo fuzz build --fuzz-dir "$FUZZ_DIR" -O --debug-assertions "$t"
  bin="$SRC/$FUZZ_DIR/target/$TRIPLE/release/$t"
  [ -x "$bin" ] || { echo "ERROR: fuzz binary not found at $bin" >&2; exit 1; }
  cp "$bin" "/mayhem/$t"
  echo "built /mayhem/$t"
done

echo "build.sh complete:"; ls -la /mayhem/parse_duration
