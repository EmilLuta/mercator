# mercator

`mercator` is a Rust CLI for mapping on-chain systems from a known root contract.

Current status: Bridgehub CTM discovery is implemented for zkSync-style Bridgehub contracts.

## Quick start

```bash
cargo run -- scan \
  --rpc-url https://ethereum-sepolia-rpc.publicnode.com \
  --bridgehub 0x236D1c3Ff32Bd0Ca26b72Af287E895627c0478cE
```

## Current output

- Chains discovered from `getAllZKChainChainIDs()`
- CTM addresses resolved via `chainTypeManager(chainId)`
- Deduplicated CTM summary plus chain -> CTM mapping

## Next slices

- Per-chain core contracts (diamond proxy, verifier)
- Privileged roles (owner/admin upgrade authorities)
- Better diagnostics and verbosity controls
