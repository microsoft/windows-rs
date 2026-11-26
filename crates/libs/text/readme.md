# windows-text

Load fonts and lay out, measure, and hit-test text with DirectWrite. The crate does not draw: use
[windows-canvas](https://docs.rs/windows-canvas) to draw a `TextLayout`.

- 📦 [crates.io](https://crates.io/crates/windows-text)
- 📖 [docs.rs](https://docs.rs/windows-text)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/text)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/text)

## Measure text

```toml
[dependencies]
windows-text = "0.101"
```

```rust,no_run
use windows_text::*;

fn main() {
    let format = TextFormat::new("Segoe UI", 16.0).with_word_wrapping(WordWrapping::Wrap);
    let layout = TextLayout::new("Hello, world!", &format, 200.0, 100.0);

    let metrics = layout.metrics();
    println!("{} x {} in {} line(s)", metrics.width, metrics.height, metrics.line_count);

    let hit = layout.hit_test_point(Vector2::new(10.0, 5.0));
    let caret = layout.caret_bounds(hit.text_range.start, hit.is_trailing_hit);
    println!("caret at {}, {}", caret.left, caret.top);
}
```

Text positions are UTF-8 byte offsets into the string passed to `TextLayout::new`. Positions you
pass in must lie on character boundaries. A hit test reports the byte range of the cluster it
found, and `caret_position` picks the side of that cluster nearest the point.

## Reuse layouts

Formats, collections, faces, and fallbacks can be shared across threads; a `TextLayout` can move
between threads. Formats use the user's locale unless you call `with_locale`.

The crate requires Windows 10 version 1703 or later. Text objects do not need a graphics device,
and a `TextLayout` can be reused across frames until its text or format changes.

The [measure sample](https://github.com/microsoft/windows-rs/tree/master/crates/samples/text/measure)
prints metrics and caret positions without a window. The
[paragraphs sample](https://github.com/microsoft/windows-rs/tree/master/crates/samples/text/paragraphs)
and [vertical sample](https://github.com/microsoft/windows-rs/tree/master/crates/samples/text/vertical)
draw layouts with Canvas.
