//! Standalone composition sample — hosts a `Windows.UI.Composition` visual tree
//! in a plain Win32 window (via `windows-window`), with no WinUI / Windows App
//! SDK dependency.

#![windows_subsystem = "windows"]

use windows_composition::*;
use windows_window::*;

fn main() -> Result<()> {
    let window = Window::new("Composition Standalone")
        .size(800, 600)
        .create()?;

    // A dispatcher queue must exist on the thread before creating a compositor.
    let _queue = DispatcherQueueController::create_on_current_thread()?;
    let compositor = Compositor::new()?;

    // Host a visual tree in the window.
    let target = compositor.create_desktop_window_target(&window, false)?;
    let root = compositor.create_container_visual()?;
    target.set_root(&root)?;

    // A background that fills the window.
    let (width, height) = window.client_size();
    let background = compositor.create_sprite_visual()?;
    background.set_size(width as f32, height as f32)?;
    let background_brush = compositor.create_color_brush(Color::rgb(30, 30, 46))?;
    background.set_brush(&background_brush)?;
    root.children()?.insert_at_top(&background)?;

    // A row of colored squares on top.
    let colors = [
        Color::rgb(0, 120, 215),
        Color::rgb(216, 59, 1),
        Color::rgb(16, 137, 62),
    ];

    for (index, color) in colors.iter().enumerate() {
        let square = compositor.create_sprite_visual()?;
        square.set_size(120.0, 120.0)?;
        square.set_offset(60.0 + index as f32 * 160.0, 240.0, 0.0)?;
        let brush = compositor.create_color_brush(*color)?;
        square.set_brush(&brush)?;
        root.children()?.insert_at_top(&square)?;
    }

    run();
    Ok(())
}
