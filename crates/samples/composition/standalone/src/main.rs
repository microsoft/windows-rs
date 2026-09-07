#![windows_subsystem = "windows"]

use windows_composition::*;
use windows_window::*;

fn main() -> Result<()> {
    // Declare these first so they outlive every composition object.
    let _queue = DispatcherQueueController::create_on_current_thread()?;
    let compositor = Compositor::new()?;

    let window = Window::new("Composition Standalone")
        .size(800, 600)
        .create()?;

    let target = compositor.create_desktop_window_target(&window, false)?;
    let root = compositor.create_container_visual();
    target.set_root(&root);

    let (width, height) = window.client_size();
    let background = compositor.create_sprite_visual();
    background.set_size(width as f32, height as f32);
    let background_brush = compositor.create_color_brush(CompositionColor::rgb(30, 30, 46));
    background.set_brush(&background_brush);
    root.children().insert_at_top(&background);

    let colors = [
        CompositionColor::rgb(0, 120, 215),
        CompositionColor::rgb(216, 59, 1),
        CompositionColor::rgb(16, 137, 62),
    ];

    for (index, color) in colors.iter().enumerate() {
        let square = compositor.create_sprite_visual();
        square.set_size(120.0, 120.0);
        square.set_offset(60.0 + index as f32 * 160.0, 240.0, 0.0);
        let brush = compositor.create_color_brush(*color);
        square.set_brush(&brush);
        root.children().insert_at_top(&square);
    }

    run();
    Ok(())
}
