# NumberBox options

Run `cargo run -p reactor-number-box`.

```rust
NumberBox::new()
    .small_change(0.25)
    .spin_button_placement_mode(NumberBoxSpinButtonPlacementMode::Inline)
```

Compare hidden, inline, and compact spinner buttons. The compact popup appears when the control
receives focus. Up/Down and the spinner buttons use `small_change`; values remain between 0 and 20.
`small_change` accepts finite `f64` values.

The step button switches between 0.25 and 2. "Use native defaults" removes both properties from the
existing controls, restoring a step of 1 and hidden buttons. The headers identify the custom modes;
all three use hidden buttons while the overrides are removed.
