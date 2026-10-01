# windows-reactor

`windows-reactor` is a typed declarative UI library for WinUI 3:

1. public builders enforce valid property and relation shapes;
2. declarations reconcile directly into a compact retained arena;
3. one reconciler produces reusable generic property and relation batches;
4. adapters translate those mutations to a concrete UI runtime.

The default surface includes the application API and adapter contract. Recording, simulation, and
state-inspection helpers require the `test` feature and are intended for crate tests, acceptance
fixtures, and benchmarks.

The crate covers single visual content, positional and keyed visual children, hierarchical
keyed structural objects with optional visual content, container-generated data items, and queued
heterogeneous component state updates that reconcile one retained subtree directly, indexed typed
contexts, effects, references, generation-checked asynchronous delivery, and component root-type
replacement without a retained wrapper object. Components may be nested in ordinary visual
relations. `View::component` derives parent-local identity from the component type and complete
owning relation path; `component(key, input)` supplies explicit application identity for dynamic
movement. Transient expansion does not add component objects to the retained tree or cache rendered
declarations.
Positional children accept heterogeneous tuples. Keyed relations accept positional tuples and
arrays for static layouts or explicit keys for dynamic identity. Component contexts provide
`callback`, `forward`, and `message` adapters that queue UI-local messages without manual sender
plumbing; generated event methods accept these retained callbacks or inline closures under the same
name. Background completions retain their `Send` boundary.
`provide` scopes a typed context value to a visual subtree, and descendant components retain the
nearest value across messages, input updates, and virtualization.

`reactor-counter` provides the minimal nested-component example. `reactor-solitaire` provides a
runnable application-shaped slice with a three-level keyed component board and controlled TextBox
commands. `reactor-explorer` exercises recursive TreeView content, filtering, reorder, selection,
and asynchronous child loading. `reactor-secondary-window` opens independently scheduled
component windows through `ComponentContext::open_window_with_policy`; components may also
activate or close their current window through the same UI-local service. Reactor owns the Windows
App Runtime bootstrap, WinUI application, dispatcher callbacks and timers, Windows thread-pool
background execution, explicit shutdown, and reusable AppWindow policy used by each sample. The
bootstrap uses an installed framework package for ordinary unpackaged apps and honors packaged or
`windows-reactor-setup` self-contained runtime deployment.
`App::run_component` owns one `ComponentHost` per live window, bounded scheduling, native window
lifetime, and last-window exit. `App::run_component_with_policy` also applies initial window policy.
`App::run` uses the same host for a static `View` that does not need component state.
Components publish retained window state through `ViewContext::window_title`,
`window_visuals`, `on_window_size`, and `on_color_scheme`. One component scope owns each contract at
a time, and removing that scope clears its title, appearance, constraints, and observation
registration. `window_frame` publishes the title and returns the standard title-bar/content layout.
`WindowVisuals` supports theme, Mica or acrylic backdrop, icon, client size, and minimum or maximum
client-size constraints without exposing the native window.
`ComponentHost::mount` uses a headless service implementation for recording tests and non-WinUI
adapters, while advanced native hosts may use `App::run_with` and pass the services provided by
`AppContext` to `ComponentHost::mount_with_services`.

`App::run_with` also supports window-independent application lifetime.
`AppContext::open_component_window` opens independently scheduled component windows without making
the last window control application lifetime. `AppContext::show_menu_at` shows one keyed Menu at
physical screen coordinates through a lazily created hidden WinUI host. The host is excluded from
ordinary component-window ownership and is released when the application message loop ends.

Generated pointer moved, entered, exited, and released events carry `PointerEventInfo`, including
element-local and window-relative coordinates, pointer identity, mouse-button state, and
pointer-capture state. Capture attempt results are optional because these events do not initiate
capture.
Border can opt into capture on pointer press and pointer focus on release. Pressed payloads report
the capture attempt, while release removes the capture before scheduling pointer focus.
Preview key-down, key-up, and character-received events use `RoutedCallback`. The callback receives
owned key status and modifier data and returns WinUI's synchronous handled result.
Got-focus and lost-focus callbacks receive the focus state and whether the routed event originated
directly from the declared element.
Border drag/drop uses a retained `DragDropPolicy` to negotiate storage-item and text operations.
Drag-enter and drag-over callbacks receive the accepted data kind, while drop callbacks receive
owned text or storage-item name/path data after asynchronous extraction completes.
GridView and ListView reorder callbacks receive the current item tags in native display order.
ListView also exposes controlled selected-index state, typed selection mode, and native drag,
reorder, and drop settings while retaining its keyed data-item container pipeline.
TreeView exposes typed selection mode and item-invoked callbacks that deliver the invoked node text
without changing structural node ownership.
BreadcrumbBar item-clicked and AutoSuggestBox suggestion-chosen callbacks unwrap their inspectable
items into owned strings before entering the ordered event queue.
TabView close requests deliver the tab's retained string tag, while reorder callbacks deliver all
tab tags in native display order.
NavigationView display mode, ColorPicker color, DatePicker date, and TimePicker time callbacks use
owned typed payloads. Image and ImageIcon accept validated URIs, absolute file paths, and shared or
static encoded bytes. `.source_file(path)` converts an absolute path to a file URI, while
`.source_data(EncodedImage::new(bytes)?)` decodes owned bytes asynchronously without exposing a
native BitmapImage. RichTextBlock owns typed paragraph, run, hyperlink, and line-break data.
Generated visual properties also include typed theme-transition declarations. Omitting a
transition property clears the WinUI dependency property, while retained equality avoids
rebuilding unchanged native transition collections.

Shared pay-for-use values cover theme and solid brushes, resource overrides, button styles,
keyboard accelerators, validated URIs, path geometry strings, and implicit Border scale and
transition values. Theme brushes compile into cached WinUI styles so `ThemeResource` references
continue to react to theme changes. Resource dictionaries, styles, and accelerator collections
retain no per-object state unless the corresponding property is used.

Shared visual builders cover min/max sizing, Grid, RelativePanel, and Canvas placement,
automation metadata, and enabled state. Focus-capable controls accept an `ElementRef`;
`Runtime::focus` validates the generated capability contract before issuing the adapter command,
while `ElementRef::request_focus_result` queues the same request with generation-checked
completion.
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
CheckBox, ToggleButton, Expander, and NavigationView also expose their controlled-state
callbacks. WinUI dependency-property observers deliver native changes, while exact feedback
expectations suppress declarative setter echoes.
CalendarDatePicker exposes a typed `on_date_changed` callback with `Option<DateTime>`, including
WinUI's nullable empty-date value.

TextBox supports placeholder text, multiline return input, and wrapping through the same generated
property contract as ordinary controls while retaining its controlled text feedback behavior.

`TitleBar` owns its window attachment through the retained declaration graph. Mounting, replacing,
changing the preferred height, or removing it updates an already-open window without a separate
object lookup or window-policy attachment. A retained root may be assigned to only one live native
window.

`TooltipExt` attaches text or rich visual content to any visual declaration. The attachment owns
an internal native `ToolTip`, preserves it across target and content updates, applies
`TooltipPlacement`, and clears native ownership before either object is replaced or destroyed.

`FlyoutExt` attaches text or rich visual content to Button and SplitButton declarations. The
attachment owns one native `Flyout`, preserves it while content and placement change, and clears
the target before replacement or destruction.

`MenuExt` attaches keyed menu values to Button, DropDownButton, and MenuBarItem declarations.
Menus support enabled and disabled commands, separators, nested submenus, and ordered callbacks
that receive the selected `Key`. Duplicate keys are rejected across the complete menu.

`CommandBarCommand` provides keyed buttons, symbol buttons, disabled buttons, and separators.
`CommandBar::owned_commands` expands those values into ordinary retained AppBar elements.
`CommandBarFlyoutExt` attaches primary and secondary command groups to Button declarations while
keeping command delivery in the ordered event queue.

`ContentDialogExt` attaches one generated `ContentDialog` declaration to any visual owner.
`ContentDialog` is attachment-only rather than a panel child, keeps its generated properties,
content, and typed `on_closed` callback, and requires controlled `.is_open(bool)` state. The owner
retains the dialog across stable updates. WinUI serializes open dialogs per XamlRoot and clears the
attachment before either object is destroyed.

`TextBlock` keeps the original builder shape, `TextBlock::new().text(value)`. Strings also convert
directly to `View`, so content controls accept values such as `.content("New Game")`.

RichEditBox controlled text uses its native text document rather than treating `Document` as a
string dependency property. Declarations and native events normalize line endings to LF, deferred
exact feedback suppresses application writes, and read-only state is restored after native writes.
Grid row and column definitions use typed `GridLength` values with Auto, Pixel, and Star sizing,
optional minimum and maximum values, validation, retained equality, and native collection clearing.

ItemsRepeater accepts eager keyed items or a lazy `VirtualSource`. Logical items remain outside the
retained graph until WinUI realizes a container. Realization and recycling share the ordered native
event stream, preserve active keyed identity across source updates and reorder, and release row
components, effects, tasks, and references on recycle. A 10,000-item source therefore retains only
the source keys and the small set of realized visual rows. Pending native realization is drained
before queued component messages, and realized component references point at the row root.

ListView uses keyed `DataItem` values. A data item may keep its text-only representation or own
visual content for richer rows while retaining data identity and native container selection.

`tool-reactor` generates the control surface and verifies the frozen semantic parity baseline.
Application hosting, reactive windows, menus, images, and the published Canvas, WebView, picker,
and composition integrations are maintained as explicit product contracts.
