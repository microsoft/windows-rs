//! Canvas ↔ composition bridge sample — draws Direct2D content into a
//! `Windows.UI.Composition` surface and paints a visual with it, hosted in a
//! plain Win32 window (via `windows-window`), with no WinUI / Windows App SDK
//! dependency.
//!
//! The scene tracks the window size: on every resize the drawing surface is
//! reallocated and its Direct2D content is redrawn to fill the new bounds, so
//! resizing the window exercises the full bridge (surface resize + redraw + the
//! surface brush repainting the visual).

#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;
use windows_canvas::{CanvasCompositionExt, ColorF, Ellipse, GpuDevice, Vector2};
use windows_composition::*;
use windows_window::*;

/// The resizable composition graph: a full-window background and a centered
/// Direct2D-backed surface that both track the client size.
struct Scene {
    surface: CompositionDrawingSurface,
    sprite: SpriteVisual,
    background: SpriteVisual,
}

impl Scene {
    /// Lays the scene out for a `width`×`height` client area, reallocating and
    /// redrawing the surface to match.
    fn resize(&self, width: i32, height: i32) -> Result<()> {
        let width = width.max(1);
        let height = height.max(1);
        self.background.set_size(width as f32, height as f32);

        // A centered square that is 80% of the smaller client dimension.
        let side = (width.min(height) as f32 * 0.8).max(1.0);
        self.sprite.set_size(side, side);
        self.sprite.set_offset(
            (width as f32 - side) / 2.0,
            (height as f32 - side) / 2.0,
            0.0,
        );

        self.surface.resize(side as i32, side as i32)?;
        draw(&self.surface, side)?;
        Ok(())
    }
}

/// Draws the Direct2D content sized to a `side`×`side` surface.
fn draw(surface: &CompositionDrawingSurface, side: f32) -> Result<()> {
    let center = Vector2::new(side / 2.0, side / 2.0);
    surface.draw(|session| {
        session.clear(ColorF::CORNFLOWER_BLUE);
        let white = session.create_solid_brush(ColorF::WHITE).unwrap();
        session.fill_ellipse(&Ellipse::circle(center, side * 0.4), &white);
        let blue = session.create_solid_brush(ColorF::CORNFLOWER_BLUE).unwrap();
        session.fill_ellipse(&Ellipse::circle(center, side * 0.24), &blue);
    })?;
    Ok(())
}

fn main() -> Result<()> {
    // The dispatcher queue and compositor are declared first so they outlive
    // every composition object and the window — they must drop last, otherwise
    // the composition engine is torn down while visuals are still being released.
    let _queue = DispatcherQueueController::create_on_current_thread()?;
    let compositor = Compositor::new()?;

    // The scene is populated after the window and composition graph are built;
    // the resize handler reads it once it exists (the initial WM_SIZE fired
    // during window creation is a no-op while it is still empty).
    let scene: Rc<RefCell<Option<Scene>>> = Rc::new(RefCell::new(None));

    let window = {
        let scene = scene.clone();
        Window::new("Composition + Canvas")
            .size(600, 600)
            .on_resize(move |width, height| {
                if let Some(scene) = scene.borrow().as_ref() {
                    scene.resize(width, height).unwrap();
                }
            })
            .create()?
    };

    let target = compositor.create_desktop_window_target(&window, false)?;
    let root = compositor.create_container_visual();
    target.set_root(&root);

    // A dark background that fills the window.
    let background = compositor.create_sprite_visual();
    background.set_brush(&compositor.create_color_brush(Color::rgb(30, 30, 46)));
    root.children().insert_at_top(&background);

    // Bridge: a Direct2D-backed composition surface painted onto a visual.
    let device = GpuDevice::new_or_warp()?;
    let graphics = device.create_graphics_device(&compositor)?;
    let surface = graphics.create_drawing_surface(1.0, 1.0)?;

    let sprite = compositor.create_sprite_visual();
    sprite.set_brush(&compositor.create_surface_brush(&surface));
    root.children().insert_at_top(&sprite);

    // Publish the scene and lay it out for the current size, then let the resize
    // handler keep it in sync as the window is resized.
    let (width, height) = window.client_size();
    let stored = Scene {
        surface,
        sprite,
        background,
    };
    stored.resize(width, height)?;
    *scene.borrow_mut() = Some(stored);

    run();
    Ok(())
}
