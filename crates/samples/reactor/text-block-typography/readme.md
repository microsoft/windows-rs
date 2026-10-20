# TextBlock typography

Run `cargo run -p reactor-text-block-typography`.

```rust
TextBlock::new()
    .text("First line\nSecond line")
    .font_family("Consolas")
    .font_size(20.0)
    .character_spacing(150)
    .line_height(36.0)
```

Switch between **Loose** and **Tight** to compare font shapes, character spacing, and line height.
Choose **Native defaults** to clear all three properties on the same text block.

| Property | Values |
| --- | --- |
| `font_family` | A WinUI font family string, such as `"Consolas"` or `"Arial"` |
| `character_spacing` | Signed 1/1000-em units; positive loosens spacing, negative tightens it |
| `line_height` | Finite, nonnegative DIPs; zero selects automatic height from the font |

Changing these properties updates the existing text block. Omitting a property clears its local
value, restoring WinUI's default or inherited value.
