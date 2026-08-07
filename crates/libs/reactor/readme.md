## Windows Reactor

Windows Reactor is a declarative WinUI 3 library with render functions, state hooks, and widget
builders.

* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Reactor
  guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-reactor.md)

A minimal app defines a render function `fn(&mut RenderCx) -> Element` and passes it to
`App::render`:

```rust,no_run
use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(0_i32);

    vstack((
        text_block(format!("count = {count}")).font_size(18.0).bold(),
        button("Click").on_click(move || set_count.call(count + 1)),
    ))
    .spacing(12.0)
    .into()
}

fn main() -> windows_core::Result<()> {
    bootstrap()?;
    App::new().title("My App").render(app)
}
```

`bootstrap()` initializes the Windows App SDK runtime for a framework-dependent app. Widget
builders convert to `Element` with `.into()`. `cx.use_state` returns the current value and a handle
whose `call` schedules a rerender. `ReactorWindow` opens more top-level windows. See the [reactor
guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-reactor.md) for
components, hooks, layout, styling, and widgets.

Multi-child builders accept tuples, arrays, vectors, and child-only fragments:

```rust,ignore
vstack((
    text_block("Header"),
    fragment((text_block("Name"), text_block("Value"))),
))
```

`Fragment` cannot be converted into `Element`, so it cannot be returned as an application root or
inserted into a single-child control.

`App::on_exit` registers synchronous cleanup or instrumentation that runs once on the UI thread
immediately before Reactor exits the process after the final window closes.

WinUI lightweight styling resources use typed values:

```rust,ignore
button("Delete").resource_overrides(|resources| {
    resources
        .set("ButtonBackground", Color::rgb(178, 34, 34))
        .set("ButtonBorderThemeThickness", Thickness::uniform(0.0))
        .set("ControlCornerRadius", CornerRadius::uniform(8.0))
})
```

Replacing or clearing the builder removes only the resource keys previously owned by Reactor.

`PointerEventInfo` reports both element-local `x`/`y` coordinates and stable window-relative
`window_x`/`window_y` coordinates for drag calculations whose target moves during the gesture.
Use `.capture_pointer_on_press()` for drag handles that must keep receiving moves outside their
hit-test bounds, and clear drag state from capture-lost and canceled callbacks.

`NavigationView` can keep pane state controlled and react to its actual responsive display mode:

```rust,ignore
NavigationView::new(items, content)
    .pane_open(pane_open)
    .on_pane_open_changed(set_pane_open)
    .pane_display_mode(NavigationViewPaneDisplayMode::Auto)
    .on_display_mode_changed(set_display_mode)
```

The callbacks report settled WinUI dependency-property values rather than pane transition intent.

Lifecycle transitions run when an element enters or leaves the WinUI visual tree:

```rust,ignore
button("Animated").transition(
    Some(AnimationConfig::fade_in(Duration::from_millis(200))),
    Some(AnimationConfig::fade_out(Duration::from_millis(300))),
)
```

Reactor removes the logical element immediately while WinUI Composition keeps the departing visual
alive until its exit animation finishes. See the `exit_transition` sample.

`TabItem::with_key` supplies the identity returned by `TabView::on_close_requested`. Key changes,
including removal, update the existing native item without leaving stale close-callback identity.
See the `tab_view_item_key` sample.

Icon-taking controls share one `Icon` model:

```rust,ignore
button("Confirm").icon(Icon::path("F1 M 0,8 L 6,14 L 16,2 L 14,0 L 6,10 L 2,6 Z"));
button("Mask").icon(Icon::bitmap_icon("ms-appx:///Assets/mask.png", true));
button("Logo").icon(Icon::image("ms-appx:///Assets/logo.svg"));
```

`bitmap_icon` uses WinUI `BitmapIcon` and makes monochrome rendering explicit. `image` uses
full-color `ImageIcon` and accepts raster, SVG, or surface sources. Image icons are constrained to
the standard 20-DIP icon box so an unconstrained SVG cannot consume the control's available space.

Long text can be limited and trimmed with native WinUI behavior:

```rust,ignore
text_block("A long label")
    .max_lines(1)
    .text_trimming(TextTrimming::CharacterEllipsis)
```
