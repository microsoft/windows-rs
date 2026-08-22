# Reactor next gallery slice

This sample ports a bounded basic-input slice from the incumbent Reactor gallery. It is an
adoption test rather than a control catalog: one application shell routes between Home, Text input,
and Numeric input pages while the shell retains controlled application data.

Run it with:

```powershell
cargo run -p sample_reactor_next_gallery
```

Try this sequence:

1. Open Text input and edit the name.
2. Open Numeric input and change the amount and volume.
3. Disable inputs from the navigation pane and confirm both numeric controls become disabled.
4. Return Home and confirm the edited values remain.
5. Reset the sample and revisit both pages.

Page replacement retires page components, so durable values stay in the gallery component and flow
back through controlled props. The page-header, sample-card, and page-content helpers now exercise
generated Border padding, border thickness, corner radius, and TextBlock font size. The shell also
declares a 1400 x 900 client area, system theme, and Mica backdrop; the runtime keeps the system
title bar in the matching mode. Theme brushes, custom TitleBar content, and full NavigationView
chrome remain part of the visual rollover rather than being approximated with fixed colors.

The recording test drives native click, text, numeric, slider, and toggle events through the same
component queues used by the live host. It also verifies the visual helper properties through
authoritative recorded native state.
