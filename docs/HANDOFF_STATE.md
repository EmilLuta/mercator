# Mercator Handoff: Current State

Last updated: 2026-02-22

## Product Scope (Current)

`mercator` is a zkSync Bridgehub-focused CLI with two operator workflows:

1. `scan` for Bridgehub topology discovery.
2. `inspect` for deep single-chain inspection.

Current commands:

```bash
mercator scan --rpc-url <RPC_URL> --bridgehub <BRIDGEHUB_ADDRESS>
mercator inspect --rpc-url <RPC_URL> --bridgehub <BRIDGEHUB_ADDRESS> --chain-id <CHAIN_ID>
```

## Implemented Features

1. Command split:
   - `scan` parses `rpc_url` + `bridgehub`.
   - `inspect` parses `rpc_url` + `bridgehub` + `chain_id`.
2. Bridgehub topology extraction:
   - `getAllZKChainChainIDs()`
   - `chainTypeManager(chainId)`
   - CTM protocol semver (`getSemverProtocolVersion()` with `protocolVersion()` fallback)
3. Per-chain deep extraction (`inspect`):
   - chain diamond proxy: `getZKChain(chainId)`
   - validator timelock ownable: `validatorTimelockPostV29()` with fallback to `validatorTimelock()` (via CTM)
   - validator timelock owner: `owner()` on timelock contract
   - chain admin ownable: `getChainAdmin(chainId)` (via CTM)
   - chain admin owner: `owner()` on admin contract
   - chain protocol semver: `getProtocolVersion(chainId)` (via CTM)
4. Partial-failure behavior:
   - unresolved fields are `unknown`
   - warnings include failed call context

## Intentional Decisions

1. Verifier extraction was intentionally removed from current output.
2. `inspect` uses operator-facing labels:
   - `BridgeHub`
   - `Chain ID`
   - `CTM`
   - `Validator Timelock Ownable`
   - `Validator Timelock Owner`
   - `Chain Diamond Proxy`
   - `Protocol`
   - `Chain Admin Ownable`
   - `Chain Admin Owner`
3. In `inspect`, `BridgeHub` is rendered inside the `Details` block (not as a standalone header line).

## Output Shape (Current)

1. `scan` output:
   - `Summary` block with:
     - `BridgeHub`
     - total chains discovered
     - total CTMs discovered
   - CTM list with:
     - CTM address
     - protocol semver
     - per-CTM chain count
     - attached chain IDs
   - warnings (if any)
2. `inspect` output:
   - `Details` block with fields listed above
   - warnings (if any)

## Known Gaps / Risks

1. `owner()` assumptions:
   - `owner()` may not exist or may be proxied differently on some deployments.
   - current behavior is warning + `unknown`.
2. Owner provenance:
   - model currently stores owner addresses but not source metadata.
3. Fallback depth:
   - no alternate owner method fallbacks yet beyond `owner()`.

## Code Pointers

- `src/main.rs`
- `src/cli.rs`
- `src/model.rs`
- `src/render.rs`
- `src/rpc.rs`
- `src/scanner/mod.rs`
- `src/scanner/bridgehub.rs`
- `tests/scan_bridgehub_ctms.rs`
- `AGENTS.md`

## Quality Gates

Run from repo root, in order:

1. `cargo fmt --all -- --check`
2. `cargo clippy --all-targets --all-features -- -D warnings`
3. `cargo test --all-targets --all-features`

## Snapshot Metrics

Rust LOC at last check:

1. With tests: `1338`
2. Without tests (`tests/` and `#[cfg(test)]` modules removed): `811`

## Session Notes

1. Keep runtime values (RPC URLs, addresses) out of committed docs/config/tests unless explicitly requested.
2. Keep `scan` concise and `inspect` detailed.
3. New-session bootstrap prompt lives in `docs/NEW_SESSION_PROMPT.md`.
