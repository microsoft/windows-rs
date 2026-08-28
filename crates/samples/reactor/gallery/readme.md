# Reactor gallery

This sample is the `windows-reactor` port of the WinUI control gallery. It contains 65
destinations across 11 categories:

- Basic Input
- Collections
- Date and Time
- Design Guidance
- Dialogs and Flyouts
- Layout
- Media
- Menus and Toolbars
- Navigation
- Status and Info
- Text

Run it with:

```powershell
cargo run -p sample_reactor_gallery
```

The shell uses a generated `TitleBar` and `NavigationView`. The pane lists categories by default.
Select a category to open its card grid, then select a card to open the control page. Search
replaces the category links with matching control destinations. Back navigation follows the
shell's page history.

Each control destination is a component under `src/pages/`. Pages use controlled state for live
examples such as text input, sliders, selection controls, dialogs, progress indicators, and layout
changes. Page-owned state is recreated when a destination is retired and later reopened. Window
state, including the selected theme and backdrop material, belongs to the shell and persists across
page replacement.

The gallery is organized as follows:

| File | Purpose |
| --- | --- |
| `src/shell.rs` | Window visuals, title bar, navigation pane, search, and history |
| `src/registry.rs` | Category and destination metadata |
| `src/router.rs` | Leaf destination to component mapping |
| `src/controls.rs` | Shared page headers, sample cards, card grids, and assets |
| `src/pages/` | Category modules and control page components |

Bundled images in `assets/` are loaded through `Image::source_file`. Sample cards use WinUI theme
brushes so their surfaces follow the selected light, dark, or system theme. The Materials page
changes the live window backdrop between Mica, Mica Alt, Acrylic, and a solid background.

The gallery uses generated `GridView` selection feedback, `BreadcrumbBar` item-click events, and
the `Viewbox` child slot. Flyout, SplitButton, TabView, and InfoBar examples use their native event
and ownership contracts rather than substituting pointer handlers.

Recording-runtime tests mount the full gallery, replace pages through shell navigation, mount every
registered destination, drive representative controlled input, reject events from retired pages,
and verify that shell-owned theme and backdrop state survives page replacement.
