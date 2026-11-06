# windows-text

Load fonts and lay out, measure, and hit-test text with DirectWrite. The crate does not draw: use
[windows-canvas](https://docs.rs/windows-canvas) to draw a `TextLayout`, or draw its
`glyph_runs` and `decorations` with your own renderer, using `FontFace::rasterize` for glyph
images. The [glyph-atlas sample](https://github.com/microsoft/windows-rs/tree/master/crates/samples/text/glyph-atlas)
draws them with Direct3D 11.

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

## Rich text

A layout can change the font, size, weight, style, decorations, and OpenType features of any
range of its text. Ranges are UTF-8 byte ranges:

```rust
use windows_text::*;

fn main() -> Result<()> {
    let format = TextFormat::new("Calibri", 16.0)?;
    let mut layout = TextLayout::new("Bold, underlined, final", &format, 300.0, 100.0)?;
    layout.set_font_weight(0..4, FontWeight::BOLD);
    layout.set_underline(6..16, true);
    layout.set_font_features(18.., &[FontFeature::new(b"liga", 0)]);

    for line in layout.line_metrics() {
        println!("{:?} is {} high", line.text_range, line.height);
    }
    let selection = layout.range_bounds(6..16);
    println!("selection covers {} rectangle(s)", selection.len());
    Ok(())
}
```

## Fonts

`FontCollection::system()` holds the installed fonts, and `FontCollection::from_data` loads
fonts from memory. Use `with_collection` to lay out with a loaded font and `with_fallback` to
choose the families tried for characters the font cannot display:

```rust
use windows_text::*;

fn main() -> Result<()> {
    let system = FontCollection::system();
    let face = system.find("Segoe UI", FontWeight::BOLD, FontStyle::Normal).unwrap();
    println!("{} units per em", face.metrics().design_units_per_em);

    let fallback = FontFallback::new(&system, &["Segoe UI Symbol"]);
    let format = TextFormat::new("Consolas", 16.0)?
        .with_style(FontStyle::Italic)
        .with_fallback(&fallback);
    let layout = TextLayout::new("\u{2603} snow", &format, 200.0, 100.0)?;
    println!("{} wide", layout.metrics().width);
    Ok(())
}
```

Formats, collections, faces, and fallbacks can be shared across threads; a `TextLayout` can move
between threads. Formats use the user's locale unless you call `with_locale`.

## Notes

The crate requires Windows 10 version 1703 or later. Text objects do not need a graphics device,
and a `TextLayout` can be reused across frames until its text or format changes. Use
`as_interface` to pass a format or layout to DirectWrite or Direct2D through other bindings, such
as the `windows` crate, and `to_interface` to query for other interfaces.

The [measure sample](https://github.com/microsoft/windows-rs/tree/master/crates/samples/text/measure)
prints metrics and caret positions without a window.
