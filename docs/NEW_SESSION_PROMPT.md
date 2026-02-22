# New Session Bootstrap Prompt

Use this prompt at the start of a new agent session for this repository:

```text
Read these first and continue from them:
- docs/HANDOFF_STATE.md
- docs/HANDOFF_PLAN.md
- README.md

Current product shape:
- `scan` = Bridgehub topology overview
- `inspect` = deep per-chain details

Operator label conventions to preserve in `inspect` output:
- BridgeHub
- Chain ID
- CTM
- Validator Timelock Ownable
- Validator Timelock Owner
- Chain Diamond Proxy
- Protocol
- Chain Admin Ownable
- Chain Admin Owner

Important current decision:
- verifier is intentionally not part of output/extraction right now

Before doing live validation, ask for runtime inputs:
1) RPC URL
2) Bridgehub address
3) Chain ID (only for `inspect`)

Treat runtime values as ephemeral/sensitive:
- do not store them in repository files
- do not commit them
- do not add them to docs/tests/config
- use them only for in-session command execution

If any required live input is missing, ask first.
Then continue with the Immediate Implementation Order in docs/HANDOFF_PLAN.md.
```
