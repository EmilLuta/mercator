# AGENTS.md

This file defines how agents should collaborate in this repository.
Treat it as a living implementation guide for `mercator`.

## Project Goal

Build a zkSync-focused CLI that maps system topology from a Bridgehub root address.

Primary user workflow:

1. User provides `rpc_url` and `bridgehub` address for topology discovery.
2. CLI resolves CTMs and chain IDs from Bridgehub (`scan`).
3. User runs deep per-chain inspection with `bridgehub + chain_id` (`inspect`).
4. CLI prints terminal output optimized for operators.

## Product Direction

- Short term: solve the operator workflow with targeted zkSync data extraction.
- Long term: make extraction logic extensible via modular scanners.
- Output preference: terminal-first; JSON can remain optional and non-default.

## Engineering Principles

- Build in very small, testable slices.
- Ship one capability at a time with tests before expanding scope.
- Prefer partial results with explicit warnings over hard failure.
- Keep extraction logic separate from output rendering.
- Record provenance for resolved fields when possible (`contract + function`).

## CI Pass Gate (Mandatory Before Commit)

Run these from repository root, in this exact order:

1. `cargo fmt --all -- --check`
2. `cargo clippy --all-targets --all-features -- -D warnings`
3. `cargo test --all-targets --all-features`

Rules:

- Do not commit if any command fails.
- Fix root causes instead of weakening checks.
- Keep local checks aligned with `.github/workflows/ci.yml`.
- If `.github/workflows/ci.yml` changes, update this CI Pass Gate section in the same PR.
- When CI fails, reproduce locally with the exact failing command before editing unrelated code.

## Incremental Delivery Plan

### Slice 0: Command split

Goal: support separate topology and deep inspection commands.

Acceptance:

- `mercator scan --rpc-url <URL> --bridgehub <0x...>` parses and validates.
- `mercator inspect --rpc-url <URL> --bridgehub <0x...> --chain-id <ID>` parses and validates.
- Unit tests cover required flags and argument validation for both commands.

### Slice 1: Topology scan (`scan`)

Goal: map Bridgehub -> CTMs -> chain IDs for operator overview.

Acceptance:

- Scanner resolves chain IDs via Bridgehub and CTM addresses via `chainTypeManager`.
- Output is topology-focused (no deep per-chain field dump by default).
- Integration tests are deterministic with mocked RPC responses.

### Slice 2: Chain deep dive (`inspect`)

Goal: inspect one chain in detail from `bridgehub + chain_id`.

Acceptance:

- Resolves chain contract, validator timelock, admin, and protocol version where available.
- Partial failures degrade to warnings with explicit failed call names.
- Output is field-oriented and readable for single-chain triage.

### Slice 3: Role provenance and fallback

Goal: make admin/upgrade role extraction source-aware and resilient.

Acceptance:

- Role fields include source method metadata.
- Fallback ordering is deterministic and warning-rich.
- Tests cover mixed-success scenarios.

### Slice 4: UX polish

Goal: improve readability and diagnostics for large systems.

Acceptance:

- `scan` stays concise; `inspect` stays detailed.
- `--verbose` includes call-level diagnostics and fallback paths.
- Exit codes distinguish success, partial, fatal.

## Contract Recon Workflow (when ABI is unclear)

When we do not know available Bridgehub/CTM methods:

1. Locate verified source or ABI from canonical zkSync repositories or explorers.
2. Enumerate candidate read methods for CTMs/chains/timelock/admin.
3. Implement minimal call path for one field.
4. Add tests for decode and failure behavior.
5. Repeat field by field.

Never assume function names without source confirmation when production behavior depends on them.

## Proposed Internal Architecture

- `cli`: clap parsing and user options.
- `scanner`: orchestration for one scan run.
- `extractors`: protocol-specific resolvers (`bridgehub`, `ctm`, `chains`, `roles`).
- `model`: canonical snapshot structs used by renderers.
- `render`: terminal output formatting.
- `rpc`: provider wrapper and retry/timeout behavior.

## Testing Strategy

- Unit tests:
  - Address parsing and CLI validation.
  - ABI decode helpers and mapping from raw RPC results to model structs.
  - Terminal rendering snapshots for key scenarios.
- Integration tests:
  - Mock RPC server with fixed responses for deterministic chain topology scenarios.
- Optional live tests:
  - Gated by environment variable and skipped by default in CI.

## Immediate Next Task

Implement Slice 0 and Slice 1:

1. Introduce `inspect` command with `rpc_url`, `bridgehub`, and `chain_id`.
2. Split scanner paths into topology (`scan`) and deep chain (`inspect`).
3. Keep `scan` output focused on CTMs + chain IDs.
4. Add tests before extending role provenance/fallback.
