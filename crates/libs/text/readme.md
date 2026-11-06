# windows-text

Lay out, measure, and hit-test text with DirectWrite. The crate does not draw: use
[windows-canvas](https://docs.rs/windows-canvas) to draw a `TextLayout`, or pass it to your own
renderer.

- 📦 [crates.io](https://crates.io/crates/windows-text)
- 📖 [docs.rs](https://docs.rs/windows-text)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/text)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/text)

## Measure text

```toml
[dependencies]
windows-text = "0.101"
```

```rust
use windows_text::*;

fn main() -> Result<()> {
    let format = TextFormat::new("Segoe UI", 16.0)?.with_word_wrapping(WordWrapping::Wrap);
    let layout = TextLayout::new("Hello, world!", &format, 200.0, 100.0)?;

    let metrics = layout.metrics();
    println!("{} x {} in {} line(s)", metrics.width, metrics.height, metrics.line_count);

    let hit = layout.hit_test_point(Vector2::new(10.0, 5.0));
    let caret = layout.caret_bounds(hit.text_position, hit.is_trailing_hit);
    println!("caret at {}, {}", caret.left, caret.top);
    Ok(())
}
```

Text positions are UTF-8 byte offsets into the string passed to `TextLayout::new`. Positions you
pass in must lie on character boundaries. A hit test reports the cluster it found, so a caret
after a multi-byte character or emoji goes at `text_position + length`.

Text objects do not need a graphics device, and a `TextLayout` can be reused across frames until
its text or format changes. Use `to_interface` to pass a format or layout to DirectWrite or
Direct2D through other bindings, such as the `windows` crate.

The [measure sample](https://github.com/microsoft/windows-rs/tree/master/crates/samples/text/measure)
prints metrics and caret positions without a window.
