# Source and attribution

This repository derives from `crates/unit` in
[anreonyr/sqware](https://github.com/anreonyr/sqware/tree/a6c6824ef91ac487919d87374a71b62174d465bd)
at `a6c6824ef91ac487919d87374a71b62174d465bd`. Original history remains available there; subsequent changes
are recorded in this repository's Git history.

Recorded source author identities:

- `anreonyr <anreonyr@gmail.com>`

The source snapshot did not declare a project license. This file records
attribution and grants no license; no license or SPDX identifier is inferred.
Third-party dependencies retain their own licenses.

The VNI1 envelope and borrowed archive implementation in `src/bundle` were
migrated from `env-initrd/src/ledger/bundle` in `shuyra-arche/env` at
`ee9357070996e2e559095904ff137a289a4ca156`. The Unit contract now owns its
archive format; the boot payload remains opaque and is validated by the
composition owner against Kernel's public contract. This attribution adds
no license grant. The archive cases in `tests/bundle.rs` adapt the source's
byte-frame tests, replacing its Kernel-owner test with an opaque-payload boundary
check.
