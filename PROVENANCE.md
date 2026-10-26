# Provenance

## Source baseline

- Repository: <https://github.com/anreonyr/sqware.git>
- Commit: `a6c6824ef91ac487919d87374a71b62174d465bd`
- Branch: `codex/shuyra-unit-extraction` (PR29)
- Source subtree: `crates/unit`

## Path mapping

| Source | This repository | Treatment |
| --- | --- | --- |
| `crates/unit/src/**` (34 `.rs` files) | `src/**` | Byte-identical snapshot; public API, fields, layout, authorization types and algorithms unchanged. |
| `crates/unit/Cargo.toml` | `Cargo.toml` | Rewritten as a standalone manifest: `name = "unit"`, `version = "0.1.0"`, `edition = "2024"`, independent `[workspace]` with `resolver = "3"`; lib path and `test`/`bench`/`doctest = false` preserved; the two pinned Git dependencies are unchanged. |
| source root `Cargo.lock` | `Cargo.lock` | Minimally trimmed by actual `cargo metadata`; reachable registry versions preserved (bitflags 2.13.1, unicode-ident 1.0.24, syn 2.0.119, proc-macro2 1.0.107, quote 1.0.47, erra 0.2.0). No package version was upgraded. |
| source root `rust-toolchain.toml` | `rust-toolchain.toml` | Copied: nightly `2026-09-04`, minimal profile, `rust-src` / `rustfmt` / `clippy`, `riscv64gc-unknown-none-elf`. |
| `programs/tests/guard-deployment/tests.rs` lines 9–27 | `tests/guard_selection.rs` | Test function body and four assertions byte-identical. Added only `use unit::GuardSelectionError;` and `#[path = "../src/guard_selection.rs"] mod guard_selection;`. The source private module is not made public and the 34 source files are unchanged. |

`source/programs/tests/guard-deployment/fixtures/unit/**` and the other system
scenario fixtures are not copied; this repository contains no product `UnitFile`
values and no `PROGRAMS` / `DEPLOYMENTS` registry.

## Fixed dependency revisions

- env: `d3000253cd5d13d466e91fafd4dbb27a5ec5aaae`
  (<https://github.com/shuyra-arche/env.git>)
- system-api (S1): `9e5d63ca536a176583cf1b0054b4ecf7ddd88257`
  (<https://github.com/shuyra-arche/system.git>)
- wire, reached transitively: `0ff77dc4573fa2620d2fe995a18c01949d9c7e57`
- mold, reached transitively: `f52501f4c91b686eded8b4bcd743b4e96084b7f9`

## Authorship history

The `crates/unit` path in the source baseline was authored by
`anreonyr <anreonyr@gmail.com>`:

- `2c17c3877e3f8c67d9db5634d693fa4303d47f8c` (2026-10-26T01:13:52+08:00)
  — Separate Unit declarations, runtime composition and image building
- `e6302af2156a6fe3766d3ccc22f080a481948bf3` (2026-10-26T08:44:38+08:00)
  — Consume published env ABI and migrate its owned contract tests
- `a6c6824ef91ac487919d87374a71b62174d465bd` (2026-10-26T09:18:51+08:00)
  — Consume published System API and move its pure tests and shape contracts

## License

The source baseline contains no `LICENSE`, `COPYING` or `NOTICE` file and no
license declaration. This repository adds no license; no license is granted by
this repository.

## Scope boundary

This is a declaration library, not a complete stage3 runtime closed loop. It
carries no server, client, execution or programs component, and it cannot be
used by itself to assemble or run a product.
