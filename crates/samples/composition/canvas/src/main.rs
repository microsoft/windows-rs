#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;
use windows_canvas::{CanvasCompositionExt, ColorF, Ellipse, GpuDevice, Vector2};
use windows_composition::*;
use windows_window::*;

struct Scene {
    surface: CompositionDrawingSurface,
    sprite: SpriteVisual,
    background: SpriteVisual,
}

impl Scene {
    fn resize(&self, width: i32, height: i32) -> Result<()> {
        let width = width.max(1);
        let height = height.max(1);
        self.background.set_size(width as f32, height as f32);

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

fn draw(surface: &CompositionDrawingSurface, side: f32) -> Result<()> {
    let center = Vector2::new(side / 2.0, side / 2.0);
    surface.draw(|session| {
        session.clear(ColorF::CORNFLOWER_BLUE);
        let white = session.create_solid_brush(ColorF::WHITE)?;
        session.fill_ellipse(&Ellipse::circle(center, side * 0.4), &white);
        let blue = session.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;
        session.fill_ellipse(&Ellipse::circle(center, side * 0.24), &blue);
        Ok(())
    })?;
    Ok(())
}

fn main() -> Result<()> {
    // Declare these first so they outlive every composition object.
    let _queue = DispatcherQueueController::create_on_current_thread()?;
    let compositor = Compositor::new()?;

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

    let background = compositor.create_sprite_visual();
    background.set_brush(&compositor.create_color_brush(CompositionColor::rgb(30, 30, 46)));
    root.children().insert_at_top(&background);

    let device = GpuDevice::new_or_warp()?;
    let graphics = device.create_graphics_device(&compositor)?;
    let surface = graphics.create_drawing_surface(1.0, 1.0)?;

    let sprite = compositor.create_sprite_visual();
    sprite.set_brush(&compositor.create_surface_brush(&surface));
    root.children().insert_at_top(&sprite);

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
