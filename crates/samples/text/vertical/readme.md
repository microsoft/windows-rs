# Vertical text

```console
cargo run -p text-vertical
```

Compare horizontal Japanese, vertical columns progressing left or right, and stacked Latin.
Resize to reflow, hover to inspect cluster bounds and carets, and click either outlined choice
to highlight it in every panel. Press B to show line/column bands and baselines.

Layouts, geometry, and UTF-8 hit testing use windows-text. Canvas draws the layouts, including
color emoji. The window uses windows-window without WinUI; generated Win32 bindings handle
mouse/keyboard messages and DPI changes only. Native DirectWrite comparisons live in test_text.

The font request is Yu Gothic with the `ja-jp` locale and system fallback. Appearance depends on
the installed fonts. Stacked orientation is not tate-chu-yoko; ruby is not included.
