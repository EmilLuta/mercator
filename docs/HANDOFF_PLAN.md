# Mercator Handoff: Forward Plan

Last updated: 2026-02-22

## Goal

Harden `inspect` role/ownership accuracy while preserving operator-friendly output.

## Priority Plan (Next Slices)

1. Role provenance in model/output.
   - Store source metadata for each resolved role-like field.
   - Example: `admin_owner` from `admin.owner()`, `timelock_owner` from `timelock.owner()`.

2. Owner-resolution fallback strategy.
   - Add controlled fallbacks when `owner()` is unavailable.
   - Keep explicit warnings for each failed path.

3. Admin semantics clarification.
   - Distinguish `chain admin ownable` contract vs true controlling account.
   - Document ambiguity when admin itself is a contract/proxy/multisig.

4. Test matrix expansion.
   - Mixed-success integration cases:
     - admin resolves, owner fails
     - timelock resolves, owner fails
     - both resolve to different owners
   - Verify warnings and `unknown` behavior are deterministic.

5. UX polish.
   - Keep `scan` concise.
   - Keep `inspect` detail-first, warning-rich, and terminology-stable.

## Immediate Implementation Order

1. Extend model with provenance/source metadata for:
   - `validator_timelock`
   - `validator_timelock_owner`
   - `chain_admin_ownable`
   - `chain_admin_owner`
2. Add owner fallback probes with explicit ordering and warning context.
3. Update renderer to optionally show provenance in `--verbose`.
4. Add/adjust tests before adding new extracted fields.
5. Run full CI gate sequence.

## Definition Of Done (Next Slice)

1. Owner fields include source provenance.
2. Owner resolution is deterministic with explicit fallback/warning behavior.
3. Integration tests cover mixed-success ownership scenarios.
4. `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all-targets --all-features` all pass.

## Non-Goals (For Now)

1. Historical tracing/indexing.
2. JSON-first output as default.
3. Multi-protocol support beyond current Bridgehub/CTM scope.
