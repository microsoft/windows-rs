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

The Reactor2 window policy sets the title, dark theme, 800x600 client size, 800x600 minimum client
size, and tall AppWindow title bar. The original sample's reposition transition remains a framework
gap rather than a sample-side fallback.

`windows-reactor2` owns the Windows App Runtime bootstrap, WinUI application, dispatcher callbacks,
explicit shutdown, game window, component state, declarations, reconciliation, events, and native
controls.
