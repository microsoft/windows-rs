# windows-reactor2

`windows-reactor2` is an experimental thin slice for a typed declarative UI pipeline:

1. public builders enforce valid property and relation shapes;
2. declarations reconcile directly into a compact retained arena;
3. one reconciler produces reusable generic property and relation batches;
4. adapters translate those mutations to a concrete UI runtime.

The prototype covers single visual content, positional and keyed visual children, hierarchical
keyed structural objects with optional visual content, container-generated data items, and queued
heterogeneous component state updates that reconcile one retained subtree directly, indexed typed
contexts, effects, references, generation-checked asynchronous delivery, and component root-type
replacement without a retained wrapper object.

The `reactor2-solitaire` sample provides a runnable application-shaped slice using a real Reactor2
window and controlled TextBox commands.

This crate is not a supported replacement for `windows-reactor`.
