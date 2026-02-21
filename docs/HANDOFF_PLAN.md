# Mercator Handoff: Forward Plan

Last updated: 2026-02-21

## Goal

Continue from current Stage-1 extraction and expand chain-level core contract mapping for zkSync systems.

## Plan (Next Slices)

1. Add chain contract introspection around diamond.
   - Identify whether diamond exposes direct verifier/admin getters.
   - Add call-path fallback strategy:
     - CTM first (if authoritative)
     - chain contract/diamond second
   - Record warnings with explicit source call that failed.

2. Add role/account clarity.
   - Distinguish chain admin vs upgrade/admin roles (if different).
   - Label role source method in model.

3. Improve output ergonomics for large chain sets.
   - Optional grouping/sorting by CTM.
   - Optional flags for output detail levels.

4. Add stronger integration coverage.
   - Expand scripted-RPC tests for chain detail paths.
   - Include mixed-success scenarios (partial failures).

## Immediate Implementation Order

1. Add chain fallback ordering for verifier/admin where multiple sources exist.
2. Extend role labeling/provenance in the model.
3. Expand scripted integration coverage for mixed-success chain detail calls.
4. Add CLI output controls for detail level on large topologies.

## Non-Goals (For Now)

1. Historical tracing/indexing.
2. JSON-first output.
3. Broad multi-protocol support beyond current Bridgehub/CTM workflow.

## Definition Of Done For Next Slice

1. Verifier/admin fallback paths are explicit, deterministic, and warning-rich when unavailable.
2. Role labels include source method where available.
3. `fmt`, `clippy -D warnings`, `test` all pass.
