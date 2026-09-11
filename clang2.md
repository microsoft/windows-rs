# windows-clang2 experiment

## Goal

Prove that a ground-up `windows-clang2` can replace `windows-clang` with a smaller, phased,
input-order-independent architecture. The experiment is unpublished and may be discarded.

## Invariants

- Extract immutable facts before making output decisions.
- Treat cursor identity as TU-local. Never infer semantic identity across TUs.
- Get final macro values and types from Clang after preprocessing.
- Resolve names before computing dependencies.
- Capture complete facts and reference edges while their originating TU is alive.
- Resolve dependencies only through the originating TU's immutable fact graph.
- Emit RDL from the finalized model without discovery or mutation.
- Do not modify `windows-clang` or committed metadata during the experiment.

## Checkpoints

| Done | Checkpoint | Continue gate |
| --- | --- | --- |
| [x] | 1. Skeleton: crate, TU loader, immutable fact model, deterministic fact dump | Two input orders produce identical facts after sorting. |
| [x] | 2. Constants slice: typedefs, enums, direct/computed macros, RDL output | Single-TU adversarial fixtures pass and RDL compiles to winmd. |
| [ ] | 3. Planner: root selection, name arbitration, TU-local dependency closure | Both input orders match; no dangling or orphan dependencies. |
| [ ] | 4. Real headers: run selected SDK headers against committed RDL | Supported declarations match, except fixture-backed corrections. |
| [ ] | 5. Breadth: records, functions, interfaces, annotations, layouts | Each added kind preserves the same pipeline and expands corpus parity. |
| [ ] | 6. Decision: run `tool-win32` and `tool-webview` through clang2 | Outputs match or every difference is reviewed and tested. |

## Stop gates

Stop the experiment if it requires cross-TU identity guesses, output mutation during extraction,
cleanup sweeps to repair ordering, unexplained RDL differences, or complexity comparable to
`windows-clang` before broad parity.

## Replacement gate

Replace `windows-clang` only if clang2 passes the generators, keeps the phased invariants, and is
materially easier to understand and change. Delete this file when the experiment is accepted or
abandoned.
