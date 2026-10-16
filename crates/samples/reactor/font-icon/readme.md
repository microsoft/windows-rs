# FontIcon size and foreground

Run `cargo run -p reactor-font-icon`.

```rust
FontIcon::new()
    .glyph("\u{E80F}")
    .font_size(48.0)
    .foreground(ThemeBrush::PrimaryText)
```

Compare small red and large blue glyphs, a theme-aware foreground, and the native defaults on the
same icon. "Toggle theme" switches the window between light and dark.

Without a foreground override, the preview inherits its color from the surrounding HyperlinkButton.
"Native defaults" removes both overrides, restoring the native font size and inherited foreground.
`font_size` accepts finite positive values. `foreground` accepts a `Color` or a `ThemeBrush`.
