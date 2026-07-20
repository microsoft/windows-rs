//! Minesweeper — a port of robmikh's `minesweeper-rs`
//! (<https://github.com/robmikh/minesweeper-rs>) onto `windows-composition`.
//!
//! The whole board is drawn with the system composition engine
//! (`Windows.UI.Composition`) hosted in a plain Win32 window via
//! `windows-window`. There is no WinUI / Windows App SDK dependency.
//!
//! Left-click sweeps a tile, right-click cycles flag / question / empty, and the
//! board scales to fit the window. Hitting a mine plays a spiral reveal
//! animation; click again to start a new game.

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
    // A dispatcher queue must exist on the thread before creating a compositor.
    let _queue = DispatcherQueueController::create_on_current_thread()?;
    let compositor = Compositor::new()?;

    let root = compositor.create_container_visual();
    root.set_relative_size_adjustment(Vector2::new(1.0, 1.0));

    // The game is built after the window so the board can be sized to the
    // window's client area. It is shared with the input closures, which observe
    // messages and forward pointer input to the game.
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
                // Observe only; let default window processing run.
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

/// Extracts a client-area point from a mouse message's `lParam` (the signed low
/// and high 16-bit words).
fn point_from_lparam(lparam: isize) -> Vector2 {
    Vector2::new((lparam as i16) as f32, ((lparam >> 16) as i16) as f32)
}
