# Mercator Handoff: Current State

Last updated: 2026-02-21

## Product Scope (Current)

`mercator` is a zkSync Bridgehub-first CLI scanner.

Current command:

```bash
mercator scan --rpc-url <RPC_URL> --bridgehub <BRIDGEHUB_ADDRESS>
```

## Implemented Features

1. Bridgehub scan entrypoint (`scan`) with input validation.
2. Chain discovery from Bridgehub:
   - `getAllZKChainChainIDs()`
3. CTM resolution:
   - `chainTypeManager(chainId)`
4. CTM protocol version rendering as semver:
   - tries `getSemverProtocolVersion()`
   - fallback decodes packed `protocolVersion()`
5. Stage-1 chain details:
   - chain diamond proxy: `getZKChain(chainId)`
   - chain verifier: `getVerifier()` (called on chain contract)
   - chain admin: `getChainAdmin(chainId)` (via CTM)
   - per-chain protocol version semver: `getProtocolVersion(chainId)` (via CTM)

## Output Shape (Current)

1. Bridgehub address
2. CTM summary:
   - CTM address
   - CTM protocol semver
   - number of chains attached
3. Chains section:
   - `chain_id`
   - `diamond` address
   - `verifier` address
   - `ctm` address
   - chain `protocol` semver
   - chain `admin`
4. warnings section when calls fail

## Key Files

- `src/main.rs`
- `src/cli.rs`
- `src/model.rs`
- `src/render.rs`
- `src/rpc.rs`
- `src/scanner/mod.rs`
- `src/scanner/bridgehub.rs`
- `tests/scan_bridgehub_ctms.rs`
- `AGENTS.md`

## Architecture Notes

1. Transport and `eth_call` abstraction in `rpc` module.
2. Bridgehub/CTM ABI interactions are typed via `alloy_sol_types::sol!`.
3. Scanner orchestrates multi-step extraction and warning collection.
4. Renderer is terminal-first.

## Quality Gates

Run from repo root:

1. `cargo fmt --all -- --check`
2. `cargo clippy --all-targets --all-features -- -D warnings`
3. `cargo test --all-targets --all-features`

If network is restricted in environment, use `--offline` for clippy/test where possible.

## Session Notes

1. Keep runtime values (RPC URLs, addresses) out of committed docs unless explicitly approved.
2. The repository AGENTS file requires CI checks to stay mirrored with workflow changes.
3. New-session bootstrap prompt:
   - `docs/NEW_SESSION_PROMPT.md`
