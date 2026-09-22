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

Generated pointer events carry `PointerEventInfo`, including element-local and window-relative
coordinates, pointer identity, mouse-button state, and pointer-capture state. Capture attempt
results are optional because events such as `PointerReleased` do not initiate capture.
Generated visual properties also include typed theme-transition declarations. Omitting a
transition property clears the WinUI dependency property, while retained equality avoids
rebuilding unchanged native transition collections.

Shared visual builders cover min/max sizing, Grid, RelativePanel, and Canvas placement,
automation metadata, and enabled state. Focus-capable controls accept an `ElementRef`;
`Runtime::focus` validates the generated capability contract before issuing the adapter command.
An `ElementRef` may appear once in a declaration tree; moving it between objects updates the
reference without depending on reconciliation order. Typed references for Grid, Image, WebView2,
and SwapChainPanel expose their native integration commands and observations. Binding generations
reject queued work and asynchronous completion after a reference moves, and dropping the runtime
completes pending requests as unavailable.
`exit_fade` removes a subtree from active identity immediately while retaining its native objects
until the WinUI fade completes. Completion shares the ordered native-occurrence stream with
observations and callbacks, so timer completion cannot overtake an earlier native event. References
and event callbacks are cleared at retirement, and component/effect ownership ends without waiting
for native cleanup.

NavigationView, ListBox, and SelectorBar expose retained selection over keyed item relations.
Native selection updates controlled item state and invokes the optional Tag or Text callback once;
application-driven selection, insertion, removal, and reorder suppress native feedback.

RichEditBox controlled text uses its native text document rather than treating `Document` as a
string dependency property. Declarations and native events normalize line endings to LF, deferred
exact feedback suppresses application writes, and read-only state is restored after native writes.
Grid row and column definitions use typed `GridLength` values with Auto, Pixel, and Star sizing,
optional minimum and maximum values, validation, retained equality, and native collection clearing.

ItemsRepeater accepts eager keyed items or a lazy `VirtualSource`. Logical items remain outside the
retained graph until WinUI realizes a container. Realization and recycling share the ordered native
event stream, preserve active keyed identity across source updates and reorder, and release row
components, effects, tasks, and references on recycle. A 10,000-item source therefore retains only
the source keys and the small set of realized visual rows.

This crate is not a supported replacement for `windows-reactor`.
