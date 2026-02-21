# Mercator Handoff: Forward Plan

Last updated: 2026-02-21

## Goal

Adopt a two-command operator workflow:
- `scan` for Bridgehub topology discovery
- `inspect` for deep single-chain analysis

## Plan (Next Slices)

1. Split command paths and output responsibilities.
   - `scan`: CTMs + chain IDs only
   - `inspect`: deep chain details

2. Add chain contract introspection around diamond.
   - Identify whether diamond exposes direct verifier/admin getters.
   - Add call-path fallback strategy with explicit warnings.

3. Add role/account clarity.
   - Distinguish chain admin vs upgrade/admin roles (if different).
   - Label role source method in model.

4. Improve output ergonomics for large chain sets.
   - Keep `scan` concise by default.
   - Keep `inspect` detailed and field-oriented.

5. Add stronger integration coverage.
   - Expand scripted-RPC tests for chain detail paths.
   - Include mixed-success scenarios (partial failures).

## Immediate Implementation Order

1. Add `inspect` CLI command (`bridgehub + chain_id`).
2. Refactor scanner into topology (`scan`) and deep chain (`inspect`) paths.
3. Update renderers so `scan` and `inspect` have distinct output shapes.
4. Add/adjust tests for parser, scanners, and rendering.

## Non-Goals (For Now)

1. Historical tracing/indexing.
2. JSON-first output.
3. Broad multi-protocol support beyond current Bridgehub/CTM workflow.

## Definition Of Done For Next Slice

1. `scan` returns CTM + chain topology only.
2. `inspect` resolves deep chain fields with warning-based partial failures.
3. CLI and renderer clearly separate the two workflows.
4. `fmt`, `clippy -D warnings`, `test` all pass.
