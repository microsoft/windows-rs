# Reactor2 Solitaire

This is the Reactor2 port of the current Solitaire sample. It uses the same deal, stacking,
foundation, automatic-move, failure-highlight, and win rules. The visual board uses positioned
keyed card components, suit colors, face-down cards, foundation and tableau slots, a stock recycle
button, and a scaled green play surface.

Run both samples:

```text
cargo run -p reactor-solitaire
cargo run -p reactor2-solitaire
```

Click the stock to draw or recycle, and click a face-up card to move it automatically to a legal
foundation or tableau destination. Click `New Game` to deal again.

The component hierarchy is `Solitaire -> Board -> keyed CardView`. Moving a card between piles
preserves its component and retained object identity.

The current Reactor application host still owns AppWindow policy. Reactor2 therefore does not yet
match the original sample's fixed 800x600 client size, minimum-size constraints, dark window
theme, tall AppWindow title bar, or reposition transition. Those are framework gaps rather than
sample-side fallbacks.

`windows-reactor` currently provides the application bootstrap while the game window, component
state, declarations, reconciliation, events, and native controls use `windows-reactor2`. A
standalone Reactor2 application host remains part of the migration work.
