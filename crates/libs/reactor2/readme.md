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
replacement without a retained wrapper object. Components may be nested in ordinary visual
relations with parent-local keys; transient expansion does not add component objects to the
retained tree or cache rendered declarations.

`reactor2-counter` provides the minimal nested-component example. `reactor2-solitaire` provides a
runnable application-shaped slice with a three-level keyed component board and controlled TextBox
commands. `reactor2-explorer` exercises recursive TreeView content, filtering, reorder, selection,
and asynchronous child loading. Reactor2 owns the Windows App Runtime bootstrap, WinUI application,
dispatcher callbacks and timers, Windows thread-pool background execution, explicit shutdown, and
reusable AppWindow policy used by each sample. `ComponentHost::mount` uses a headless service
implementation for recording tests and non-WinUI adapters, while native hosts pass the services
provided by `AppContext` to `ComponentHost::mount_with_services`.

This crate is not a supported replacement for `windows-reactor`.
