# mercator

`mercator` is a Rust CLI for zkSync Bridgehub operations: `scan` discovers CTM/chain topology and `inspect` performs deep, single-chain analysis.

Current status:
- `scan`: Bridgehub topology discovery
- `inspect`: deep chain inspection

## Quick start

```bash
cargo run -- scan \
  --rpc-url https://ethereum-sepolia-rpc.publicnode.com \
  --bridgehub 0x236D1c3Ff32Bd0Ca26b72Af287E895627c0478cE
```

```bash
cargo run -- inspect \
  --rpc-url https://ethereum-sepolia-rpc.publicnode.com \
  --bridgehub 0x236D1c3Ff32Bd0Ca26b72Af287E895627c0478cE \
  --chain-id 324
```

## Commands

- `scan` (topology mode)
  - input: `rpc_url`, `bridgehub`
  - output: CTMs, per-CTM chain count, and attached chain IDs
- `inspect` (chain mode)
  - input: `rpc_url`, `bridgehub`, `chain_id`
  - output: deep per-chain details (diamond, validator timelock, protocol/admin/admin-owner, warnings)

## Current extraction coverage

- CTM addresses resolved via `chainTypeManager(chainId)`
- CTM protocol versions from `protocolVersion()`
- Chain contract from `getZKChain(chainId)`
- Per-chain validator timelock/admin/protocol from CTM (`validatorTimelockPostV29`/`validatorTimelock`, `getChainAdmin`, `getProtocolVersion`)
- Admin owner from admin contract `owner()` when available

## Next slices

- Command/output polish for operator workflows
- Chain contract introspection and fallback strategy
- Privileged roles (owner/admin upgrade authorities)
- Provenance labels and diagnostics verbosity
