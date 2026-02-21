# New Session Bootstrap Prompt

Use this prompt at the start of a new agent session for this repository:

```text
Read these first and continue from them:
- docs/HANDOFF_STATE.md
- docs/HANDOFF_PLAN.md

Before doing implementation work, ask me for the runtime inputs you need for live validation:
1) RPC URL
2) Bridgehub address

Treat these values as ephemeral and sensitive:
- do not store them in repository files
- do not commit them
- do not add them to docs/tests/config
- use them only for in-session command execution

If any live call needs those values and they are missing, ask for them first.
Then continue with the Immediate Implementation Order in HANDOFF_PLAN.md.
```
