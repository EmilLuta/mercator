# Mercator Handoff: Current State

Last updated: 2026-02-21

## Product Scope (Current)

`mercator` is a zkSync Bridgehub-first CLI with split operator workflows:
- topology scan (`scan`)
- deep single-chain inspection (`inspect`)

Current commands:

```bash
mercator scan --rpc-url <RPC_URL> --bridgehub <BRIDGEHUB_ADDRESS>
mercator inspect --rpc-url <RPC_URL> --bridgehub <BRIDGEHUB_ADDRESS> --chain-id <CHAIN_ID>
```

## Implemented Features

1. Bridgehub scan entrypoint (`scan`) with input validation.
2. Chain inspection entrypoint (`inspect`) with `chain_id`.
3. Chain discovery from Bridgehub:
   - `getAllZKChainChainIDs()`
4. CTM resolution:
   - `chainTypeManager(chainId)`
5. CTM protocol version rendering as semver:
   - tries `getSemverProtocolVersion()`
   - fallback decodes packed `protocolVersion()`
6. Stage-1 chain details (used by `inspect`):
   - chain diamond proxy: `getZKChain(chainId)`
   - chain validator timelock: `validatorTimelockPostV29()` with fallback to `validatorTimelock()` (via CTM)
   - chain admin: `getChainAdmin(chainId)` (via CTM)
   - per-chain protocol version semver: `getProtocolVersion(chainId)` (via CTM)

## Output Shape (Current)

1. `scan` output:
   - Bridgehub address
   - CTM summary:
     - CTM address
     - CTM protocol semver
     - number of chains attached
2. `inspect` output:
   - Bridgehub + chain ID
   - `chain_id`
   - `diamond` address
   - `validator_timelock` address
   - `ctm` address
   - chain `protocol` semver
   - chain `admin`
3. warnings section when calls fail

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
