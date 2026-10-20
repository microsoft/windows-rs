# TextBlock alignment

Run `cargo run -p reactor-text-block-alignment`.

```rust
TextBlock::new()
    .text("Centered text")
    .text_alignment(TextAlignment::Center)
```

`text_alignment` positions text within the TextBlock's available width. `horizontal_alignment`
positions the TextBlock itself. The sample compares left, centered, and right text in stretched
controls with a centered, content-sized control. Resize the window to see the difference.

TextBlock and TextBox share Reactor's `TextAlignment` enum. Canvas has a separate `TextAlignment`
for DirectWrite; qualify the type or use an import alias when using both crates.

Updating or omitting `text_alignment` keeps the native TextBlock. Omitting a previously declared
alignment clears its local value and lets WinUI supply the alignment again.
