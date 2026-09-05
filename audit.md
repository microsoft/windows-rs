# Stacker Architecture and Implementation Audit

## Follow-up status

The first pre-merge cleanup pass resolved these findings:

- Removed per-message focus restoration.
- Replaced the five duplicate-orientation piece labels with the two free trominoes and randomized
  spawn rotations.
- Unified Canvas and Composition cell inset and corner-radius ratios.
- Shared one rounded-rectangle geometry and five color brushes across settled cells.
- Avoided Canvas redraws for ignored input and stale ticks.
- Deferred overlay-only brush and text-format creation until an overlay is visible.
- Added one owned, cooperatively cancellable timer task with explicit rejection handling.
- Added multi-row, scoring, level, rounded-geometry, scalar-animation, surface-stretch, drawing, and
  device-rebinding coverage.
- Updated the Composition generation and Canvas recovery documentation and registered Stacker in
  the sample index.

The feature-precedence policy, RDL interface naming and output type, declarative scene
synchronization, fade-completion ownership, accelerator API shape, and sample package placement
remain design decisions rather than changes in this cleanup pass.

## Overall verdict

The three-crate architecture is valid for the sample's teaching goal, and the implementation meets
the central resource constraint:

- Swap chains created: 0
- Steady-state application `GpuDevice`s: 1
- Steady-state `CompositionGraphicsDevice`s: 1
- Canvas composition surfaces: 1
- Compositors created by the sample: 0 - it adopts Reactor's lifted compositor
- Settled-cell visuals: 1 `ShapeVisual` per occupied cell

There is no `SwapChainPanel`, `create_swap_chain`, or `Present` path in this sample. On device loss,
a replacement `GpuDevice` is briefly created, rebound to the existing
`CompositionGraphicsDevice`, and replaces the failed device. That temporary overlap is necessary
for recovery; it does not create two steady-state graphics stacks.

However, the current implementation should not be merged unchanged. The core rendering split is
sound, but the sample and supporting crate changes still have several correctness, efficiency,
documentation, and architectural issues.

## Architecture assessment

For the stated goal - visibly demonstrating all three crates without an extra swap chain - the
current split is the best of the available options:

| Design | Assessment |
| --- | --- |
| Current: Reactor shell, Composition settled cells, Canvas active piece | Best match for the sample's explicit learning goal |
| All Composition | Simplest and most efficient game implementation, but Canvas would have no meaningful role |
| One Canvas surface for the whole board | Simpler, but loses the retained-board demonstration and compositor-owned line animations |
| Reactor Canvas `SwapChainPanel` over Composition | Worst choice here: adds a swap chain and redundant presentation infrastructure |

The division of responsibility is mostly right:

- Reactor: window, layout, buttons, state, input, and score header.
- Composition: retained settled cells and compositor-driven clear animations.
- Canvas: active piece, landing outline, and pause/game-over overlay.

Canvas drawing the overlay text is less accessible than Reactor text, but it was explicitly
required by the sketch, so it is faithful to the requested architecture. It should still avoid
creating a `TextFormat` when no overlay is visible.

The larger architectural weakness is state synchronization. `Stacker::update` mutates both `Game`
and `Scene`, while `shared_game: Rc<RefCell<Game>>` manually mirrors the model for the host
callback. Existing Reactor Composition samples use effects keyed by model state. A revision-driven
scene-sync effect would be more idiomatic and eliminate the second copy of `Game`.

## Resource and efficiency review

### What is efficient

`Scene::build` creates one `GpuDevice`, one `CompositionGraphicsDevice`, and one drawing surface.
Settled blocks are Composition vectors, not Canvas textures, so Canvas clears and redraws only:

- Three active cells.
- Three landing-outline cells.
- Occasionally one veil and text overlay.

That is a very small immediate-mode pass. Settled cells incur no per-tick redraw, and line-clear
animations continue in Composition without an application frame loop.

DPI handling is conceptually correct:

- Composition visuals stay in DIPs.
- The Canvas surface is sized in physical pixels.
- Canvas drawing receives a rasterization-scale transform.
- `SurfaceStretch::Fill` maps the physical surface into the visual's DIP bounds.

### Remaining inefficiencies

1. **Every settled cell allocates four Composition objects.** `Scene::add_visual` creates a
   geometry, sprite shape, color brush, and shape visual per cell. The geometry and five color
   brushes can be shared. Only the sprite shape and visual need to be per cell.

2. **No-op messages still redraw Canvas.** `refresh_scene` runs after stale ticks and ignored input
   while paused or over. `update` should track whether the active or overlay state actually
   changed.

3. **Text resources are allocated on every draw.** `try_draw_active` creates the overlay brushes
   and `TextFormat` even during ordinary play. Create them only inside the overlay branch, or cache
   the text format by cell size.

4. **The first host-ready draw may be wasted.** Reactor can report a zero-size host before layout.
   The code clamps that to a tiny surface, draws it, and resizes shortly afterward.

5. **Gravity occupies a thread-pool worker while sleeping.** A Dispatcher timer would be the
   natural implementation. Reactor currently lacks a public timer primitive, so the sample uses
   `spawn_background`, but that is not ideal for a long-running periodic operation.

## `windows-reactor`

### Sensible changes

Adding `Left`, `Right`, `Up`, `Down`, `Space`, `N`, and `P` to `AcceleratorKey` is directly
justified by the sample. The WinUI backend mapping is straightforward and the public API snapshot
was updated.

### Problems

1. **Per-tick focus stealing is incorrect.** `Stacker::update` calls
   `play_host.request_focus()` after every message, including every gravity tick. This prevents
   the Pause and New Game buttons from retaining keyboard focus, harms Tab navigation, causes
   repeated accessibility focus events, and adds an unnecessary imperative request each tick.

   Keep the initial `stacker-focus` effect and remove the update-time focus request.

2. **`AcceleratorKey` remains exhaustively matchable.** Adding variants is a breaking change for
   downstream exhaustive matches. Since this enum will likely grow, it should be considered for
   `#[non_exhaustive]` or redesigned to cover the full supported key set.

3. **A public Reactor timer is missing.** The sample exposes this framework gap.
   `spawn_background_with_rejection` would at least be safer than `spawn_background`: if the task
   limiter rejects a scheduled sleep, gravity currently stops permanently.

## `windows-composition`

### Faithful and warranted changes

The lifted interop declarations are ABI-correct against the Windows App SDK headers:

- `ICompositorInterop`: IID `FAB19398-6D19-4D8A-B752-8F096C396069`, with
  `CreateGraphicsDevice`.
- `ICompositionDrawingSurfaceInterop`: IID `2D6355C2-AD57-4EAE-92E4-4C3EFF65D578`, with
  the correct six-method vtable order.
- `ICompositionGraphicsDeviceInterop`: IID `4AFA8030-BC70-4B0C-B1C7-6E69F933DC83`, with
  `GetRenderingDevice` before `SetRenderingDevice`.

The additions are coherent:

- Lifted drawing surfaces are required for zero-swap-chain Canvas rendering.
- `SurfaceStretch` is required for DPI-correct surface presentation.
- Rounded rectangle geometry lets Composition own rounded settled cells.
- Scalar key frames are the right API for opacity animation.
- Rendering-device replacement is the correct device-loss mechanism.

The generated bindings are reproducible from `tool_composition`.

### Problems

1. **The feature-precedence policy has real costs.** Per the chosen rule, `reactor` wins when both
   `system` and `reactor` are enabled. This hides temporary feature complexity, but:

   - `windows-canvas[composition]` still enables `windows-composition/system`.
   - A Reactor app therefore resolves both features.
   - `windows-window` is compiled as a dead dependency.
   - A unified build containing system and Reactor consumers selects lifted APIs and causes system
     consumers to fail with missing methods rather than a targeted diagnostic.

   This is consistent with the requested policy, but it is not free or entirely local. It should
   be documented as a temporary compatibility rule.

2. **The RDL out parameter is unnecessarily raw.** `CreateGraphicsDevice` uses
   `*mut *mut void`, requiring handwritten null handling and access to
   `windows_core::imp::E_POINTER`. A typed out parameter would generate a safer wrapper, although
   the generator's current Clippy behavior may need adjustment before that is practical.

3. **The invented `IMicrosoft*` names add migration debt.** Since system and lifted bindings are
   separate modules, the lifted declarations could retain the real names (`ICompositorInterop`,
   etc.). Future metadata convergence would then mostly delete custom declarations rather than
   rename call sites.

4. **New public wrappers lack live tests.** At minimum, system-stack tests should exercise:

   - Rounded rectangle geometry setters.
   - `SurfaceStretch`.
   - Scalar key-frame insertion.
   - Drawing-surface creation.
   - Rebinding a graphics device to a second WARP device.

5. **Documentation is incomplete or stale.** `docs/crates/windows-composition.md` still contains
   system-only claims around `surface.rs` and surface interop. The code-generation section does not
   describe `interop.rdl`, the additional metadata inputs, or the stack-specific filter
   substitutions.

## `windows-canvas`

### Sensible changes

Extending `CanvasCompositionExt` to work with the selected Composition stack is natural. It
preserves the existing draw contract and avoids creating a second Canvas host or swap chain.

`GpuDevice::create_graphics_device` is a useful convenience because Canvas owns the underlying D2D
device needed by Composition.

### Questionable change

`GpuDevice::replace_graphics_device` is only a receiver-inverted wrapper around public
`CompositionGraphicsDevice::set_rendering_device`. It exists because `GpuDevice::d2d_device()` is
private, but two public spellings for the same operation may not justify the API surface.

A better long-term design would put a tested recovery owner in `windows-canvas`, rather than making
every sample implement:

```text
draw -> detect device loss -> create GpuDevice -> rebind -> retry
```

The current sample implementation is correct in outline, but recovery policy is library-level
behavior.

The Canvas documentation should describe this new recovery protocol.

## `tool_composition`

The generator extension is justified because the lifted COM interop interfaces are header-defined
and absent from the WinRT metadata consumed by the normal projection.

The generated vtables preserve unused methods as slots, which is essential and correct.

Remaining work:

- Update the top-level explanation in `composition.txt`; it currently implies that lifted
  generation only rewrites namespaces.
- Correct the statement that the Win32 interop interfaces work with both stacks. The stacks have
  distinct interface identities; the generator substitutes lifted declarations.
- Explain why the custom RDL is needed and where its GUIDs and signatures come from.
- Consider naming the RDL interfaces after their real header names rather than `IMicrosoft*`.

## `tool_yml` and workflows

The workflow split remains necessary even with Reactor precedence because a unified
Reactor-primary build cannot exercise system-only APIs.

Some comments still describe the stacks as mutually exclusive or say `windows-reactor` itself
requires `windows-composition`. The dependency actually comes through sample and integration
crates. `no_default_features.rs` was also missed and still describes the old mutual-exclusion
policy.

These changes are documentation and workflow consistency work rather than functional requirements,
but if precedence is retained they need to be completed.

## `sample_reactor_apps` placement

Placing Stacker in `sample_reactor_apps` matches the original request, but it changes that package
from pure Reactor examples to a package depending on Canvas, Composition, and Windows Core.
Running another example from the package can now compile graphics dependencies it never uses.

A separate `sample_reactor_stacker` package, or placement under `sample_reactor_composition`, would
provide cleaner dependency isolation. Against that, the existing apps package already contains
similarly large game examples, so the current location is understandable and explicitly
requested.

The sample is also missing from `crates/samples/readme.md`.

## Game logic

### Correct

- Board is 8x16.
- Collision checks reject walls, floor, and occupied cells.
- Rotation tries no kick, then exactly one cell left or right.
- Soft drop and hard drop behave sensibly.
- Full-row compaction is correct.
- Score equals lines and level advances every ten lines, exactly as requested.
- Gravity accelerates with a lower bound.
- Spawn collision ends the game.
- Pausing and new-game timer generations invalidate stale ticks.

### Fundamental specification issue

There are only two free trominoes mathematically: straight and L-shaped. Consequently:

- `Bar` is the straight orbit.
- `Corner`, `Elbow`, `Step`, and `Notch` are rotations of the same L orbit.
- `Notch` and `Elbow` are currently byte-for-byte identical.

Thus the advertised five gameplay-distinct pieces cannot exist if every piece has exactly three
connected cells and can rotate. The current implementation effectively produces 20% straight and
80% L-shaped pieces, with duplicate labels skewing the distribution.

This must be resolved in the sample definition, not hidden in code. The clean choices are:

1. Use two honest tromino kinds and randomize color and spawn rotation.
2. Keep five named spawn configurations but document that four are orientations of the corner
   tromino.
3. Change the piece-cell count or allow disconnected pieces, which would no longer be trominoes.

### Missing game tests

Tests should also cover:

- Clearing multiple rows at once.
- Score and level progression.
- Spawn collision and game over.
- Hard-drop lock and next spawn.
- Failed rotation when neither one-cell kick fits.
- Timer generation behavior across pause and new game.
- Visual compaction mapping against board compaction.

## Sample implementation issues

### Merge blockers

1. Remove per-message focus restoration.
2. Resolve or document the impossible five-tromino requirement.
3. Make active and settled cells use identical inset and radius ratios. They currently visibly
   grow on lock: Composition cells are about 90.6% of the cell width while Canvas cells are about
   82%.
4. Finish the stale documentation and sample registration.
5. Use timer rejection handling or expose and use a Reactor timer.
6. Add tests for the new Composition public API.

### Strong improvements

- Share one Composition rounded-rectangle geometry and five color brushes across all settled
  cells.
- Create or cache Canvas text resources only when needed.
- Avoid redraws for ignored messages.
- Remove `shared_game` and drive scene synchronization through a model revision effect, following
  `crates/samples/reactor/composition/examples/circles.rs`.
- Have game settlement return an explicit visual diff instead of duplicating row-compaction logic
  in `Scene::commit`.
- Use Composition batch completion to remove faded visuals; message-driven cleanup can retain
  invisible cleared visuals indefinitely after a game-ending clear.
- Rename `_device` and `_graphics`; they are actively used during recovery.

## Final recommendation

Keep the three-way retained/immediate architecture. It is faithful to the sample's purpose, uses
the three crates meaningfully, creates no swap chain, and keeps exactly one steady-state graphics
device.

Before merging, simplify and harden the implementation around that architecture. The most
important work is the tromino-definition decision, focus behavior, timer design, renderer geometry
consistency, shared Composition resources, declarative scene synchronization, new API tests, and
complete documentation. The supporting bridge is technically sound, but the current diff is not
yet as simple, idiomatic, or fully justified as it should be for a flagship sample.

## Post-audit implementation status

The follow-up cleanup resolves the remaining sample-level architecture problems:

- `Stacker` now owns `Game` and `Option<Scene>` directly. Composition host callbacks only enqueue
  `CompositionHostEvent` messages, so scene creation, layout, drawing, and mutation all happen in
  `Component::update`.
- Reactor now provides `ComponentContext::set_timeout`. It uses a one-shot dispatcher timer,
  accepts UI-thread-only message types, does not occupy a thread-pool worker, and is cancelled when
  its handle is dropped. Stacker no longer polls cancellation or carries timer generations.
- Soft drop and hard drop restart gravity after locking, so a newly spawned piece receives its
  full initial interval.
- The retained visual board is a fixed `8 x 16` array matching the game board. Settlement reports
  the locked piece and cleared rows, so the scene adds three cells directly rather than scanning a
  copied board.
- `CompositionScopedBatch::on_completed` now exposes animation completion. Stacker sends a Reactor
  message from that callback and removes faded visuals in `Component::update`; it no longer polls
  `Instant` or depends on another input event for cleanup.
- The styled `Button` that existed only to receive focus is gone. Key accelerators live on the root
  Grid, and the playfield is the Composition host Grid itself.
- Dead width, height, score, mirrored-game, focus-effect, and wrapper state has been removed.

The resulting sample keeps the intended rendering split: Reactor owns state and chrome,
Composition retains settled cells, and Canvas redraws only the active piece, landing outline, and
overlay. It still creates no swap chain and uses one steady-state D3D device shared with the
Composition graphics device. Reactor's `use_effect_guard` now owns drop-based observations without
caller-side cleanup boxing, and Composition's `Visual::host_visual` hides the raw visual conversion.
`Compositor::from_host` remains the one explicit stack boundary. Removing it would require
`windows-composition` to depend on `windows-reactor`, which is not warranted for one conversion.
