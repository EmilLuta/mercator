# AGENTS.md

This file defines how agents should collaborate in this repository.
Treat it as a living implementation guide for `mercator`.

## Project Goal

Build a zkSync-focused CLI that maps system topology from a Bridgehub root address.

Primary user workflow:

1. User provides `rpc_url` and `bridgehub` address.
2. CLI resolves connected protocol entities (starting with CTMs).
3. CLI prints rich terminal output optimized for human operators.

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

## Incremental Delivery Plan

### Slice 0: CLI foundation reset

Goal: replace placeholder commands with a single `scan` command.

Acceptance:

- `mercator scan --rpc-url <URL> --bridgehub <0x...>` parses and validates inputs.
- Command returns structured internal result object (even if mostly empty).
- Unit tests cover argument validation and required flags.

### Slice 1: Bridgehub -> CTMs

Goal: from Bridgehub, resolve attached CTMs.

Acceptance:

- Scanner returns CTM addresses for a known Bridgehub.
- Terminal renderer prints CTM section clearly.
- Unit tests validate parser and mapper logic.
- Integration test uses mocked RPC responses and is deterministic.

### Slice 2: Bridgehub -> chains

Goal: resolve chain list connected to Bridgehub.

Acceptance:

- Scanner returns chain identifiers and relevant per-chain references available at Bridgehub layer.
- Tests cover empty, single, and multi-chain responses.

### Slice 3: Per-chain core contracts

Goal: resolve diamond proxy and verifier for each chain.

Acceptance:

- Per-chain section contains diamond proxy and verifier (or explicit unresolved status).
- Tests cover mixed success where one chain fails and others succeed.

### Slice 4: Privileged roles

Goal: resolve key admin and upgrade authorities.

Acceptance:

- Reports owner/admin-style addresses with label + source.
- Unknown role methods are reported as warnings, not silent omissions.

### Slice 5: UX polish

Goal: improve operator readability and diagnostics.

Acceptance:

- Concise summary block + detailed sections.
- `--verbose` includes call-level failures and fallback paths.
- Exit codes distinguish success, partial, fatal.

## Contract Recon Workflow (when ABI is unclear)

When we do not know available Bridgehub/CTM methods:

1. Locate verified source or ABI from canonical zkSync repositories or explorers.
2. Enumerate candidate read methods for CTMs/chains/verifiers/admin.
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

Implement Slice 0 and Slice 1 only:

1. Introduce `scan` command with `rpc_url` and `bridgehub`.
2. Add scanner skeleton and CTM extractor interface.
3. Implement first Bridgehub -> CTM resolution path.
4. Add tests before moving to chains/verifiers/roles.

