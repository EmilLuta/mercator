# mercator

`mercator` is a Rust CLI for mapping on-chain systems from a known root contract.

Current status: scaffolded CLI with commands and publish-ready crate metadata.

## Quick start

```bash
cargo run -- init
cargo run -- probe 0x0000000000000000000000000000000000000000
```

## Planned direction

- Start from a root address (for example, zkSync L1 Bridgehub)
- Resolve linked contracts and key roles
- Output structured JSON for downstream analysis

