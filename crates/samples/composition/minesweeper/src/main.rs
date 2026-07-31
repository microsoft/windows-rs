#![windows_subsystem = "windows"]

mod colors;
mod comp_assets;
mod comp_ui;
mod minesweeper;
mod numerics;
mod rng;
mod visual_grid;

use minesweeper::Minesweeper;
use std::cell::RefCell;
use std::rc::Rc;
use windows_composition::{Compositor, DispatcherQueueController, Result, Vector2};
use windows_window::Window;

const WM_MOUSEMOVE: u32 = 0x0200;
const WM_LBUTTONDOWN: u32 = 0x0201;
const WM_RBUTTONDOWN: u32 = 0x0204;

fn main() -> Result<()> {
    let _queue = DispatcherQueueController::create_on_current_thread()?;
    let compositor = Compositor::new()?;

    let root = compositor.create_container_visual();
    root.set_relative_size_adjustment(Vector2::new(1.0, 1.0));

    let game: Rc<RefCell<Option<Minesweeper>>> = Rc::new(RefCell::new(None));

    let window = {
        let game_message = game.clone();
        let game_resize = game.clone();
        Window::new("Minesweeper")
            .size(800, 600)
            .on_message(move |_hwnd, message, _wparam, lparam| {
                if let Some(game) = game_message.borrow_mut().as_mut() {
                    match message {
                        WM_MOUSEMOVE => game.on_pointer_moved(&point_from_lparam(lparam)).unwrap(),
                        WM_LBUTTONDOWN => game.on_pointer_pressed(false, false).unwrap(),
                        WM_RBUTTONDOWN => game.on_pointer_pressed(true, false).unwrap(),
                        _ => {}
                    }
                }
                None
            })
            .on_resize(move |width, height| {
                if let Some(game) = game_resize.borrow_mut().as_mut() {
                    game.on_parent_size_changed(&Vector2::new(width as f32, height as f32))
                        .unwrap();
                }
            })
            .create()?
    };

    let target = compositor.create_desktop_window_target(&window, false)?;
    target.set_root(&root);

    let (width, height) = window.client_size();
    *game.borrow_mut() = Some(Minesweeper::new(
        &root,
        &Vector2::new(width as f32, height as f32),
    )?);

    windows_window::run();
    Ok(())
}

fn point_from_lparam(lparam: isize) -> Vector2 {
    Vector2::new((lparam as i16) as f32, ((lparam >> 16) as i16) as f32)
}
