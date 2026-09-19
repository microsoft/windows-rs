# windows-reactor2

`windows-reactor2` is an experimental thin slice for a typed declarative UI pipeline:

1. public builders enforce valid property and relation shapes;
2. declarations reconcile directly into a compact retained arena;
3. one reconciler produces reusable generic property and relation batches;
4. adapters translate those mutations to a concrete UI runtime.

The prototype covers single visual content, positional and keyed visual children, hierarchical
keyed structural objects with optional visual content, and container-generated data items.

This crate is not a supported replacement for `windows-reactor`.
