# Reactor2 Solitaire

This is an application-shaped Reactor2 sample rather than a visual port of the current Solitaire
sample. It uses the same component/update pattern and a real WinUI window, but presents the game as
text because Reactor2 does not yet project the button, pointer, styling, sizing, alignment,
TitleBar, Viewbox, or attached-positioning APIs used by `reactor-solitaire`.

Run both samples:

```text
cargo run -p reactor-solitaire
cargo run -p reactor2-solitaire
```

The Reactor2 sample accepts commands in its text box:

| Command | Action |
| --- | --- |
| `draw` | Draw from the stock or recycle the waste |
| `waste` | Move the waste card to a foundation or tableau pile |
| `t1` ... `t7` | Move the top card from a tableau pile |
| `new` | Deal a new game |

`windows-reactor` currently provides the application bootstrap while the game window, component
state, declarations, reconciliation, events, and native controls use `windows-reactor2`. A
standalone Reactor2 application host remains part of the migration work.
