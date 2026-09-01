# Windows Reactor

Windows Reactor is a declarative WinUI 3 library for Rust. A `Component` owns state, receives
parent-owned input, handles typed messages, and returns a `View`. Reactor reconciles each new view
with the native UI tree and applies the required WinUI changes.

* [Guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-reactor.md)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor)

Add Reactor and the Windows App Runtime setup helper:

```toml
[dependencies]
windows-reactor = "0.100"

[build-dependencies]
windows-reactor-setup = "0.100"
```

For a framework-dependent application, stage the runtime from `build.rs`:

```rust,ignore
fn main() {
    windows_reactor_setup::as_framework_dependent();
}
```

This counter shows the Component/View model:

```rust,no_run
use windows_reactor::*;

struct Counter {
    count: u32,
}

impl Component for Counter {
    type Input = ();
    type Message = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {
        self.count += 1;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        StackPanel::new().spacing(8.0).children((
            TextBlock::new().text(format!("Count: {}", self.count)),
            Button::new()
                .on_click(context.forward())
                .content("Increment"),
        ))
    }
}

fn main() {
    App::run_component::<Counter>(()).unwrap();
}
```

`ComponentContext` supplies message senders, background work, and window requests.
`ViewContext::forward` sends an event payload that already matches the component message.
`ViewContext::message` maps a payload-free event to a fixed message. Generated control builders
expose typed properties, events, content, and child collections. `ElementRef` provides typed
imperative operations for focus and integration points that must run after publication.

Use ordinary functions returning `View` for stateless presentation. Use a `Component` when a
subtree owns state, handles messages, uses lifecycle work, or needs its own recomposition boundary.
Generated metadata value enums and slot enums are non-exhaustive, so matches must include a wildcard
arm.

Selection indices, empty numeric controls, and nullable date or time picker events use `Option<T>`
instead of native `-1`, `NaN`, or null sentinels. Omitting a property builder inherits the native
default; passing `None` sets an explicit empty value.

Large `ItemsRepeater` collections can use `VirtualSource` to construct views only for realized
items. Its key revision changes when keys or their order change; view-only updates retain the
revision and recompose only realized rows.

Constrained values are checked by their builders. For example, text weights use constants such as
`FontWeight::BOLD`, with custom values available through `FontWeight::new`.

Images can load encoded PNG and other WinUI-supported bitmap data without a file or URI:

```rust,ignore
Image::new()
    .source_data(EncodedImage::from_static(include_bytes!("logo.png")))
    .on_opened(|| println!("image ready"))
    .on_failed(|| eprintln!("image could not be decoded"))
```

`EncodedImage::from_static` borrows static data without copying it. `EncodedImage::new` owns shared
data supplied at runtime. Decoding is asynchronous; replacing or removing the image cancels its
pending load.
