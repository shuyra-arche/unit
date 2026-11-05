# Dynamic Unit records

Validation used the installed `nightly-2026-10-09` toolchain. The historical
repository toolchain pin was left unchanged.

- Six Rust record tests passed: strict TOML, full authority policy roundtrip,
  borrowed string provenance, all truncated prefixes, version/length/trailing
  rejection, and complete u64 text/binary preservation.
- One typed event frame test passed, including truncated frames and arbitrary
  string escaping.
- The native runtime conversion compiled for `riscv64gc-unknown-none-elf`.
- System's new host tests exercise decoding the same immutable binary records,
  requested Units remaining available without startup, missing requirements,
  boot image mismatch, inactive after edges, and selected cycles.
- No QEMU or guest acceptance was run.

Strict Clippy currently also reports existing `construction/mod.rs` module
inception and the existing large `construction::Identity` enum. These baseline
diagnostics have not been hidden or reclassified as a passing repository check.
